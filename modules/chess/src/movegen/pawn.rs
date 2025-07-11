/*
 * A simple and growing chess library in Rust.
 * Copyright (C) 2025  Clifton Toaster Reid
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::constants::*;
use crate::model::{
  gameboard::GameBoard,
  piecemove::{PieceMove, PromotionType},
};

const MAX_PAWN_MOVES: usize = 16;

/// Helper function to add a move
#[inline]
fn add_move_to_list(
  moves: &mut [PieceMove; MAX_PAWN_MOVES],
  count: &mut usize,
  piece_move: PieceMove,
) {
  if *count < MAX_PAWN_MOVES {
    moves[*count] = piece_move;
    *count += 1;
  } else {
    #[cfg(debug_assertions)]
    panic!(
      "Move array overflow! MAX_PAWN_MOVES (={}) is too small.",
      MAX_PAWN_MOVES
    );
  }
}

pub(crate) fn generate_pawn_moves(state: &GameBoard) -> ([PieceMove; MAX_PAWN_MOVES], usize) {
  let mut moves: [PieceMove; MAX_PAWN_MOVES] = Default::default();
  let mut count = 0;

  let all_occupied =
    state.pawns | state.knights | state.bishops | state.rooks | state.queens | state.kings;
  let empty_squares = !all_occupied;

  let single_pushes;
  let double_pushes;
  let right_captures;
  let left_captures;

  if state.playing {
    let white_pawns = state.pawns & state.colour;
    let opponent_pieces = all_occupied & !state.colour;

    // --- White Pawn Moves ---

    // 1. Single Push: Pawns move one step forward (up the board)
    single_pushes = (white_pawns << 8) & empty_squares;

    // 2. Double Push: Pawns on their base rank move two steps forward
    //    - Must start on RANK_2.
    //    - The square one step ahead must be empty (already checked by `single_pushes`).
    //    - The square two steps ahead must also be empty.
    let double_push_starts = single_pushes & RANK_3; // Pawns that successfully moved one step to rank 3
    double_pushes = (double_push_starts << 8) & empty_squares;

    // 3. Captures
    right_captures = (white_pawns << 9) & opponent_pieces & !FILE_A; // Capture right, avoiding wrap-around
    left_captures = (white_pawns << 7) & opponent_pieces & !FILE_H; // Capture left, avoiding wrap-around
  } else {
    // Black's turn
    let black_pawns = state.pawns & !state.colour;
    let opponent_pieces = all_occupied & state.colour;

    // --- Black Pawn Moves ---

    // 1. Single Push: Pawns move one step forward (down the board)
    single_pushes = (black_pawns >> 8) & empty_squares;

    // 2. Double Push: Pawns on their base rank move two steps forward
    //    - Must start on RANK_7.
    //    - The square one step ahead must be empty.
    //    - The square two steps ahead must also be empty.
    let double_push_starts = single_pushes & RANK_6; // Pawns that successfully moved one step to rank 6
    double_pushes = (double_push_starts >> 8) & empty_squares;

    // 3. Captures
    right_captures = (black_pawns >> 7) & opponent_pieces & !FILE_A; // Capture right
    left_captures = (black_pawns >> 9) & opponent_pieces & !FILE_H; // Capture left
  }

  // TODO: Convert bitboards (single_pushes, double_pushes, etc.) to PieceMove objects.
  // TODO: Handle promotions (any move landing on RANK_8).
  // TODO: Handle en passant captures.

  // 1. Single Pushes
  let mut tmp_single: u64 = single_pushes.into();
  while tmp_single != 0 {
    let to_sq_idx = tmp_single.trailing_zeros() as u8;
    let to_sq_bb = 1u64 << to_sq_idx; // Bitboard for the 'to' square

    let from_sq_idx = if state.playing {
      to_sq_idx - 8 // White pawns move up
    } else {
      to_sq_idx + 8 // Black pawns move down
    };

    // Check for promotion
    let is_promotion_rank =
      (state.playing && (to_sq_bb & RANK_8) != 0) || (!state.playing && (to_sq_bb & RANK_1) != 0);

    if is_promotion_rank {
      // Generate 4 promotion moves (Queen, Rook, Bishop, Knight)
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Queen)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Rook)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Bishop)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Knight)),
      );
    } else {
      // Normal single push
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, false, None),
      );
    }

    tmp_single &= tmp_single - 1; // Clear the least significant bit
  }

  // 2. Double Pushes
  let mut tmp_double: u64 = double_pushes.into();
  while tmp_double != 0 {
    let to_sq_idx = tmp_double.trailing_zeros() as u8;

    // Determine the 'from' square based on the direction of the push
    let from_sq_idx = if state.playing {
      to_sq_idx - 16
    } else {
      to_sq_idx + 16
    };

    // Double pushes are not captures, and CANNOT be promotions.
    // The `is_two_square_advance` information isn't directly in PieceMove's packed bits,
    // but the (from, to) squares uniquely identify it for a pawn.
    add_move_to_list(
      &mut moves,
      &mut count,
      PieceMove::new(from_sq_idx, to_sq_idx, false, None),
    );

    tmp_double &= tmp_double - 1; // Clear the least significant bit
  }

  // 3. Right Captures
  let mut tmp_right: u64 = right_captures.into();
  while tmp_right != 0 {
    let to_sq_idx = tmp_right.trailing_zeros() as u8;
    let to_sq_bb = 1u64 << to_sq_idx;

    // Determine the 'from' square based on the direction of the capture
    let from_sq_idx = if state.playing {
      to_sq_idx - 9
    } else {
      to_sq_idx + 7
    };

    // Check for promotion (capturing promotion)
    let is_promotion_rank =
      (state.playing && (to_sq_bb & RANK_8) != 0) || (!state.playing && (to_sq_bb & RANK_1) != 0);

    if is_promotion_rank {
      // Generate 4 capturing promotion moves
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Queen)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Rook)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Bishop)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Knight)),
      );
    } else {
      // Normal capture
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, None),
      );
    }

    // Clear the least significant bit
    tmp_right &= tmp_right - 1;
  }

  // 4. Left Captures
  let mut tmp_left: u64 = left_captures.into();
  while tmp_left != 0 {
    let to_sq_idx = tmp_left.trailing_zeros() as u8;
    let to_sq_bb = 1u64 << to_sq_idx;

    // Determine the 'from' square based on the direction of the capture
    let from_sq_idx = if state.playing {
      to_sq_idx - 7
    } else {
      to_sq_idx + 9
    };

    // Check for promotion (capturing promotion)
    let is_promotion_rank =
      (state.playing && (to_sq_bb & RANK_8) != 0) || (!state.playing && (to_sq_bb & RANK_1) != 0);

    if is_promotion_rank {
      // Generate 4 capturing promotion moves
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Queen)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Rook)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Bishop)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Knight)),
      );
    } else {
      // Normal capture
      add_move_to_list(
        &mut moves,
        &mut count,
        PieceMove::new(from_sq_idx, to_sq_idx, true, None),
      );
    }

    // Clear the least significant bit
    tmp_left &= tmp_left - 1;
  }

  (moves, count)
}
