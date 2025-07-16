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

use core::u64;

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
  let mut ray_attackers: u64 = my_bishops.clone().into();
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
  ray_attackers = my_bishops.clone().into();
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
  ray_attackers = my_bishops.clone().into();
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
  ray_attackers = my_bishops.clone().into();
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
