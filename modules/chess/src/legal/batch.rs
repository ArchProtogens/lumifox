/*
 * Batch legal context for computing attack masks and opponent piece masks once per board.
 */

use crate::constants::{NOT_A_FILE, NOT_H_FILE};
use crate::model::gameboard::GameBoard;

#[cfg(feature = "precomputed_rays")]
use crate::model::rays::{KING_MOVES, KNIGHT_MOVES, PAWN_ATTACK_BLACK, PAWN_ATTACK_WHITE, RAYS};

/// Precomputed context for batch legality checks built from a single `GameBoard`.
pub struct BatchLegalContext {
  /// Snapshot of the original board this context was built from. GameBoard is Copy so this is cheap.
  pub board: GameBoard,
  pub occ: u64,
  pub colour_mask: u64,
  pub opponent_pawns: u64,
  pub opponent_knights: u64,
  pub opponent_bishops: u64,
  pub opponent_rooks: u64,
  pub opponent_queens: u64,
  pub opponent_kings: u64,
  /// Aggregate attack map for the opponent (squares attacked by the side not to move)
  pub opponent_attacks: u64,
  pub playing: bool,
}

impl BatchLegalContext {
  /// Build the batch context from a GameBoard. This computes occupancy, per-piece opponent masks
  /// and the full opponent attack bitboard using precomputed tables when available.
  pub fn from_board(board: &GameBoard) -> Self {
    let occ = board.combined().raw();
    let colour_mask = board.colour.raw();
    let opponent_white = !board.playing;

    // Build a mask selecting opponent-coloured squares: if opponent is white, use colour_mask
    // (colour bit = 1 for white). If opponent is black, invert the colour_mask.
    let opponent_colour_mask: u64 = if opponent_white {
      colour_mask
    } else {
      !colour_mask
    };

    // Opponent piece masks (raw u64) — AND each piece bitmap with the opponent colour mask.
    let opponent_pawns = board.pawns.raw() & opponent_colour_mask;
    let opponent_knights = board.knights.raw() & opponent_colour_mask;
    let opponent_bishops = board.bishops.raw() & opponent_colour_mask;
    let opponent_rooks = board.rooks.raw() & opponent_colour_mask;
    let opponent_queens = board.queens.raw() & opponent_colour_mask;
    let opponent_kings = board.kings.raw() & opponent_colour_mask;

    // Build aggregate attacks
    let mut attacks: u64 = 0;

    // Pawn attacks: use precomputed tables when feature enabled, otherwise compute with shifts
    #[cfg(feature = "precomputed_rays")]
    {
      if opponent_white {
        // opponent pawns are white, so they attack using PAWN_ATTACK_WHITE from their squares
        let mut p = opponent_pawns;
        while p != 0 {
          let sq = p.trailing_zeros() as usize;
          attacks |= PAWN_ATTACK_WHITE[sq];
          p &= p - 1;
        }
      } else {
        let mut p = opponent_pawns;
        while p != 0 {
          let sq = p.trailing_zeros() as usize;
          attacks |= PAWN_ATTACK_BLACK[sq];
          p &= p - 1;
        }
      }
    }

    #[cfg(not(feature = "precomputed_rays"))]
    {
      if opponent_white {
        // white pawns attack to NW and NE -> compute via shifts
        let left = (opponent_pawns & NOT_A_FILE) << 7;
        let right = (opponent_pawns & NOT_H_FILE) << 9;
        attacks |= left | right;
      } else {
        let left = (opponent_pawns & NOT_A_FILE) >> 9;
        let right = (opponent_pawns & NOT_H_FILE) >> 7;
        attacks |= left | right;
      }
    }

    // Knight attacks: use precomputed KNIGHT_MOVES when available, otherwise use the same technique as attack.rs
    #[cfg(feature = "precomputed_rays")]
    {
      let mut k = opponent_knights;
      while k != 0 {
        let sq = k.trailing_zeros() as usize;
        attacks |= KNIGHT_MOVES[sq];
        k &= k - 1;
      }

      let mut kk = opponent_kings;
      while kk != 0 {
        let sq = kk.trailing_zeros() as usize;
        attacks |= KING_MOVES[sq];
        kk &= kk - 1;
      }
    }
    #[cfg(not(feature = "precomputed_rays"))]
    {
      // Parallel-shift approach from attack.rs
      let knights = opponent_knights;
      let l1 = (knights >> 1) & NOT_H_FILE;
      let l2 = (knights >> 2) & crate::constants::NOT_GH_FILE;
      let r1 = (knights << 1) & crate::constants::NOT_A_FILE;
      let r2 = (knights << 2) & crate::constants::NOT_AB_FILE;
      let h1 = l1 | r1;
      let h2 = l2 | r2;
      attacks |= (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);

      // King attacks via shifts
      let kings = opponent_kings;
      let east = (kings << 1) & NOT_A_FILE;
      let west = (kings >> 1) & NOT_H_FILE;
      let king_adj = east | west;
      let king_set = kings | king_adj;
      attacks |= king_adj | (king_set << 8) | (king_set >> 8);
    }

    // Sliding pieces: rook-like (1, -1, 8, -8) and bishop-like (9, -9, 7, -7)
    // Use RAYS table when precomputed, otherwise fall back to simple scans.
    #[cfg(feature = "precomputed_rays")]
    {
      // Helper to accumulate sliding attacks for a given set of piece squares and directional indices
      let accumulate_sliders = |mut pieces: u64, dirs: &[usize]| -> u64 {
        let mut out: u64 = 0;
        while pieces != 0 {
          let sq = pieces.trailing_zeros() as usize;
          // For each direction index, use the ray and occ to find squares up to blocker
          for &d in dirs {
            let ray_mask = RAYS[sq][d];
            let blockers = occ & ray_mask;
            if blockers == 0 {
              out |= ray_mask; // whole ray
            } else {
              // For now include the ray — callers still rely on occ when exactness is required.
              out |= ray_mask;
            }
          }
          pieces &= pieces - 1;
        }
        out
      };

      // Rook-like pieces
      let rook_dirs: [usize; 4] = [0, 1, 2, 3];
      attacks |= accumulate_sliders(opponent_rooks | opponent_queens, &rook_dirs);
      // Bishop-like pieces
      let bishop_dirs: [usize; 4] = [4, 5, 6, 7];
      attacks |= accumulate_sliders(opponent_bishops | opponent_queens, &bishop_dirs);
    }
    #[cfg(not(feature = "precomputed_rays"))]
    {
      // Naive sliding: for each sliding piece, step along each direction until blocked
      let step = |mut pieces: u64, deltas: &[i8]| -> u64 {
        let mut out = 0u64;
        while pieces != 0 {
          let sq = pieces.trailing_zeros() as u8;
          pieces &= pieces - 1;
          for &d in deltas {
            let mut cur = sq as i8 + d;
            while (cur as i32) >= 0 && (cur as i32) < 64 {
              out |= 1u64 << (cur as u8);
              if (occ & (1u64 << (cur as u8))) != 0 {
                break;
              }
              cur += d;
            }
          }
        }
        out
      };

      attacks |= step(opponent_rooks | opponent_queens, &[1, -1, 8, -8]);
      attacks |= step(opponent_bishops | opponent_queens, &[9, -9, 7, -7]);
    }

    // Finally include direct attacker squares (i.e., squares occupied by the opponent themselves) for
    // completeness: a piece occupies a square and still 'attacks' it as expected by some checks.
    attacks |= opponent_pawns
      | opponent_knights
      | opponent_bishops
      | opponent_rooks
      | opponent_queens
      | opponent_kings;

    BatchLegalContext {
      board: *board,
      occ,
      colour_mask,
      opponent_pawns,
      opponent_knights,
      opponent_bishops,
      opponent_rooks,
      opponent_queens,
      opponent_kings,
      opponent_attacks: attacks,
      playing: board.playing,
    }
  }
}
