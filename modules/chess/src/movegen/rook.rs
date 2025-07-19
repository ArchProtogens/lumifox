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

pub const MAX_ROOK_MOVES: usize = 28;

pub(crate) fn generate_rook_moves(state: &GameBoard) -> ([PieceMove; MAX_ROOK_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_ROOK_MOVES];
  let mut count = 0;

  let all_occupied =
    state.pawns | state.knights | state.bishops | state.rooks | state.queens | state.kings;

  let (my_rooks, other_pieces): (BitBoard, u64) = if state.playing {
    (
      state.rooks & state.colour,
      (all_occupied & !state.colour).into(),
    )
  } else {
    (
      state.rooks & !state.colour,
      (all_occupied & state.colour).into(),
    )
  };

  // Ray-casting for all 4 directions

  // 1. Top moves (shift by 8)
  let mut ray_attackers: u64 = my_rooks.into();
  for i in 1..8 {
    // We move the rooks up.
    ray_attackers <<= 8;

    // Potential captures are ray attacks that land on an opponent's piece.
    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board - (i * 8);

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
      let from_board = to_board - (i * 8);

      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;

      // Remove this processed move.
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  // 2. Right moves (shift by 1)
  ray_attackers = my_rooks.into();
  for i in 1..8 {
    // We move the rooks right.
    ray_attackers <<= 1;
    // Remove all who warp around to file A.
    ray_attackers &= !FILE_A;

    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board - i;
      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;
      captures &= captures - 1;
    }

    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board - i;
      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  // 3. Bottom moves (shift by -8)
  ray_attackers = my_rooks.into();
  for i in 1..8 {
    // We move the rooks down.
    ray_attackers >>= 8;

    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board + (i * 8);
      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;
      captures &= captures - 1;
    }

    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board + (i * 8);
      moves[count] = PieceMove::new(from_board, to_board, false, None);
      count += 1;
      quiet_moves &= quiet_moves - 1;
    }

    if ray_attackers == 0 {
      break;
    }
  }

  // 4. Left moves (shift by -1)
  ray_attackers = my_rooks.into();
  for i in 1..8 {
    // We move the rooks left and remove all who warp around to file H.
    ray_attackers >>= 1;
    ray_attackers &= !FILE_H;

    let mut captures = ray_attackers & other_pieces;
    while captures != 0 {
      let to_board = captures.trailing_zeros() as u8;
      let from_board = to_board + i;
      moves[count] = PieceMove::new(from_board, to_board, true, None);
      count += 1;
      captures &= captures - 1;
    }

    let blockers = ray_attackers & all_occupied.raw();
    ray_attackers &= !blockers;

    let mut quiet_moves = ray_attackers;
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = to_board + i;
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
    moves.sort();
    moves
  }

  #[test]
  fn test_single_white_rook_center() {
    // White rook on D4 with clear ranks and files
    let board = GameData::from_fen("8/8/8/8/3R4/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
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
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_single_black_rook_center() {
    // Black rook on D4 with clear ranks and files
    let board = GameData::from_fen("8/8/8/8/3r4/8/8/8 b - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
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
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_rook_corner_a1() {
    // White rook on A1 corner
    let board = GameData::from_fen("8/8/8/8/8/8/8/R7 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
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
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_rook_corner_h8() {
    // White rook on H8 corner
    let board = GameData::from_fen("7R/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Down the H file
      PieceMove::new(H8, H7, false, None),
      PieceMove::new(H8, H6, false, None),
      PieceMove::new(H8, H5, false, None),
      PieceMove::new(H8, H4, false, None),
      PieceMove::new(H8, H3, false, None),
      PieceMove::new(H8, H2, false, None),
      PieceMove::new(H8, H1, false, None),
      // Left on 8th rank
      PieceMove::new(H8, G8, false, None),
      PieceMove::new(H8, F8, false, None),
      PieceMove::new(H8, E8, false, None),
      PieceMove::new(H8, D8, false, None),
      PieceMove::new(H8, C8, false, None),
      PieceMove::new(H8, B8, false, None),
      PieceMove::new(H8, A8, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_white_rook_captures() {
    // White rook on D4 with black pieces to capture
    let board = GameData::from_fen("8/8/8/3p4/1p1R1p2/8/3p4/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Up the D file - capture on D5
      PieceMove::new(D4, D5, true, None), // Capture
      // Right on 4th rank - capture on F4
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, true, None), // Capture
      // Down the D file - capture on D2
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, D2, true, None), // Capture
      // Left on 4th rank - capture on B4
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, B4, true, None), // Capture
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_black_rook_captures() {
    // Black rook on D4 with white pieces to capture
    let board = GameData::from_fen("8/8/8/3P4/1P1r1P2/8/3P4/8 b - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Up the D file - capture on D5
      PieceMove::new(D4, D5, true, None), // Capture
      // Right on 4th rank - capture on F4
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, true, None), // Capture
      // Down the D file - capture on D2
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, D2, true, None), // Capture
      // Left on 4th rank - capture on B4
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, B4, true, None), // Capture
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_rook_blocked_by_own_pieces() {
    // White rook on D4 blocked by own pawns
    let board = GameData::from_fen("8/8/8/3P4/1P1R1P2/8/3P4/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Rook should have no moves as all directions are blocked by own pieces
    let expected_moves = vec![
      // Only the squares immediately adjacent to own pieces
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, C4, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_rook_partially_blocked() {
    // White rook on D4 with some directions blocked
    let board = GameData::from_fen("8/8/8/8/3R4/8/1P6/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Up the D file - clear
      PieceMove::new(D4, D5, false, None),
      PieceMove::new(D4, D6, false, None),
      PieceMove::new(D4, D7, false, None),
      PieceMove::new(D4, D8, false, None),
      // Right on 4th rank - clear
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, false, None),
      PieceMove::new(D4, G4, false, None),
      PieceMove::new(D4, H4, false, None),
      // Down the D file - clear
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, D2, false, None),
      PieceMove::new(D4, D1, false, None),
      // Left on 4th rank - clear
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, B4, false, None),
      PieceMove::new(D4, A4, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_multiple_rooks() {
    // Two white rooks on the board
    let board = GameData::from_fen("8/8/8/8/3R4/8/8/R7 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    // Should include moves for both rooks
    // Rook on D4
    let mut d4_moves = vec![
      PieceMove::new(D4, D1, false, None),
      PieceMove::new(D4, D2, false, None),
      PieceMove::new(D4, D3, false, None),
      PieceMove::new(D4, D5, false, None),
      PieceMove::new(D4, D6, false, None),
      PieceMove::new(D4, D7, false, None),
      PieceMove::new(D4, D8, false, None),
      PieceMove::new(D4, A4, false, None),
      PieceMove::new(D4, B4, false, None),
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, false, None),
      PieceMove::new(D4, G4, false, None),
      PieceMove::new(D4, H4, false, None),
    ];

    // Rook on A1
    let mut a1_moves = vec![
      PieceMove::new(A1, A2, false, None),
      PieceMove::new(A1, A3, false, None),
      PieceMove::new(A1, A4, false, None),
      PieceMove::new(A1, A5, false, None),
      PieceMove::new(A1, A6, false, None),
      PieceMove::new(A1, A7, false, None),
      PieceMove::new(A1, A8, false, None),
      PieceMove::new(A1, B1, false, None),
      PieceMove::new(A1, C1, false, None),
      PieceMove::new(A1, D1, false, None),
      PieceMove::new(A1, E1, false, None),
      PieceMove::new(A1, F1, false, None),
      PieceMove::new(A1, G1, false, None),
      PieceMove::new(A1, H1, false, None),
    ];

    let mut expected_moves = Vec::new();
    expected_moves.append(&mut d4_moves);
    expected_moves.append(&mut a1_moves);

    assert!(expected_moves.len() == MAX_ROOK_MOVES);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_rook_edge_cases() {
    // Rook on edge of board
    let board = GameData::from_fen("8/8/8/8/R7/8/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Up the A file
      PieceMove::new(A4, A5, false, None),
      PieceMove::new(A4, A6, false, None),
      PieceMove::new(A4, A7, false, None),
      PieceMove::new(A4, A8, false, None),
      // Right on 4th rank
      PieceMove::new(A4, B4, false, None),
      PieceMove::new(A4, C4, false, None),
      PieceMove::new(A4, D4, false, None),
      PieceMove::new(A4, E4, false, None),
      PieceMove::new(A4, F4, false, None),
      PieceMove::new(A4, G4, false, None),
      PieceMove::new(A4, H4, false, None),
      // Down the A file
      PieceMove::new(A4, A3, false, None),
      PieceMove::new(A4, A2, false, None),
      PieceMove::new(A4, A1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_no_rooks() {
    // No rooks on the board
    let board = GameData::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let (_moves, count) = generate_rook_moves(&board.board);

    assert_eq!(count, 0);
  }

  #[test]
  fn test_rook_initial_position() {
    // Standard chess starting position rooks
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (_moves, count) = generate_rook_moves(&board.board);

    // Rooks should have no moves in starting position due to pawns and pieces blocking
    assert_eq!(count, 0);
  }

  #[test]
  fn test_rook_after_pawn_moves() {
    // Position after some pawn moves to open up rook files
    let board =
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // White rook on H1 can now move up the H file
      PieceMove::new(H1, H2, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_complex_rook_position() {
    // Complex position with mixed piece placement
    let board = GameData::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RN2K1NR w KQkq - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    drop(generated_moves); // Drop to avoid unused variable warning

    // In this position, rooks should still have no moves due to blocking pawns
    assert_eq!(count, 0);
  }

  #[test]
  fn test_rook_open_files() {
    // Rook on open file
    let board = GameData::from_fen("8/8/8/8/8/8/8/3R4 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Up the D file
      PieceMove::new(D1, D2, false, None),
      PieceMove::new(D1, D3, false, None),
      PieceMove::new(D1, D4, false, None),
      PieceMove::new(D1, D5, false, None),
      PieceMove::new(D1, D6, false, None),
      PieceMove::new(D1, D7, false, None),
      PieceMove::new(D1, D8, false, None),
      // Right on 1st rank
      PieceMove::new(D1, E1, false, None),
      PieceMove::new(D1, F1, false, None),
      PieceMove::new(D1, G1, false, None),
      PieceMove::new(D1, H1, false, None),
      // Left on 1st rank
      PieceMove::new(D1, C1, false, None),
      PieceMove::new(D1, B1, false, None),
      PieceMove::new(D1, A1, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_rook_x_ray_attacks() {
    // Test that rook stops at first piece it encounters
    let board = GameData::from_fen("8/8/8/3p4/3R4/3p4/8/8 w - - 0 1").unwrap();
    let (moves, count) = generate_rook_moves(&board.board);
    let generated_moves: Vec<PieceMove> = moves[..count].to_vec();

    let expected_moves = vec![
      // Up the D file - capture on D5, can't go further
      PieceMove::new(D4, D5, true, None), // Capture, stops here
      // Right on 4th rank - clear
      PieceMove::new(D4, E4, false, None),
      PieceMove::new(D4, F4, false, None),
      PieceMove::new(D4, G4, false, None),
      PieceMove::new(D4, H4, false, None),
      // Down the D file - capture on D3, can't go further
      PieceMove::new(D4, D3, true, None), // Capture, stops here
      // Left on 4th rank - clear
      PieceMove::new(D4, C4, false, None),
      PieceMove::new(D4, B4, false, None),
      PieceMove::new(D4, A4, false, None),
    ];

    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }
}
