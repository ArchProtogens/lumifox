/*
 * A high-performance chess library licensed under the LGPLv3.
 * Copyright (C) 2025 Clifton Toaster Reid
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library. If not, see <https://opensource.org/license/lgpl-3-0>.
 */

use crate::constants::{FILE_A, FILE_H, NOT_A_FILE, NOT_AB_FILE, NOT_GH_FILE, NOT_H_FILE};
use crate::model::bitboard::BitBoard;
use crate::model::gameboard::GameBoard;
use crate::model::rays::{DIR_OFFSETS, RAYS};

fn is_square_attacked_pawn(board: &GameBoard, square: u8) -> bool {
  if square >= 64 {
    return false;
  }

  let opponent_white = !board.playing;
  let desired_for_opponent = !opponent_white;
  let opponent_pawns = board.pawns & board.combined_coloured(desired_for_opponent);

  let attacks = if opponent_white {
    let left_attacks = (opponent_pawns & BitBoard::new(!FILE_A)) << 7;
    let right_attacks = (opponent_pawns & BitBoard::new(!FILE_H)) << 9;
    left_attacks | right_attacks
  } else {
    let left_attacks = (opponent_pawns & BitBoard::new(!FILE_A)) >> 9;
    let right_attacks = (opponent_pawns & BitBoard::new(!FILE_H)) >> 7;
    left_attacks | right_attacks
  };

  attacks.get_bit_unchecked(square)
}

fn is_square_attacked_knight(board: &GameBoard, square: u8) -> bool {
  let opponent_white = !board.playing;
  let desired = !opponent_white;
  let opponent_knights = board.knights & board.combined_coloured(desired);
  let knights = opponent_knights.raw();

  let l1 = (knights >> 1) & NOT_H_FILE;
  let l2 = (knights >> 2) & NOT_GH_FILE;
  let r1 = (knights << 1) & NOT_A_FILE;
  let r2 = (knights << 2) & NOT_AB_FILE;
  let h1 = l1 | r1;
  let h2 = l2 | r2;
  let attacks = (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);

  (attacks & (1u64 << square)) != 0
}

fn is_square_attacked_king(board: &GameBoard, square: u8) -> bool {
  let opponent_white = !board.playing;
  let desired = !opponent_white;
  let opponent_kings = board.kings & board.combined_coloured(desired);
  let kings = opponent_kings.raw();

  let east = (kings << 1) & NOT_A_FILE;
  let west = (kings >> 1) & NOT_H_FILE;
  let attacks = east | west;
  let king_set = kings | attacks;
  let north = king_set << 8;
  let south = king_set >> 8;
  let all_attacks = attacks | north | south;

  (all_attacks & (1u64 << square)) != 0
}

fn is_square_attacked_sliding(
  board: &GameBoard,
  square: u8,
  dirs: &[i8],
  piece_bb: BitBoard,
  opponent_white: bool,
) -> bool {
  // Cache frequently used bitboard raw values to avoid method call overhead
  let occ: u64 = board.combined().into();
  let colour_mask: u64 = board.colour.into();
  let piece_mask: u64 = piece_bb.into();

  // Map requested directions (i8 offsets) to the RAYS table indices.
  // RAYS ordering matches DIR_OFFSETS constant.
  for &dir in dirs {
    // find index of dir in DIR_OFFSETS
    let mut idx: usize = 0;
    while idx < DIR_OFFSETS.len() {
      if DIR_OFFSETS[idx] == dir {
        break;
      }
      idx += 1;
    }
    if idx >= DIR_OFFSETS.len() {
      continue; // unknown direction
    }

    let ray_mask = RAYS[square as usize][idx];
    let blockers = occ & ray_mask;
    if blockers == 0 {
      continue;
    }

    // Determine nearest blocker depending on direction sign
    let blocker_sq: u8 = if DIR_OFFSETS[idx] > 0 {
      blockers.trailing_zeros() as u8
    } else {
      (63 - blockers.leading_zeros()) as u8
    };

    let bit = 1u64 << blocker_sq;
    let square_is_opponent = ((colour_mask & bit) != 0) == opponent_white;
    if square_is_opponent && (piece_mask & bit) != 0 {
      return true;
    }
  }

  false
}

fn is_square_attacked_rook(board: &GameBoard, square: u8) -> bool {
  let opponent_white = !board.playing;
  let desired = !opponent_white;
  let opponent_rooks = board.rooks & board.combined_coloured(desired);
  let opponent_queens = board.queens & board.combined_coloured(desired);
  let piece_bb = opponent_rooks | opponent_queens;
  let dirs: [i8; 4] = [1, -1, 8, -8];
  is_square_attacked_sliding(board, square, &dirs, piece_bb, opponent_white)
}

fn is_square_attacked_bishop(board: &GameBoard, square: u8) -> bool {
  let opponent_white = !board.playing;
  let desired = !opponent_white;
  let opponent_bishops = board.bishops & board.combined_coloured(desired);
  let opponent_queens = board.queens & board.combined_coloured(desired);
  let piece_bb = opponent_bishops | opponent_queens;
  let dirs: [i8; 4] = [9, -9, 7, -7];
  is_square_attacked_sliding(board, square, &dirs, piece_bb, opponent_white)
}

pub fn is_square_attacked(board: &GameBoard, square: u8) -> bool {
  is_square_attacked_pawn(board, square)
    || is_square_attacked_knight(board, square)
    || is_square_attacked_king(board, square)
    || is_square_attacked_rook(board, square)
    || is_square_attacked_bishop(board, square)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::constants::*;
  use crate::model::gamedata::GameData;

  fn get_board(fen: &str) -> GameBoard {
    GameData::from_fen(fen).unwrap().board
  }

  // Pawn Tests
  #[test]
  fn test_pawn_attacks() {
    // It's white's turn, so we check for attacks by black pieces.
    // Black pawn on e4 attacks d3 and f3.
    let board = get_board("8/8/8/8/4p3/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, D3));
    assert!(is_square_attacked(&board, F3));
    assert!(!is_square_attacked(&board, E4));
    assert!(!is_square_attacked(&board, E3));

    // It's black's turn, so we check for attacks by white pieces.
    // White pawn on e5 attacks d6 and f6.
    let board = get_board("8/8/8/4P3/8/8/8/8 b - - 0 1");
    assert!(is_square_attacked(&board, D6));
    assert!(is_square_attacked(&board, F6));
    assert!(!is_square_attacked(&board, E5));
    assert!(!is_square_attacked(&board, E6));
  }

  #[test]
  fn test_pawn_edge_attacks() {
    // Black pawn on a5 only attacks b4 (not off-board to the left)
    let board = get_board("8/8/8/p7/8/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, B4));
    assert!(!is_square_attacked(&board, A4));
    assert!(!is_square_attacked(&board, A5));

    // White pawn on h4 only attacks g5
    let board = get_board("8/8/8/8/7P/8/8/8 b - - 0 1");
    assert!(is_square_attacked(&board, G5));
    assert!(!is_square_attacked(&board, H5));
    assert!(!is_square_attacked(&board, H4));
  }

  #[test]
  fn test_pawn_backward_not_attack() {
    // Black pawn on e4 should not attack forward squares e3/e5 nor backward like d5/f5 when side to move is white
    let board = get_board("8/8/8/8/4p3/8/8/8 w - - 0 1");
    assert!(!is_square_attacked(&board, E5));
    assert!(!is_square_attacked(&board, D5));
    assert!(!is_square_attacked(&board, F5));

    // White pawn on e5 should not attack backward squares d4/f4 when side to move is black
    let board = get_board("8/8/8/4P3/8/8/8/8 b - - 0 1");
    assert!(!is_square_attacked(&board, D4));
    assert!(!is_square_attacked(&board, F4));
  }

  #[test]
  fn test_pawn_attacks_occupied_target() {
    // Even if the attacked square is occupied (by any piece), it's still considered attacked.
    // Black pawn on e4, black knight on d3. White to move -> checking black attacks.
    let board = get_board("8/8/8/8/4p3/3n4/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, D3));
    // White pawn on e5, white knight on f6. Black to move -> checking white attacks.
    let board = get_board("8/8/5N2/4P3/8/8/8/8 b - - 0 1");
    assert!(is_square_attacked(&board, F6));
  }

  // Knight Tests
  #[test]
  fn test_knight_attacks() {
    // Black knight on d4 attacks 8 squares around it.
    let board = get_board("8/8/8/8/3n4/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, C2));
    assert!(is_square_attacked(&board, E2));
    assert!(is_square_attacked(&board, B3));
    assert!(is_square_attacked(&board, F3));
    assert!(is_square_attacked(&board, B5));
    assert!(is_square_attacked(&board, F5));
    assert!(is_square_attacked(&board, C6));
    assert!(is_square_attacked(&board, E6));
    assert!(!is_square_attacked(&board, D4));
  }

  #[test]
  fn test_knight_edge_attacks() {
    // Black knight on a1 only has 2 legal attack squares b3 & c2
    let board = get_board("8/8/8/8/8/8/8/n7 w - - 0 1");
    assert!(is_square_attacked(&board, B3));
    assert!(is_square_attacked(&board, C2));
    // Squares that would be off-board are naturally not attacked
    assert!(!is_square_attacked(&board, A1));
    assert!(!is_square_attacked(&board, A2));
  }

  #[test]
  fn test_knight_blocking_irrelevant() {
    // Pieces between a knight and its destination do not matter.
    // Black knight on d4, white pawns placed where "blocking" would be for sliding pieces.
    let board = get_board("8/8/3P1P2/8/3n4/3P1P2/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, C2));
    assert!(is_square_attacked(&board, E2));
    assert!(is_square_attacked(&board, B5));
    assert!(is_square_attacked(&board, F5));
  }

  #[test]
  fn test_knight_only_opponent_counted() {
    // Side to move white -> counting black attacks. Include both black & white knights; only black's should count.
    let board = get_board("8/8/8/3N4/3n4/8/8/8 w - - 0 1");
    // Black knight d4 attacks C2; white knight d5 would also attack C3 but shouldn't influence C2
    assert!(is_square_attacked(&board, C2));
    // Square C3 attacked only by white knight -> should be false when evaluating black attacks
    assert!(!is_square_attacked(&board, C3));
  }

  // Bishop Tests
  #[test]
  fn test_bishop_attacks() {
    // Black bishop on d4 attacks along diagonals.
    let board = get_board("8/8/8/8/3b4/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, A1));
    assert!(is_square_attacked(&board, B2));
    assert!(is_square_attacked(&board, C3));
    assert!(is_square_attacked(&board, E5));
    assert!(is_square_attacked(&board, F6));
    assert!(is_square_attacked(&board, G7));
    assert!(is_square_attacked(&board, H8));
    assert!(!is_square_attacked(&board, D4));
    assert!(!is_square_attacked(&board, D5));
  }

  #[test]
  fn test_bishop_blocked() {
    // Black bishop on d4, but attacks are blocked by white pawns.
    let board = get_board("8/8/8/2P1P3/3b4/2P1P3/8/8 w - - 0 1");
    assert!(!is_square_attacked(&board, A1));
    assert!(!is_square_attacked(&board, H8));
    assert!(is_square_attacked(&board, C3));
    assert!(is_square_attacked(&board, E5));
  }

  #[test]
  fn test_bishop_corner() {
    // Black bishop on a1 attacks diagonal up-right.
    let board = get_board("8/8/8/8/8/8/8/b7 w - - 0 1");
    assert!(is_square_attacked(&board, B2));
    assert!(is_square_attacked(&board, C3));
    assert!(is_square_attacked(&board, H8));
    assert!(!is_square_attacked(&board, A1));
  }

  // Rook Tests
  #[test]
  fn test_rook_attacks() {
    // Black rook on d4 attacks along rank and file.
    let board = get_board("8/8/8/8/3r4/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, D1));
    assert!(is_square_attacked(&board, D8));
    assert!(is_square_attacked(&board, A4));
    assert!(is_square_attacked(&board, H4));
    assert!(!is_square_attacked(&board, E5));
  }

  #[test]
  fn test_rook_blocked() {
    // Black rook on d4, but file attacks are blocked by white pawns.
    let board = get_board("8/8/3P4/3r4/3P4/8/8/8 w - - 0 1");
    assert!(!is_square_attacked(&board, D1));
    assert!(!is_square_attacked(&board, D8));
    assert!(is_square_attacked(&board, A5));
    assert!(is_square_attacked(&board, H5));
  }

  #[test]
  fn test_rook_friendly_block() {
    // Black rook on d4 with black pawns blocking horizontally; horizontal rays blocked, vertical open.
    let board = get_board("8/8/8/8/2prp3/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, D1));
    assert!(is_square_attacked(&board, D8));
    // Immediate friendly pieces mean those squares and beyond are not attacked horizontally
    assert!(is_square_attacked(&board, C4)); // occupied by friendly piece still attacked
    assert!(is_square_attacked(&board, E4)); // occupied by friendly piece still attacked
    assert!(!is_square_attacked(&board, A4));
    assert!(!is_square_attacked(&board, H4));
  }

  // Queen Tests
  #[test]
  fn test_queen_attacks() {
    // Black queen on d4 attacks like a rook and bishop.
    let board = get_board("8/8/8/8/3q4/8/8/8 w - - 0 1");
    // Rook-like moves
    assert!(is_square_attacked(&board, D1));
    assert!(is_square_attacked(&board, D8));
    assert!(is_square_attacked(&board, A4));
    assert!(is_square_attacked(&board, H4));
    // Bishop-like moves
    assert!(is_square_attacked(&board, A1));
    assert!(is_square_attacked(&board, H8));
    assert!(!is_square_attacked(&board, E2));
  }

  #[test]
  fn test_queen_blocked() {
    // Black queen on d4 but surrounded by friendly pieces blocking lines.
    let board = get_board("8/8/8/3p1p2/2pqp3/5p2/8/8 w - - 0 1");
    // Immediate adjacent diagonals / orthogonals with blockers should not allow attacks beyond.
    assert!(is_square_attacked(&board, C3)); // adjacent blocker square itself attacked
    assert!(!is_square_attacked(&board, C2)); // not on queen line and no pawn now
    assert!(!is_square_attacked(&board, H4)); // far horizontal blocked
  }

  #[test]
  fn test_queen_mixed_attacks() {
    // Black queen centralized with partial blockers allowing some rays.
    let board = get_board("8/8/8/8/3q4/3P4/8/8 w - - 0 1");
    // Up-left diagonal open
    assert!(is_square_attacked(&board, C3));
    // Up file blocked at d3 (white pawn) so further d2 not attacked
    assert!(is_square_attacked(&board, D3));
    assert!(!is_square_attacked(&board, D2));
    // Down-right diagonal open
    assert!(is_square_attacked(&board, E3));
  }

  // King Tests
  #[test]
  fn test_king_attacks() {
    // Black king on d4 attacks adjacent squares.
    let board = get_board("8/8/8/8/3k4/8/8/8 w - - 0 1");
    assert!(is_square_attacked(&board, C3));
    assert!(is_square_attacked(&board, D3));
    assert!(is_square_attacked(&board, E3));
    assert!(is_square_attacked(&board, C4));
    assert!(is_square_attacked(&board, E4));
    assert!(is_square_attacked(&board, C5));
    assert!(is_square_attacked(&board, D5));
    assert!(is_square_attacked(&board, E5));
    assert!(!is_square_attacked(&board, D4));
    assert!(!is_square_attacked(&board, A1));
  }

  #[test]
  fn test_no_attacks() {
    let board = get_board("8/8/8/8/8/8/8/8 w - - 0 1");
    for i in 0..64 {
      assert!(!is_square_attacked(&board, i));
    }
  }

  #[test]
  fn test_initial_position_no_attacks_in_center() {
    let board = get_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert!(!is_square_attacked(&board, E4));
    assert!(!is_square_attacked(&board, D4));
    assert!(!is_square_attacked(&board, E5));
    assert!(!is_square_attacked(&board, D5));
  }

  #[test]
  fn test_reciprocal_king_attack() {
    // Black king on e8, white king on e1. Black to move.
    // The white king on e1 is attacked by the black king on e8.
    let board = get_board("4k3/8/8/8/8/8/8/4K3 b - - 0 1");
    assert!(is_square_attacked(&board, E2));
    assert!(is_square_attacked(&board, D2));
    assert!(is_square_attacked(&board, F2));
  }

  #[test]
  fn test_king_edge_attacks() {
    // Black king on a1 only 3 adjacent squares.
    let board = get_board("8/8/8/8/8/8/8/k7 w - - 0 1");
    assert!(is_square_attacked(&board, A2));
    assert!(is_square_attacked(&board, B1));
    assert!(is_square_attacked(&board, B2));
    assert!(!is_square_attacked(&board, A1));
    assert!(!is_square_attacked(&board, C2));
  }
}
