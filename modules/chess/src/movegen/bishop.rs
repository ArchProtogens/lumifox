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

use crate::{
  constants::{FILE_A, FILE_H}, // Added FILE_A for wrap-around protection
  model::{bitboard::BitBoard, gameboard::GameBoard, piecemove::PieceMove},
};

pub const MAX_BISHOP_MOVES: usize = 28;

pub(crate) fn generate_bishop_moves(state: &GameBoard) -> ([PieceMove; MAX_BISHOP_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_BISHOP_MOVES];
  let mut count = 0;

  let all_occupied =
    state.pawns | state.knights | state.bishops | state.rooks | state.queens | state.kings;

  let (my_bishops, other_pieces): (BitBoard, u64) = if state.playing {
    (
      state.bishops & state.colour,
      (all_occupied & !state.colour).into(),
    )
  } else {
    (
      state.bishops & !state.colour,
      (all_occupied & state.colour).into(),
    )
  };

  // Ray-casting for all 4 diagonal directions

  // 1. Top Left moves (shift by 7)
  let mut ray_attackers: u64 = my_bishops.into();
  for i in 1..8 {
    // We move the bishops up-left, and remove all who warp around to file H.
    ray_attackers = (ray_attackers << 7) & !FILE_H;

    // Potential captures are ray attacks that land on an opponent's piece.
    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board - (i * 7);

      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;

      // Remove this processed capture from the captures bitboard.
      captures &= captures - 1;
    }

    // The ray is blocked by any piece it hits.
    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    // Process quiet moves (those that didn't land on a blocker).
    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board - (i * 7);

      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;

      // Remove this processed move.
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  // 2. Top Right moves (shift by 9)
  ray_attackers = my_bishops.into();
  for i in 1..8 {
    // We move the bishops up-right, and remove all who warp around to file A.
    ray_attackers = (ray_attackers << 9) & !FILE_A;

    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board - (i * 9);
      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;
      captures &= captures - 1;
    }

    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board - (i * 9);
      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  // 3. Bottom Left moves (shift by -9)
  ray_attackers = my_bishops.into();
  for i in 1..8 {
    // We move the bishops down-left, and remove all who warp around to file H.
    ray_attackers = (ray_attackers >> 9) & !FILE_H;

    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board + (i * 9);
      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;
      captures &= captures - 1;
    }

    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board + (i * 9);
      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  // 4. Bottom Right moves (shift by -7)
  ray_attackers = my_bishops.into();
  for i in 1..8 {
    // We move the bishops down-right, and remove all who warp around to file A.
    ray_attackers = (ray_attackers >> 7) & !FILE_A;

    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board + (i * 7);
      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;
      captures &= captures - 1;
    }

    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board + (i * 7);
      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  (moves, count)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::constants::*;
  use crate::model::gamedata::GameData;
  use crate::model::piecemove::PieceMove;

  // Helper function to sort and compare PieceMove arrays
  fn sort_and_compare_moves(mut moves: Vec<PieceMove>) -> Vec<PieceMove> {
    moves.sort_by_key(|m| {
      (
        m.from_square(),
        m.to_square(),
        m.is_capture(),
        m.promotion_type().map(|p| p as u8).unwrap_or(0),
      )
    });
    moves
  }

  #[test]
  fn test_single_white_bishop_center() {
    // White bishop on D4 with clear diagonals
    let board = GameData::from_fen("8/8/8/8/3B4/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Top-left diagonal
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      // Top-right diagonal
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None),
      // Bottom-left diagonal
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, false, None),
      PieceMove::new(D4, A1, false, None),
      // Bottom-right diagonal
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, false, None),
      PieceMove::new(D4, G1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_single_black_bishop_center() {
    // Black bishop on D4 with clear diagonals
    let board = GameData::from_fen("8/8/8/8/3b4/8/8/8 b - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Top-left diagonal
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      // Top-right diagonal
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None),
      // Bottom-left diagonal
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, false, None),
      PieceMove::new(D4, A1, false, None),
      // Bottom-right diagonal
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, false, None),
      PieceMove::new(D4, G1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_bishop_corner() {
    // White bishop on A1 corner
    let board = GameData::from_fen("8/8/8/8/8/8/8/B7 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Only top-right diagonal available from A1
      PieceMove::new(A1, B2, false, None),
      PieceMove::new(A1, C3, false, None),
      PieceMove::new(A1, D4, false, None),
      PieceMove::new(A1, E5, false, None),
      PieceMove::new(A1, F6, false, None),
      PieceMove::new(A1, G7, false, None),
      PieceMove::new(A1, H8, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_bishop_h8_corner() {
    // White bishop on H8 corner
    let board = GameData::from_fen("7B/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Only bottom-left diagonal available from H8
      PieceMove::new(H8, G7, false, None),
      PieceMove::new(H8, F6, false, None),
      PieceMove::new(H8, E5, false, None),
      PieceMove::new(H8, D4, false, None),
      PieceMove::new(H8, C3, false, None),
      PieceMove::new(H8, B2, false, None),
      PieceMove::new(H8, A1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_bishop_captures() {
    // White bishop on D4 with black pieces to capture
    let board = GameData::from_fen("8/3p3p/8/8/3B4/8/1p3p2/8 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Top-left diagonal - capture on D7
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      // Top-right diagonal
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None), // Capture
      // Bottom-left diagonal - capture on B2
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, true, None), // Capture
      // Bottom-right diagonal - capture on F2
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, true, None), // Capture
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_black_bishop_captures() {
    // Black bishop on D4 with white pieces to capture
    let board = GameData::from_fen("8/3P3P/8/8/3b4/8/1P3P2/8 b - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Top-left diagonal - capture on D7
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      // Top-right diagonal
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None), // Capture
      // Bottom-left diagonal - capture on B2
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, true, None), // Capture
      // Bottom-right diagonal - capture on F2
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, true, None), // Capture
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_bishop_blocked_by_own_pieces() {
    // White bishop on D4 blocked by own pawns
    let board = GameData::from_fen("8/8/8/2P1P3/3B4/2P1P3/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Bishop should have no moves as all diagonals are blocked by own pieces
    let expected_moves = vec![];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_bishop_partially_blocked() {
    // White bishop on D4 with some diagonals blocked
    let board = GameData::from_fen("8/8/8/8/3B4/8/1P6/8 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Top-left diagonal - clear
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      // Top-right diagonal - clear
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None),
      // Bottom-left diagonal - blocked at B2
      PieceMove::new(D4, C3, false, None),
      // Bottom-right diagonal - clear
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, false, None),
      PieceMove::new(D4, G1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_multiple_bishops() {
    // Two white bishops on the board
    let board = GameData::from_fen("8/8/8/8/3B4/8/8/B7 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Bishop on D4 moves
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None),
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, false, None),
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, false, None),
      PieceMove::new(D4, G1, false, None),
      // Bishop on A1 moves (can't go to A1 as it's blocked by D4 bishop)
      PieceMove::new(A1, B2, false, None),
      PieceMove::new(A1, C3, false, None),
      // Note: D4 is occupied by own bishop, so A1 bishop can't go there
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_bishop_edge_cases() {
    // Bishop on edge of board
    let board = GameData::from_fen("8/8/8/8/B7/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // From A4, only top-right and bottom-right diagonals available
      PieceMove::new(A4, B5, false, None),
      PieceMove::new(A4, C6, false, None),
      PieceMove::new(A4, D7, false, None),
      PieceMove::new(A4, E8, false, None),
      PieceMove::new(A4, B3, false, None),
      PieceMove::new(A4, C2, false, None),
      PieceMove::new(A4, D1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_no_bishops() {
    // No bishops on the board
    let board = GameData::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let (_moves, count) = generate_bishop_moves(&board.board);

    assert_eq!(count, 0);
  }

  #[test]
  fn test_bishop_initial_position() {
    // Standard chess starting position bishops
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (_moves, count) = generate_bishop_moves(&board.board);

    // Bishops should have no moves in starting position due to pawns blocking
    assert_eq!(count, 0);
  }

  #[test]
  fn test_bishop_after_pawn_moves() {
    // Position after some pawn moves to open up bishop diagonals
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // White bishop on F1 can now move
      PieceMove::new(F1, E2, false, None),
      PieceMove::new(F1, D3, false, None),
      PieceMove::new(F1, C4, false, None),
      PieceMove::new(F1, B5, false, None),
      PieceMove::new(F1, A6, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_complex_bishop_position() {
    // Complex position with mixed piece placement
    let board =
      GameData::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1")
        .unwrap();
    let (moves, count) = generate_bishop_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // This tests a more realistic game position
    // White bishop on C4 should have several moves available
    let mut found_bishop_moves = false;
    for move_item in &generated_moves {
      if move_item.from_square() == C4 {
        found_bishop_moves = true;
        break;
      }
    }
    assert!(found_bishop_moves, "Should find moves for bishop on C4");
    assert!(count > 0, "Should generate some bishop moves");
  }
}
