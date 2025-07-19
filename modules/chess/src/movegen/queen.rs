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
  constants::{FILE_A, FILE_H},
  model::{bitboard::BitBoard, gameboard::GameBoard, piecemove::PieceMove},
};

pub const MAX_QUEEN_MOVES: usize = 56; // 28 (rook-like) + 28 (bishop-like) = 56 max

pub(crate) fn generate_queen_moves(state: &GameBoard) -> ([PieceMove; MAX_QUEEN_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_QUEEN_MOVES];
  let mut count = 0;

  let all_occupied =
    state.pawns | state.knights | state.bishops | state.rooks | state.queens | state.kings;

  let (my_queens, other_pieces): (BitBoard, u64) = if state.playing {
    (
      state.queens & state.colour,
      (all_occupied & !state.colour).into(),
    )
  } else {
    (
      state.queens & !state.colour,
      (all_occupied & state.colour).into(),
    )
  };

  // Queen moves are combination of rook and bishop moves
  // Using a unified approach with direction data: (shift_amount, mask, is_positive_shift)
  let queen_directions: [(i8, u64, bool); 8] = [
    // Rook-like moves (orthogonal)
    (8, 0, true),        // Up
    (1, FILE_A, true),   // Right (mask FILE_A to prevent wrap-around)
    (-8, 0, false),      // Down
    (-1, FILE_H, false), // Left (mask FILE_H to prevent wrap-around)
    // Bishop-like moves (diagonal)
    (7, FILE_H, true),   // Up-Left (mask FILE_H to prevent wrap-around)
    (9, FILE_A, true),   // Up-Right (mask FILE_A to prevent wrap-around)
    (-9, FILE_H, false), // Down-Left (mask FILE_H to prevent wrap-around)
    (-7, FILE_A, false), // Down-Right (mask FILE_A to prevent wrap-around)
  ];

  for (shift, mask, is_positive) in queen_directions {
    let mut ray_attackers: u64 = my_queens.into();

    for i in 1..8 {
      // Apply the shift for this direction
      if is_positive {
        ray_attackers <<= shift as u8;
      } else {
        ray_attackers >>= (-shift) as u8;
      }

      // Apply the mask to prevent wrap-around
      ray_attackers &= !mask;

      // Process captures
      let mut captures = ray_attackers & other_pieces;
      while captures != 0 {
        let to_board = captures.trailing_zeros() as u8;
        let from_board = if is_positive {
          to_board - (i * (shift as u8))
        } else {
          to_board + (i * ((-shift) as u8))
        };

        if count < MAX_QUEEN_MOVES {
          moves[count] = PieceMove::new(from_board, to_board, true, None);
          count += 1;
        }

        // Remove this processed capture
        captures &= captures - 1;
      }

      // The ray is blocked by any piece it hits
      let blockers = ray_attackers & all_occupied.raw();
      ray_attackers &= !blockers;

      // Process quiet moves
      let mut quiet_moves = ray_attackers;
      while quiet_moves != 0 {
        let to_board = quiet_moves.trailing_zeros() as u8;
        let from_board = if is_positive {
          to_board - (i * (shift as u8))
        } else {
          to_board + (i * ((-shift) as u8))
        };

        if count < MAX_QUEEN_MOVES {
          moves[count] = PieceMove::new(from_board, to_board, false, None);
          count += 1;
        }

        // Remove this processed move
        quiet_moves &= quiet_moves - 1;
      }

      // If no more pieces can move in this direction, break
      if ray_attackers == 0 {
        break;
      }
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
    moves.sort();
    moves
  }

  #[test]
  fn test_single_white_queen_center() {
    // White queen on D4 with clear ranks, files, and diagonals
    let board = GameData::from_fen("8/8/8/8/3Q4/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Rook-like moves
      // Up the D file
      PieceMove::new(D4, D5, false, None),
      PieceMove::new(D4, D6, false, None),
      PieceMove::new(D4, D7, false, None),
      PieceMove::new(D4, D8, false, None),
      // Right on 4th rank
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, false, None),
      PieceMove::new(D4, G4, false, None),
      PieceMove::new(D4, H4, false, None),
      // Down the D file
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, D2, false, None),
      PieceMove::new(D4, D1, false, None),
      // Left on 4th rank
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, B4, false, None),
      PieceMove::new(D4, A4, false, None),
      // Bishop-like moves
      // Up-Left diagonal
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      // Up-Right diagonal
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None),
      // Down-Left diagonal
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, false, None),
      PieceMove::new(D4, A1, false, None),
      // Down-Right diagonal
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, false, None),
      PieceMove::new(D4, G1, false, None),
    ];

    assert_eq!(count, 27); // 14 rook moves + 13 bishop moves
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_single_black_queen_center() {
    // Black queen on D4 with clear ranks, files, and diagonals
    let board = GameData::from_fen("8/8/8/8/3q4/8/8/8 b - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Same moves as white queen test
    let expected_moves = vec![
      // Rook-like moves
      PieceMove::new(D4, D5, false, None),
      PieceMove::new(D4, D6, false, None),
      PieceMove::new(D4, D7, false, None),
      PieceMove::new(D4, D8, false, None),
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, false, None),
      PieceMove::new(D4, G4, false, None),
      PieceMove::new(D4, H4, false, None),
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, D2, false, None),
      PieceMove::new(D4, D1, false, None),
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, B4, false, None),
      PieceMove::new(D4, A4, false, None),
      // Bishop-like moves
      PieceMove::new(D4, C5, false, None),
      PieceMove::new(D4, B6, false, None),
      PieceMove::new(D4, A7, false, None),
      PieceMove::new(D4, E5, false, None),
      PieceMove::new(D4, F6, false, None),
      PieceMove::new(D4, G7, false, None),
      PieceMove::new(D4, H8, false, None),
      PieceMove::new(D4, C3, false, None),
      PieceMove::new(D4, B2, false, None),
      PieceMove::new(D4, A1, false, None),
      PieceMove::new(D4, E3, false, None),
      PieceMove::new(D4, F2, false, None),
      PieceMove::new(D4, G1, false, None),
    ];

    assert_eq!(count, 27);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_queen_corner_a1() {
    // White queen on A1 corner
    let board = GameData::from_fen("8/8/8/8/8/8/8/Q7 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Rook-like moves from A1
      // Up the A file
      PieceMove::new(A1, A2, false, None),
      PieceMove::new(A1, A3, false, None),
      PieceMove::new(A1, A4, false, None),
      PieceMove::new(A1, A5, false, None),
      PieceMove::new(A1, A6, false, None),
      PieceMove::new(A1, A7, false, None),
      PieceMove::new(A1, A8, false, None),
      // Right on 1st rank
      PieceMove::new(A1, B1, false, None),
      PieceMove::new(A1, C1, false, None),
      PieceMove::new(A1, D1, false, None),
      PieceMove::new(A1, E1, false, None),
      PieceMove::new(A1, F1, false, None),
      PieceMove::new(A1, G1, false, None),
      PieceMove::new(A1, H1, false, None),
      // Bishop-like moves from A1
      // Only Up-Right diagonal available from A1
      PieceMove::new(A1, B2, false, None),
      PieceMove::new(A1, C3, false, None),
      PieceMove::new(A1, D4, false, None),
      PieceMove::new(A1, E5, false, None),
      PieceMove::new(A1, F6, false, None),
      PieceMove::new(A1, G7, false, None),
      PieceMove::new(A1, H8, false, None),
    ];

    assert_eq!(count, 21); // 14 rook moves + 7 bishop moves
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_queen_captures() {
    // White queen on D4 with black pieces to capture
    let board = GameData::from_fen("8/3p1p1p/8/1p1Q1p2/8/1p1p1p2/3p1p1p/8 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      PieceMove::new(D5, A8, false, None),
      PieceMove::new(D5, B7, false, None),
      PieceMove::new(D5, C6, false, None),
      PieceMove::new(D5, D7, true, None), // Capture on D7
      PieceMove::new(D5, D6, false, None),
      PieceMove::new(D5, F7, true, None), // Capture on F7
      PieceMove::new(D5, E6, false, None),
      PieceMove::new(D5, B5, true, None), // Capture on B5
      PieceMove::new(D5, C5, false, None),
      PieceMove::new(D5, F5, true, None), // Capture on F5
      PieceMove::new(D5, E5, false, None),
      PieceMove::new(D5, B3, true, None), // Capture on B3
      PieceMove::new(D5, C4, false, None),
      PieceMove::new(D5, D3, true, None), // Capture on D3
      PieceMove::new(D5, D4, false, None),
      PieceMove::new(D5, F3, true, None), // Capture on F3
      PieceMove::new(D5, E4, false, None),
    ];

    // Should have some captures and some quiet moves
    assert!(count > 0);
    let captures_count = generated_moves.iter().filter(|m| m.is_capture()).count();
    assert!(captures_count > 0);

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_queen_blocked_by_own_pieces() {
    // White queen on D4 surrounded by own pieces
    let board = GameData::from_fen("8/8/8/2PPP3/2PQP3/2PPP3/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Queen should have no moves as all directions are blocked by own pieces
    let expected_moves = vec![];

    assert_eq!(count, 0);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_queen_partially_blocked() {
    // White queen on D4 with some directions blocked
    let board = GameData::from_fen("8/8/8/8/3Q4/8/1P6/8 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Most directions should be clear, only down-left diagonal blocked at B2
    let captures_and_quiet = generated_moves.len();
    assert!(captures_and_quiet > 20); // Should have most of the 27 possible moves

    // Check that some key moves are present
    let has_d8_move = generated_moves.iter().any(|m| m.to_square() == D8);
    let has_h4_move = generated_moves.iter().any(|m| m.to_square() == H4);
    let has_h8_move = generated_moves.iter().any(|m| m.to_square() == H8);

    assert!(has_d8_move, "Should be able to move to D8");
    assert!(has_h4_move, "Should be able to move to H4");
    assert!(has_h8_move, "Should be able to move to H8");
  }

  #[test]
  fn test_no_queens() {
    // No queens on the board
    let board = GameData::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let (_moves, count) = generate_queen_moves(&board.board);

    assert_eq!(count, 0);
  }

  #[test]
  fn test_queen_initial_position() {
    // Standard chess starting position
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (_moves, count) = generate_queen_moves(&board.board);

    // Queen should have no moves in starting position due to pieces blocking
    assert_eq!(count, 0);
  }

  #[test]
  fn test_queen_after_pawn_moves() {
    // Position after some pawn moves to open up queen diagonals
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // White queen on D1 can now move
      PieceMove::new(D1, E2, false, None),
      PieceMove::new(D1, F3, false, None),
      PieceMove::new(D1, G4, false, None),
      PieceMove::new(D1, H5, false, None),
    ];

    assert_eq!(count, 4);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_multiple_queens() {
    // Two white queens on the board (unlikely but valid for testing)
    let board = GameData::from_fen("8/8/8/8/3Q4/8/8/Q7 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Should include moves for both queens
    assert!(count > 27); // More than just one queen's moves

    // Check that both queens have some moves
    let d4_moves = generated_moves
      .iter()
      .filter(|m| m.from_square() == D4)
      .count();
    let a1_moves = generated_moves
      .iter()
      .filter(|m| m.from_square() == A1)
      .count();

    assert!(d4_moves > 0, "D4 queen should have moves");
    assert!(a1_moves > 0, "A1 queen should have moves");
  }

  #[test]
  fn test_complex_queen_position() {
    // Complex position with mixed piece placement
    let board =
      GameData::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1")
        .unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // This tests a more realistic game position
    // White queen on D1 should have some moves available
    let mut found_queen_moves = false;
    for move_item in &generated_moves {
      if move_item.from_square() == D1 {
        found_queen_moves = true;
        break;
      }
    }
    assert!(
      found_queen_moves || count == 0,
      "Should find moves for queen on D1 or no moves if blocked"
    );
  }

  #[test]
  fn test_queen_x_ray_attacks() {
    // Test that queen stops at first piece it encounters in each direction
    let board = GameData::from_fen("8/7p/2ppp3/2pQp3/2ppp3/8/7p/8 w - - 0 1").unwrap();
    let (moves, count) = generate_queen_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Queen should capture pieces but not move beyond them
    let captures_count = generated_moves.iter().filter(|m| m.is_capture()).count();
    assert!(captures_count > 0, "Should have some captures");

    // Verify that queen doesn't move beyond captured pieces
    let has_invalid_move = generated_moves.iter().any(|m| {
      // Check for moves that would be beyond captures
      (m.from_square() == D5 && m.to_square() == A8) || // Beyond B6 capture
      (m.from_square() == D5 && m.to_square() == H1) // Beyond F3 capture
    });
    assert!(
      !has_invalid_move,
      "Queen should not move beyond captured pieces"
    );
  }
}
