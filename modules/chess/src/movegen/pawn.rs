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

use crate::constants::*;
use crate::model::{
  gameboard::GameBoard,
  piecemove::{PieceMove, PromotionType},
};
use crate::movegen::add_move_to_list;

pub const MAX_PAWN_MOVES: usize = 56;

pub(crate) fn generate_pawn_moves(state: &GameBoard) -> ([PieceMove; MAX_PAWN_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_PAWN_MOVES];
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
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Queen)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Rook)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Bishop)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, false, Some(PromotionType::Knight)),
      );
    } else {
      // Normal single push
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
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
      MAX_PAWN_MOVES,
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
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Queen)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Rook)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Bishop)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Knight)),
      );
    } else {
      // Normal capture
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
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
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Queen)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Rook)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Bishop)),
      );
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, Some(PromotionType::Knight)),
      );
    } else {
      // Normal capture
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new(from_sq_idx, to_sq_idx, true, None),
      );
    }

    // Clear the least significant bit
    tmp_left &= tmp_left - 1;
  }

  // 5. En Passant captures
  if state.en_passant != PieceMove::NULL {
    let ep_target_sq = state.en_passant.to_square();
    let ep_target_bb = 1u64 << ep_target_sq;

    let pawn_attacks = if state.playing {
      // White attacks black pawns
      ((ep_target_bb >> 7) & !FILE_A) | ((ep_target_bb >> 9) & !FILE_H)
    } else {
      // Black attacks white pawns
      ((ep_target_bb << 7) & !FILE_H) | ((ep_target_bb << 9) & !FILE_A)
    };

    let friendly_pawns: u64 = if state.playing {
      (state.pawns & state.colour).into()
    } else {
      (state.pawns & !state.colour).into()
    };

    let mut attackers = pawn_attacks & friendly_pawns;
    while attackers != 0 {
      let from_sq = attackers.trailing_zeros() as u8;
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_PAWN_MOVES,
        PieceMove::new_en_passant(from_sq, ep_target_sq),
      );
      attackers &= attackers - 1;
    }
  }

  (moves, count)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::gamedata::GameData;
  use crate::model::piecemove::{PieceMove, PromotionType};

  // Helper function to sort and compare PieceMove arrays
  fn sort_and_compare_moves(mut moves: Vec<PieceMove>) -> Vec<PieceMove> {
    moves.sort();
    moves
  }

  #[test]
  fn test_white_pawn_single_pushes() {
    let board = GameData::from_fen("8/8/8/8/8/8/P7/8 w - - 0 1").unwrap(); // White pawn on A2
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A3, false, None),
      PieceMove::new(A2, A4, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/8/8/8/8/8/PPPPPPPP/8 w - - 0 1").unwrap(); // All white pawns on 2nd rank
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A3, false, None),
      PieceMove::new(B2, B3, false, None),
      PieceMove::new(C2, C3, false, None),
      PieceMove::new(D2, D3, false, None),
      PieceMove::new(E2, E3, false, None),
      PieceMove::new(F2, F3, false, None),
      PieceMove::new(G2, G3, false, None),
      PieceMove::new(H2, H3, false, None),
      // Double pushes
      PieceMove::new(A2, A4, false, None),
      PieceMove::new(B2, B4, false, None),
      PieceMove::new(C2, C4, false, None),
      PieceMove::new(D2, D4, false, None),
      PieceMove::new(E2, E4, false, None),
      PieceMove::new(F2, F4, false, None),
      PieceMove::new(G2, G4, false, None),
      PieceMove::new(H2, H4, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_pawn_double_pushes() {
    let board = GameData::from_fen("8/8/8/8/8/8/P7/8 w - - 0 1").unwrap(); // White pawn on A2
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A3, false, None),
      PieceMove::new(A2, A4, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/8/8/8/8/8/PPPPPPPP/8 w - - 0 1").unwrap(); // All white pawns on 2nd rank
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A3, false, None),
      PieceMove::new(A2, A4, false, None),
      PieceMove::new(B2, B3, false, None),
      PieceMove::new(B2, B4, false, None),
      PieceMove::new(C2, C3, false, None),
      PieceMove::new(C2, C4, false, None),
      PieceMove::new(D2, D3, false, None),
      PieceMove::new(D2, D4, false, None),
      PieceMove::new(E2, E3, false, None),
      PieceMove::new(E2, E4, false, None),
      PieceMove::new(F2, F3, false, None),
      PieceMove::new(F2, F4, false, None),
      PieceMove::new(G2, G3, false, None),
      PieceMove::new(G2, G4, false, None),
      PieceMove::new(H2, H3, false, None),
      PieceMove::new(H2, H4, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_pawn_captures() {
    let board = GameData::from_fen("8/8/8/8/3p4/2P5/8/8 w - - 0 1").unwrap(); // White pawn on C3, black pawn on D4
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(C3, C4, false, None),
      PieceMove::new(C3, D4, true, None), // Capture
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/8/8/8/1p1p4/2P5/8/8 w - - 0 1").unwrap(); // White pawn on C3, black pawns on B4 and D4
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(C3, C4, false, None),
      PieceMove::new(C3, B4, true, None), // Capture
      PieceMove::new(C3, D4, true, None), // Capture
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_pawn_promotions() {
    let board = GameData::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap(); // White pawn on A7
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A8, false, Some(PromotionType::Queen)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Rook)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Bishop)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Knight)),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("7p/P7/8/8/8/8/8/8 w - - 0 1").unwrap(); // White pawn on A7, black pawn on H8
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A8, false, Some(PromotionType::Queen)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Rook)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Bishop)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Knight)),
      // No capture promotion for now, as there's no piece to capture on B8
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("1p6/P7/8/8/8/8/8/8 w - - 0 1").unwrap(); // White pawn on A7, black pawn on C7
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A8, false, Some(PromotionType::Queen)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Rook)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Bishop)),
      PieceMove::new(A7, A8, false, Some(PromotionType::Knight)),
      PieceMove::new(A7, B8, true, Some(PromotionType::Queen)), // Capture promotion
      PieceMove::new(A7, B8, true, Some(PromotionType::Rook)),
      PieceMove::new(A7, B8, true, Some(PromotionType::Bishop)),
      PieceMove::new(A7, B8, true, Some(PromotionType::Knight)),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_black_pawn_single_pushes() {
    let board = GameData::from_fen("8/p7/8/8/8/8/8/8 b - - 0 1").unwrap(); // Black pawn on A7
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A6, false, None),
      PieceMove::new(A7, A5, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/pppppppp/8/8/8/8/8/8 b - - 0 1").unwrap(); // All black pawns on 7th rank
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A6, false, None),
      PieceMove::new(B7, B6, false, None),
      PieceMove::new(C7, C6, false, None),
      PieceMove::new(D7, D6, false, None),
      PieceMove::new(E7, E6, false, None),
      PieceMove::new(F7, F6, false, None),
      PieceMove::new(G7, G6, false, None),
      PieceMove::new(H7, H6, false, None),
      // Double pushes
      PieceMove::new(A7, A5, false, None),
      PieceMove::new(B7, B5, false, None),
      PieceMove::new(C7, C5, false, None),
      PieceMove::new(D7, D5, false, None),
      PieceMove::new(E7, E5, false, None),
      PieceMove::new(F7, F5, false, None),
      PieceMove::new(G7, G5, false, None),
      PieceMove::new(H7, H5, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_black_pawn_double_pushes() {
    let board = GameData::from_fen("8/p7/8/8/8/8/8/8 b - - 0 1").unwrap(); // Black pawn on A7
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A6, false, None),
      PieceMove::new(A7, A5, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/pppppppp/8/8/8/8/8/8 b - - 0 1").unwrap(); // All black pawns on 7th rank
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A6, false, None),
      PieceMove::new(A7, A5, false, None),
      PieceMove::new(B7, B6, false, None),
      PieceMove::new(B7, B5, false, None),
      PieceMove::new(C7, C6, false, None),
      PieceMove::new(C7, C5, false, None),
      PieceMove::new(D7, D6, false, None),
      PieceMove::new(D7, D5, false, None),
      PieceMove::new(E7, E6, false, None),
      PieceMove::new(E7, E5, false, None),
      PieceMove::new(F7, F6, false, None),
      PieceMove::new(F7, F5, false, None),
      PieceMove::new(G7, G6, false, None),
      PieceMove::new(G7, G5, false, None),
      PieceMove::new(H7, H6, false, None),
      PieceMove::new(H7, H5, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_black_pawn_captures() {
    let board = GameData::from_fen("8/8/2p5/3P4/8/8/8/8 b - - 0 1").unwrap(); // Black pawn on C6, white pawn on D5
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(C6, D5, true, None),
      PieceMove::new(C6, C5, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/8/1P1P4/2p5/8/8/8/8 b - - 0 1").unwrap(); // Black pawn on C5, white pawns on B6 and D6
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![PieceMove::new(C5, C4, false, None)];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_black_pawn_promotions() {
    let board = GameData::from_fen("8/8/8/8/8/8/p7/8 b - - 0 1").unwrap(); // Black pawn on A2
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A1, false, Some(PromotionType::Queen)),
      PieceMove::new(A2, A1, false, Some(PromotionType::Rook)),
      PieceMove::new(A2, A1, false, Some(PromotionType::Bishop)),
      PieceMove::new(A2, A1, false, Some(PromotionType::Knight)),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );

    let board = GameData::from_fen("8/8/8/8/8/8/p7/1P6 b - - 0 1").unwrap(); // Black pawn on A2, white pawn on C2
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A1, false, Some(PromotionType::Queen)),
      PieceMove::new(A2, A1, false, Some(PromotionType::Rook)),
      PieceMove::new(A2, A1, false, Some(PromotionType::Bishop)),
      PieceMove::new(A2, A1, false, Some(PromotionType::Knight)),
      PieceMove::new(A2, B1, true, Some(PromotionType::Queen)), // Capture promotion
      PieceMove::new(A2, B1, true, Some(PromotionType::Rook)),
      PieceMove::new(A2, B1, true, Some(PromotionType::Bishop)),
      PieceMove::new(A2, B1, true, Some(PromotionType::Knight)),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_en_passant_white() {
    // White pawn on E5, black pawn on D5 (just moved from D7-D5)
    let board =
      GameData::from_fen("rnbqkbnr/ppp1pppp/8/2Pp4/8/8/PP1PPPPP/RNBQKBNR w KQkq d6 0 1").unwrap();
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Other pawn moves...
      PieceMove::new_en_passant(C5, D6), // En passant capture
    ];
    // Filter for en passant moves only for this test
    let filtered_moves: Vec<PieceMove> = generated_moves
      .into_iter()
      .filter(|m| m.is_en_passant())
      .collect();
    assert_eq!(
      sort_and_compare_moves(filtered_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_en_passant_black() {
    // Black pawn on E4, white pawn on D4 (just moved from D2-D4)
    let board =
      GameData::from_fen("rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1").unwrap();
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new_en_passant(E4, D3), // En passant capture
    ];
    // Filter for en passant moves only for this test
    let filtered_moves: Vec<PieceMove> = generated_moves
      .into_iter()
      .filter(|m| m.is_en_passant())
      .collect();
    assert_eq!(
      sort_and_compare_moves(filtered_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_pawn_blocked() {
    let board = GameData::from_fen("8/8/8/8/8/P1P5/1P1P4/8 w - - 0 1").unwrap(); // Pawns on A2, B3, C2, D3
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Single pushes
      PieceMove::new(A3, A4, false, None),
      PieceMove::new(B2, B3, false, None),
      PieceMove::new(C3, C4, false, None),
      PieceMove::new(D2, D3, false, None),
      // Double pushes
      PieceMove::new(B2, B4, false, None),
      PieceMove::new(D2, D4, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_initial_position_white_pawns() {
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A2, A3, false, None),
      PieceMove::new(A2, A4, false, None),
      PieceMove::new(B2, B3, false, None),
      PieceMove::new(B2, B4, false, None),
      PieceMove::new(C2, C3, false, None),
      PieceMove::new(C2, C4, false, None),
      PieceMove::new(D2, D3, false, None),
      PieceMove::new(D2, D4, false, None),
      PieceMove::new(E2, E3, false, None),
      PieceMove::new(E2, E4, false, None),
      PieceMove::new(F2, F3, false, None),
      PieceMove::new(F2, F4, false, None),
      PieceMove::new(G2, G3, false, None),
      PieceMove::new(G2, G4, false, None),
      PieceMove::new(H2, H3, false, None),
      PieceMove::new(H2, H4, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_initial_position_black_pawns() {
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
    let (moves, count) = generate_pawn_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(A7, A6, false, None),
      PieceMove::new(A7, A5, false, None),
      PieceMove::new(B7, B6, false, None),
      PieceMove::new(B7, B5, false, None),
      PieceMove::new(C7, C6, false, None),
      PieceMove::new(C7, C5, false, None),
      PieceMove::new(D7, D6, false, None),
      PieceMove::new(D7, D5, false, None),
      PieceMove::new(E7, E6, false, None),
      PieceMove::new(E7, E5, false, None),
      PieceMove::new(F7, F6, false, None),
      PieceMove::new(F7, F5, false, None),
      PieceMove::new(G7, G6, false, None),
      PieceMove::new(G7, G5, false, None),
      PieceMove::new(H7, H6, false, None),
      PieceMove::new(H7, H5, false, None),
    ];
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }
}
