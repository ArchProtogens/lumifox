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
  movegen::add_move_to_list,
};

pub const MAX_KING_MOVES: usize = 8;

pub(crate) fn generate_king_moves(state: &GameBoard) -> ([PieceMove; MAX_KING_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_KING_MOVES];
  let mut count = 0;

  let all_occupied =
    state.pawns | state.knights | state.bishops | state.rooks | state.queens | state.kings;

  let (my_king, other_pieces): (BitBoard, u64) = if state.playing {
    (
      state.kings & state.colour,
      (all_occupied & !state.colour).into(),
    )
  } else {
    (
      state.kings & !state.colour,
      (all_occupied & state.colour).into(),
    )
  };

  // Possible king moves the king may make
  let king_move_data: [(i8, Option<u64>); 8] = [
    (-8, None),         // up
    (-7, Some(FILE_H)), // up-right
    (1, Some(FILE_H)),  // right
    (9, Some(FILE_H)),  // down-right
    (8, None),          // down
    (7, Some(FILE_A)),  // down-left
    (-1, Some(FILE_A)), // left
    (-9, Some(FILE_A)), // up-left
  ];

  for (dir, mask) in king_move_data {
    let new_pos = if dir > 0 {
      (my_king & !mask.unwrap_or(u64::MIN)) << (dir as u8)
    } else {
      (my_king & !mask.unwrap_or(u64::MIN)) >> ((-dir) as u8)
    };

    let blockers = new_pos.raw() & all_occupied.raw();

    let mut attackers = blockers & other_pieces;
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
        MAX_KING_MOVES,
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
        MAX_KING_MOVES,
        PieceMove::new(from_board, to_board, false, None),
      );

      quiet_moves &= !(1 << to_board);
    }
  }

  // Check for castling moves
  let (queen_side, king_side) = if state.playing {
    state.casling_right_white()
  } else {
    state.casling_right_black()
  };

  // Get my rooks (same color as the king)
  let my_rooks = if state.playing {
    state.rooks & state.colour
  } else {
    state.rooks & !state.colour
  };

  if queen_side {
    // Queenside castling
    let (king_pos, rook_pos, empty_squares) = if state.playing {
      // White queenside: King from E1 to C1, Rook from A1 to D1
      (
        crate::constants::E1,
        crate::constants::A1,
        (1u64 << crate::constants::B1)
          | (1u64 << crate::constants::C1)
          | (1u64 << crate::constants::D1),
      )
    } else {
      // Black queenside: King from E8 to C8, Rook from A8 to D8
      (
        crate::constants::E8,
        crate::constants::A8,
        (1u64 << crate::constants::B8)
          | (1u64 << crate::constants::C8)
          | (1u64 << crate::constants::D8),
      )
    };

    // Check if rook is in correct position and path is clear
    if my_rooks.get_bit(rook_pos).unwrap_or(false) && (all_occupied.raw() & empty_squares) == 0 {
      let king_to = if state.playing {
        crate::constants::C1
      } else {
        crate::constants::C8
      };
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_KING_MOVES,
        PieceMove::new_castling(king_pos, king_to),
      );
    }
  }

  if king_side {
    // Kingside castling
    let (king_pos, rook_pos, empty_squares) = if state.playing {
      // White kingside: King from E1 to G1, Rook from H1 to F1
      (
        crate::constants::E1,
        crate::constants::H1,
        (1u64 << crate::constants::F1) | (1u64 << crate::constants::G1),
      )
    } else {
      // Black kingside: King from E8 to G8, Rook from H8 to F8
      (
        crate::constants::E8,
        crate::constants::H8,
        (1u64 << crate::constants::F8) | (1u64 << crate::constants::G8),
      )
    };

    // Check if rook is in correct position and path is clear
    if my_rooks.get_bit(rook_pos).unwrap_or(false) && (all_occupied.raw() & empty_squares) == 0 {
      let king_to = if state.playing {
        crate::constants::G1
      } else {
        crate::constants::G8
      };
      add_move_to_list(
        &mut moves,
        &mut count,
        MAX_KING_MOVES,
        PieceMove::new_castling(king_pos, king_to),
      );
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
      gamedata::GameData,
      piecemove::PieceMove,
    },
  };

  // Helper function to sort and compare PieceMove arrays
  fn sort_and_compare_moves(mut moves: Vec<PieceMove>) -> Vec<PieceMove> {
    moves.sort();
    moves
  }

  // Helper to convert a list of PieceMoves to a Vec for easier comparison
  fn moves_to_vec(moves: &[PieceMove; MAX_KING_MOVES], count: usize) -> Vec<PieceMove> {
    moves[0..count].to_vec()
  }

  // Helper to create a GameBoard from a FEN string
  fn board_from_fen(fen: &str) -> GameBoard {
    let gamedata = GameData::from_fen(fen).unwrap_or_else(|e| panic!("Failed to parse FEN: {e:?}"));
    gamedata.board
  }

  #[test]
  fn test_generate_king_moves_empty_board() {
    let board = GameBoard::new(); // Empty board
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 0);
    assert!(generated_moves.is_empty());
  }

  #[test]
  fn test_generate_king_moves_white_king_center() {
    // White king on D4, empty board otherwise
    let mut board = GameBoard::new();
    board.set_square(D4, PieceType::King, true); // White king on d4
    board.playing = true; // White to move

    let (moves, count) = generate_king_moves(&board);
    let expected_moves = vec![
      PieceMove::new(D4, C3, false, None), // d4 -> c3
      PieceMove::new(D4, C4, false, None), // d4 -> c4
      PieceMove::new(D4, C5, false, None), // d4 -> c5
      PieceMove::new(D4, D3, false, None), // d4 -> d3
      PieceMove::new(D4, D5, false, None), // d4 -> d5
      PieceMove::new(D4, E3, false, None), // d4 -> e3
      PieceMove::new(D4, E4, false, None), // d4 -> e4
      PieceMove::new(D4, E5, false, None), // d4 -> e5
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_white_king_corner_a1() {
    // White king on A1 corner
    let mut board = GameBoard::new();
    board.set_square(A1, PieceType::King, true); // White king on a1
    board.playing = true; // White to move

    let (moves, count) = generate_king_moves(&board);
    let expected_moves = vec![
      PieceMove::new(A1, A2, false, None), // a1 -> a2
      PieceMove::new(A1, B1, false, None), // a1 -> b1
      PieceMove::new(A1, B2, false, None), // a1 -> b2
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 3);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_black_king_corner_h8() {
    // Black king on H8 corner
    let mut board = GameBoard::new();
    board.set_square(H8, PieceType::King, false); // Black king on h8
    board.playing = false; // Black to move

    let (moves, count) = generate_king_moves(&board);
    let expected_moves = vec![
      PieceMove::new(H8, G7, false, None), // h8 -> g7
      PieceMove::new(H8, G8, false, None), // h8 -> g8
      PieceMove::new(H8, H7, false, None), // h8 -> h7
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 3);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_white_king_blocked_by_friendly() {
    // White king on D4, surrounded by friendly pieces
    let board = board_from_fen("8/8/8/2PPP3/2PKP3/2PPP3/8/8 w - - 0 1"); // White king on d4, white pawns surrounding
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 0); // No moves possible
    assert!(generated_moves.is_empty());
  }

  #[test]
  fn test_generate_king_moves_white_king_captures() {
    // White king on D4, black pieces around
    let board = board_from_fen("8/8/8/2ppp3/2pKp3/2ppp3/8/8 w - - 0 1"); // White king on d4, black pawns surrounding
    let (moves, count) = generate_king_moves(&board);
    let expected_moves = vec![
      PieceMove::new(D4, C3, true, None), // d4 -> c3 (capture)
      PieceMove::new(D4, C4, true, None), // d4 -> c4 (capture)
      PieceMove::new(D4, C5, true, None), // d4 -> c5 (capture)
      PieceMove::new(D4, D3, true, None), // d4 -> d3 (capture)
      PieceMove::new(D4, D5, true, None), // d4 -> d5 (capture)
      PieceMove::new(D4, E3, true, None), // d4 -> e3 (capture)
      PieceMove::new(D4, E4, true, None), // d4 -> e4 (capture)
      PieceMove::new(D4, E5, true, None), // d4 -> e5 (capture)
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_black_king_captures() {
    // Black king on D4, white pieces around
    let board = board_from_fen("8/8/8/2PPP3/2PkP3/2PPP3/8/8 b - - 0 1"); // Black king on d4, white pawns surrounding
    let (moves, count) = generate_king_moves(&board);
    let expected_moves = vec![
      PieceMove::new(D4, C3, true, None), // d4 -> c3 (capture)
      PieceMove::new(D4, C4, true, None), // d4 -> c4 (capture)
      PieceMove::new(D4, C5, true, None), // d4 -> c5 (capture)
      PieceMove::new(D4, D3, true, None), // d4 -> d3 (capture)
      PieceMove::new(D4, D5, true, None), // d4 -> d5 (capture)
      PieceMove::new(D4, E3, true, None), // d4 -> e3 (capture)
      PieceMove::new(D4, E4, true, None), // d4 -> e4 (capture)
      PieceMove::new(D4, E5, true, None), // d4 -> e5 (capture)
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_initial_position_white() {
    // Initial board position, white to move
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // King should have no moves in starting position
    assert_eq!(count, 0);
    assert!(generated_moves.is_empty());
  }

  #[test]
  fn test_generate_king_moves_initial_position_black() {
    // Initial board position, black to move
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // King should have no moves in starting position
    assert_eq!(count, 0);
    assert!(generated_moves.is_empty());
  }

  #[test]
  fn test_generate_king_moves_white_kingside_castling() {
    // Position where white can castle kingside
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // Should include kingside castling move
    let has_castling = generated_moves
      .iter()
      .any(|m| m.from_square() == E1 && m.to_square() == G1);
    assert!(has_castling, "Should be able to castle kingside");
    assert!(count > 0);
  }

  #[test]
  fn test_generate_king_moves_white_queenside_castling() {
    // Position where white can castle queenside
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // Should include queenside castling move
    let has_castling = generated_moves
      .iter()
      .any(|m| m.from_square() == E1 && m.to_square() == C1);
    assert!(has_castling, "Should be able to castle queenside");
    assert!(count > 0);
  }

  #[test]
  fn test_generate_king_moves_black_kingside_castling() {
    // Position where black can castle kingside
    let board = board_from_fen("rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // Should include kingside castling move
    let has_castling = generated_moves
      .iter()
      .any(|m| m.from_square() == E8 && m.to_square() == G8);
    assert!(has_castling, "Should be able to castle kingside");
    assert!(count > 0);
  }

  #[test]
  fn test_generate_king_moves_black_queenside_castling() {
    // Position where black can castle queenside
    let board = board_from_fen("r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // Should include queenside castling move
    let has_castling = generated_moves
      .iter()
      .any(|m| m.from_square() == E8 && m.to_square() == C8);
    assert!(has_castling, "Should be able to castle queenside");
    assert!(count > 0);
  }

  #[test]
  fn test_generate_king_moves_castling_blocked() {
    // Position where castling is blocked by pieces
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // Should not include any castling moves since pieces are blocking
    let has_kingside_castling = generated_moves
      .iter()
      .any(|m| m.from_square() == E1 && m.to_square() == G1);
    let has_queenside_castling = generated_moves
      .iter()
      .any(|m| m.from_square() == E1 && m.to_square() == C1);

    assert!(
      !has_kingside_castling,
      "Should not be able to castle kingside when blocked"
    );
    assert!(
      !has_queenside_castling,
      "Should not be able to castle queenside when blocked"
    );
  }

  #[test]
  fn test_generate_king_moves_no_castling_rights() {
    // Position where king has no castling rights
    let board = board_from_fen("rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w - - 0 1");
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // Should not include any castling moves
    let has_castling = generated_moves.iter().any(|m| {
      (m.from_square() == E1 && m.to_square() == G1)
        || (m.from_square() == E1 && m.to_square() == C1)
    });
    assert!(!has_castling, "Should not be able to castle without rights");
  }

  #[test]
  fn test_generate_king_moves_mixed_scenario() {
    // King with some moves available and some blocked
    let board = board_from_fen("8/8/8/8/2pK4/3P4/8/8 w - - 0 1"); // White king on d4, white pawn on d3, black pawn on c4
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    let expected_moves = vec![
      PieceMove::new(D4, C3, false, None), // d4 -> c3
      PieceMove::new(D4, C4, true, None),  // d4 -> c4 (capture)
      PieceMove::new(D4, C5, false, None), // d4 -> c5
      PieceMove::new(D4, D5, false, None), // d4 -> d5
      PieceMove::new(D4, E3, false, None), // d4 -> e3
      PieceMove::new(D4, E4, false, None), // d4 -> e4
      PieceMove::new(D4, E5, false, None), // d4 -> e5
                                           // Note: d4 -> d3 is blocked by own pawn
    ];

    assert_eq!(count, 7);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_edge_of_board() {
    // King on edge of board
    let mut board = GameBoard::new();
    board.set_square(A4, PieceType::King, true); // White king on a4 (left edge)
    board.playing = true; // White to move

    let (moves, count) = generate_king_moves(&board);
    let expected_moves = vec![
      PieceMove::new(A4, A3, false, None), // a4 -> a3
      PieceMove::new(A4, A5, false, None), // a4 -> a5
      PieceMove::new(A4, B3, false, None), // a4 -> b3
      PieceMove::new(A4, B4, false, None), // a4 -> b4
      PieceMove::new(A4, B5, false, None), // a4 -> b5
    ];

    let generated_moves = moves_to_vec(&moves, count);
    assert_eq!(count, 5);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }

  #[test]
  fn test_generate_king_moves_complex_scenario() {
    // Complex position with mixed pieces
    let board = board_from_fen("8/8/8/3k4/2pKp3/3r4/8/8 w - - 0 1"); // White king on d4, black king on d5, black rook on d3, black pawns on c4 and e4
    let (moves, count) = generate_king_moves(&board);
    let generated_moves = moves_to_vec(&moves, count);

    // King should be able to capture pawns and move to available squares
    let expected_moves = vec![
      PieceMove::new(D4, C5, false, None), // d4 -> c5
      PieceMove::new(D4, D5, true, None),  // d4 -> d5 (capture black king)
      PieceMove::new(D4, E5, false, None), // d4 -> e5
      PieceMove::new(D4, C4, true, None),  // d4 -> c4 (capture black pawn)
      PieceMove::new(D4, E4, true, None),  // d4 -> e4 (capture black pawn)
      PieceMove::new(D4, C3, false, None), // d4 -> c3
      PieceMove::new(D4, D3, true, None),  // d4 -> d3 (capture black rook)
      PieceMove::new(D4, E3, false, None), // d4 -> e3
    ];

    assert_eq!(count, 8);
    assert_eq!(
      sort_and_compare_moves(generated_moves),
      sort_and_compare_moves(expected_moves)
    );
  }
}
