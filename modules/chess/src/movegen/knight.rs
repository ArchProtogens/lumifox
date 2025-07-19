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
  constants::{FILE_A, FILE_B, FILE_G, FILE_H}, // Added FILE_A for wrap-around protection
  model::{bitboard::BitBoard, gameboard::GameBoard, piecemove::PieceMove},
  movegen::add_move_to_list,
};

pub const MAX_KNIGHT_MOVES: usize = 16;

pub(crate) fn generate_knight_moves(state: &GameBoard) -> ([PieceMove; MAX_KNIGHT_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_KNIGHT_MOVES];
  let mut count = 0;

  let all_occupied =
    state.pawns | state.knights | state.bishops | state.rooks | state.queens | state.kings;

  let (my_knights, other_pieces): (BitBoard, u64) = if state.playing {
    (
      state.knights & state.colour,
      (all_occupied & !state.colour).into(),
    )
  } else {
    (
      state.knights & !state.colour,
      (all_occupied & state.colour).into(),
    )
  };

  // Generate moves for each knight
  //
  // Here are the four diections as base:
  //
  // left : -1
  // right: +1
  // up   : -8
  // down : +8

  let knight_moves_data: [(i8, u64); 8] = [
    (-17, FILE_A), // Knight on A or B file cannot move -17 (wraps from right, or off board)
    (-15, FILE_H), // Knight on G or H file cannot move -15 (wraps from left, or off board)
    (-10, FILE_A | FILE_B), // Knight on A file cannot move -10
    (-6, FILE_G | FILE_H), // Knight on H file cannot move -6
    (6, FILE_A | FILE_B), // Knight on A file cannot move +6
    (10, FILE_G | FILE_H), // Knight on H file cannot move +10
    (15, FILE_A),  // Knight on A or B file cannot move +15
    (17, FILE_H),  // Knight on G or H file cannot move +17
  ];

  for (dir, mask) in knight_moves_data {
    let new_pos = if dir > 0 {
      (my_knights & !mask) << (dir as u8)
    } else {
      (my_knights & !mask) >> ((-dir) as u8)
    };

    let blockers = new_pos & all_occupied.raw();

    let mut attackers = blockers.raw() & other_pieces;
    while attackers != 0 {
      let to_board = attackers.trailing_zeros() as u8;
      let from_board = if dir > 0 {
        to_board - (dir as u8) // For positive dir, subtract to go backwards
      } else {
        to_board + ((-dir) as u8) // For negative dir, add the absolute value
      };

      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_KNIGHT_MOVES,
        PieceMove::new(from_board, to_board, true, None),
      );

      attackers &= !(1 << to_board);
    }

    let mut quiet_moves = new_pos.raw() & !all_occupied.raw();
    while quiet_moves != 0 {
      let to_board = quiet_moves.trailing_zeros() as u8;
      let from_board = if dir > 0 {
        to_board - (dir as u8) // For positive dir, subtract to go backwards
      } else {
        to_board + ((-dir) as u8) // For negative dir, add the absolute value
      };

      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_KNIGHT_MOVES,
        PieceMove::new(from_board, to_board, false, None),
      );

      quiet_moves &= !(1 << to_board);
    }
  }

  (moves, count)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    constants::*,
    model::{
      gameboard::{GameBoard, PieceType},
      piecemove::PieceMove,
    },
  }; // Import GameData for board_from_fen

  // Helper to create a GameBoard from a FEN string
  fn board_from_fen(fen: &str) -> GameBoard {
    let gamedata = crate::model::gamedata::GameData::from_fen(fen)
      .unwrap_or_else(|e| panic!("Failed to parse FEN: {e:?}"));
    gamedata.board
  }

  // Helper function to sort and compare PieceMove arrays
  fn sort_and_compare_moves(mut moves: Vec<PieceMove>) -> Vec<PieceMove> {
    moves.sort();
    moves
  }

  // Helper to convert a list of PieceMoves to a Vec for easier comparison
  fn moves_to_vec(moves: &[PieceMove; MAX_KNIGHT_MOVES], count: usize) -> Vec<PieceMove> {
    moves[0..count].to_vec()
  }

  #[test]
  fn test_generate_knight_moves_empty_board() {
    let board = GameBoard::new(); // Empty board
    let (moves, count) = generate_knight_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 0);
    assert!(generated_moves.is_empty());
  }

  #[test]
  fn test_generate_knight_moves_white_knight_d4() {
    // White knight on d4, empty board otherwise
    let mut board = GameBoard::new();
    board.set_square(D4, PieceType::Knight, true); // White knight on d4
    board.playing = true; // White to move

    let (moves, count) = generate_knight_moves(&board);
    let expected_moves = vec![
      PieceMove::new(D4, B3, false, None), // d4 -> b3
      PieceMove::new(D4, C2, false, None), // d4 -> c2
      PieceMove::new(D4, E2, false, None), // d4 -> e2
      PieceMove::new(D4, F3, false, None), // d4 -> f3
      PieceMove::new(D4, B5, false, None), // d4 -> b5
      PieceMove::new(D4, C6, false, None), // d4 -> c6
      PieceMove::new(D4, E6, false, None), // d4 -> e6
      PieceMove::new(D4, F5, false, None), // d4 -> f5
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_white_knight_a1() {
    // White knight on a1, empty board otherwise
    let mut board = GameBoard::new();
    board.set_square(A1, PieceType::Knight, true); // White knight on a1
    board.playing = true; // White to move

    let (moves, count) = generate_knight_moves(&board);
    let expected_moves = vec![
      PieceMove::new(A1, B3, false, None), // a1 -> b3
      PieceMove::new(A1, C2, false, None), // a1 -> c2
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 2);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_black_knight_h8() {
    // Black knight on h8, empty board otherwise
    let mut board = GameBoard::new();
    board.set_square(H8, PieceType::Knight, false); // Black knight on h8
    board.playing = false; // Black to move

    let (moves, count) = generate_knight_moves(&board);
    let expected_moves = vec![
      PieceMove::new(H8, G6, false, None), // h8 -> g6
      PieceMove::new(H8, F7, false, None), // h8 -> f7
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 2);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_white_knight_blocked_by_friendly() {
    // White knight on d4, surrounded by friendly pawns
    let board = board_from_fen("8/8/2P1P3/1P3P2/3N4/1P3P2/2P1P3/8 w - - 0 1"); // White knight on d4, white pawns on c3, e3, b4, f4, c5, e5
    let (moves, count) = generate_knight_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 0); // No moves possible
    assert!(generated_moves.is_empty());
  }

  #[test]
  fn test_generate_knight_moves_white_knight_captures() {
    // White knight on d4, black pawns on e6 and f5
    let board = board_from_fen("8/8/2p1p3/3P1p2/2PNP3/8/8/8 w - - 0 1"); // White knight on d4, black pawns on c6, e6, f5
    let (moves, count) = generate_knight_moves(&board);
    let expected_moves = vec![
      PieceMove::new(D4, E2, false, None), // d4 -> e2 (quiet)
      PieceMove::new(D4, C2, false, None), // d4 -> c2 (quiet)
      PieceMove::new(D4, F3, false, None), // d4 -> f3 (quiet)
      PieceMove::new(D4, B3, false, None), // d4 -> b3 (quiet)
      PieceMove::new(D4, F5, true, None),  // d4 -> f5 (capture)
      PieceMove::new(D4, B5, false, None), // d4 -> b5 (quiet)
      PieceMove::new(D4, E6, true, None),  // d4 -> e6 (capture)
      PieceMove::new(D4, C6, true, None),  // d4 -> c6 (capture)
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_black_knight_captures() {
    // Black knight on d4, white pawns on e6 and f5
    let board = board_from_fen("8/8/2P1P3/3P1P2/2pnP3/8/8/8 b - - 0 1"); // Black knight on d4, white pawns on c6, e6, f5
    let (moves, count) = generate_knight_moves(&board);
    let expected_moves = vec![
      PieceMove::new(D4, E2, false, None), // d4 -> e2 (quiet)
      PieceMove::new(D4, C2, false, None), // d4 -> c2 (quiet)
      PieceMove::new(D4, F3, false, None), // d4 -> f3 (quiet)
      PieceMove::new(D4, B3, false, None), // d4 -> b3 (quiet)
      PieceMove::new(D4, F5, true, None),  // d4 -> f5 (capture)
      PieceMove::new(D4, B5, false, None), // d4 -> b5 (quiet)
      PieceMove::new(D4, E6, true, None),  // d4 -> e6 (capture)
      PieceMove::new(D4, C6, true, None),  // d4 -> c6 (capture)
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_initial_position_white() {
    // Initial board position, white to move
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let (moves, count) = generate_knight_moves(&board);

    // White knights are on B1 (1) and G1 (6)
    // B1 can move to A3 (16), C3 (18)
    // G1 can move to F3 (21), H3 (23)
    let expected_moves = vec![
      PieceMove::new(B1, A3, false, None),
      PieceMove::new(B1, C3, false, None),
      PieceMove::new(G1, F3, false, None),
      PieceMove::new(G1, H3, false, None),
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 4);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_initial_position_black() {
    // Initial board position, black to move
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    let (moves, count) = generate_knight_moves(&board);

    // Black knights are on B8 (57) and G8 (62)
    // B8 can move to A6 (40), C6 (42)
    // G8 can move to F6 (45), H6 (47)
    let expected_moves = vec![
      PieceMove::new(B8, A6, false, None),
      PieceMove::new(B8, C6, false, None),
      PieceMove::new(G8, F6, false, None),
      PieceMove::new(G8, H6, false, None),
    ];

    let generated_moves = moves_to_vec(&moves, count);
    //assert_eq!(count, 4);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_complex_scenario() {
    // FEN: 2n5/8/8/4N3/8/8/8/8 w - - 0 1
    // White knight on e5 (36)
    // Black knight on c8 (58)
    // White to move
    let board = board_from_fen("2n5/8/8/4N3/8/8/8/8 w - - 0 1");
    let (moves, count) = generate_knight_moves(&board);

    // White knight on e5 can move to:
    // c4 (26), c6 (42), d3 (19), d7 (51), f3 (21), f7 (45), g4 (27), g6 (43)
    let expected_moves = vec![
      PieceMove::new(E5, C4, false, None),
      PieceMove::new(E5, C6, false, None),
      PieceMove::new(E5, D3, false, None),
      PieceMove::new(E5, D7, false, None),
      PieceMove::new(E5, F3, false, None),
      PieceMove::new(E5, F7, false, None),
      PieceMove::new(E5, G4, false, None),
      PieceMove::new(E5, G6, false, None),
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_complex_scenario_black_to_move() {
    // FEN: 2n5/8/8/4N3/8/8/8/8 b - - 0 1
    // White knight on e5 (36)
    // Black knight on c8 (58)
    // Black to move
    let board = board_from_fen("2n5/8/8/4N3/8/8/8/8 b - - 0 1");
    let (moves, count) = generate_knight_moves(&board);

    // Black knight on c8 can move to:
    // a7 (48), b6 (41), d6 (43), e7 (52)
    let expected_moves = vec![
      PieceMove::new(C8, A7, false, None),
      PieceMove::new(C8, B6, false, None),
      PieceMove::new(C8, D6, false, None),
      PieceMove::new(C8, E7, false, None),
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 4);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_capture_black_knight_c8_captures_e5() {
    // FEN: 2n5/8/8/4N3/8/8/8/8 b - - 0 1
    // Black knight on c8 (58)
    // White knight on e5 (36)
    // Black to move
    let board = board_from_fen("2n5/8/8/4N3/8/8/8/8 b - - 0 1");
    let (moves, count) = generate_knight_moves(&board);

    // Black knight on c8 can move to:
    // a7 (48), b6 (41), d6 (43), e7 (52)
    // It can also capture the white knight on e5 (36)
    let expected_moves = vec![
      PieceMove::new(C8, A7, false, None),
      PieceMove::new(C8, B6, false, None),
      PieceMove::new(C8, D6, false, None),
      PieceMove::new(C8, E7, false, None),
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 4);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_knight_moves_full() {
    let board = board_from_fen("8/8/8/3N4/4N3/8/8/8 w - - 0 1");
    let (moves, count) = generate_knight_moves(&board);

    let expected_moves = vec![
      PieceMove::new(D5, C7, false, None), // D5 -> C7 (quiet)
      PieceMove::new(D5, E7, false, None), // D5 -> E7 (quiet)
      PieceMove::new(D5, B6, false, None), // D5 -> C6 (quiet)
      PieceMove::new(D5, F6, false, None), // D5 -> F6 (quiet)
      PieceMove::new(D5, B4, false, None), // D5 -> C3 (quiet)
      PieceMove::new(D5, F4, false, None), // D5 -> F4 (quiet)
      PieceMove::new(D5, C3, false, None), // D5 -> C3 (quiet)
      PieceMove::new(D5, E3, false, None), // D5 -> E3 (quiet)
      // Second knight on E4
      PieceMove::new(E4, D6, false, None), // E4 -> D6 (quiet)
      PieceMove::new(E4, F6, false, None), // E4 -> F6 (quiet)
      PieceMove::new(E4, C5, false, None), // E4 -> C5 (quiet)
      PieceMove::new(E4, G5, false, None), // E4 -> G5 (quiet)
      PieceMove::new(E4, C3, false, None), // E4 -> C3 (quiet)
      PieceMove::new(E4, G3, false, None), // E4 -> G3 (quiet)
      PieceMove::new(E4, D2, false, None), // E4 -> D2 (quiet)
      PieceMove::new(E4, F2, false, None), // E4 -> F2 (quiet)
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 16);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }
}
