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
  model::{gameboard::GameBoard, piecemove::PieceMove},
  movegen::{bishop::MAX_BISHOP_MOVES, knight::MAX_KNIGHT_MOVES, pawn::MAX_PAWN_MOVES},
};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

pub const MAX_MOVES: usize = MAX_PAWN_MOVES
  + MAX_BISHOP_MOVES
  + MAX_KNIGHT_MOVES
  + rook::MAX_ROOK_MOVES
  + queen::MAX_QUEEN_MOVES
  + king::MAX_KING_MOVES;

/// Helper function to add a move
#[inline]
fn add_move_to_list(
  moves: &mut [PieceMove],
  count: &mut usize,
  size: usize,
  piece_move: PieceMove,
) {
  if *count <= size {
    moves[*count] = piece_move;
    *count += 1;
  } else {
    #[cfg(debug_assertions)]
    panic!("Move array overflow! {size} is too small.");
  }
}

pub fn generate_moves(state: &GameBoard) -> ([PieceMove; MAX_MOVES], usize) {
  let mut moves = [PieceMove::NULL; MAX_MOVES];
  let mut count = 0;

  let (pawn_moves, pawn_count) = pawn::generate_pawn_moves(state);
  for &piece_move in pawn_moves.iter().take(pawn_count) {
    add_move_to_list(&mut moves, &mut count, MAX_MOVES, piece_move);
  }

  let (bishop_moves, bishop_count) = bishop::generate_bishop_moves(state);
  for &piece_move in bishop_moves.iter().take(bishop_count) {
    add_move_to_list(&mut moves, &mut count, MAX_MOVES, piece_move);
  }

  let (knight_moves, knight_count) = knight::generate_knight_moves(state);
  for &piece_move in knight_moves.iter().take(knight_count) {
    add_move_to_list(&mut moves, &mut count, MAX_MOVES, piece_move);
  }

  let (rook_moves, rook_count) = rook::generate_rook_moves(state);
  for &piece_move in rook_moves.iter().take(rook_count) {
    add_move_to_list(&mut moves, &mut count, MAX_MOVES, piece_move);
  }

  let (queen_moves, queen_count) = queen::generate_queen_moves(state);
  for &piece_move in queen_moves.iter().take(queen_count) {
    add_move_to_list(&mut moves, &mut count, MAX_MOVES, piece_move);
  }

  let (king_moves, king_count) = king::generate_king_moves(state);
  for &piece_move in king_moves.iter().take(king_count) {
    add_move_to_list(&mut moves, &mut count, MAX_MOVES, piece_move);
  }

  (moves, count)
}
