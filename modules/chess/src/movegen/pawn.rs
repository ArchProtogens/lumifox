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

const MAX_PAWN_MOVES: usize = 12;
const FILE_A: u64 = 0x0101_0101_0101_0101;
const FILE_H: u64 = 0x8080_8080_8080_8080;
const RANK_1: u64 = 0x0000_0000_00FF_00FF;
const RANK_2: u64 = 0x0000_0000_0000_FF00;
const RANK_3: u64 = 0x0000_0000_00FF_0000;
const RANK_4: u64 = 0x0000_0000_FF00_0000;
const RANK_5: u64 = 0x0000_00FF_0000_0000;
const RANK_6: u64 = 0x0000_FF00_0000_0000;
const RANK_7: u64 = 0x00FF_0000_0000_0000;
const RANK_8: u64 = 0xFF00_0000_0000_0000;

use crate::model::{gameboard::GameBoard, piecemove::PieceMove};

pub(crate) fn generate_pawn_moves(state: &GameBoard) -> ([PieceMove; MAX_PAWN_MOVES], usize) {
  let mut moves: [PieceMove; MAX_PAWN_MOVES] = Default::default();
  let mut count = 0;

  if (state.playing) {
    // Generate moves for white pawns
    let white_pawns = state.pawns & state.colour;
  }

  todo!()
}
