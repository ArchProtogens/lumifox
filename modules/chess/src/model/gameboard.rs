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

use crate::model::piecemove::PieceMove;

use super::bitboard::BitBoard;

#[derive(Clone, Copy, Debug)]
pub struct GameBoard {
  // Boards for each piece type
  pub pawns: BitBoard,
  pub knights: BitBoard,
  pub bishops: BitBoard,
  pub rooks: BitBoard,
  pub queens: BitBoard,
  pub kings: BitBoard,

  // Now for additional metadata
  pub colour: BitBoard, // BitBoard indicating which pieces are white (1) or black (0)
  pub castling: u8,
  pub en_passant: PieceMove,
  pub playing: bool,
}

impl Default for GameBoard {
  fn default() -> Self {
    GameBoard {
      pawns: BitBoard::EMPTY,
      knights: BitBoard::EMPTY,
      bishops: BitBoard::EMPTY,
      rooks: BitBoard::EMPTY,
      queens: BitBoard::EMPTY,
      kings: BitBoard::EMPTY,
      colour: BitBoard::EMPTY,
      castling: 0,
      en_passant: PieceMove::NULL,
      playing: true,
    }
  }
}

impl GameBoard {
  pub fn new() -> Self {
    GameBoard::default()
  }

  pub fn reset(&mut self) {
    *self = GameBoard::new();
  }

  pub fn combined(&self) -> BitBoard {
    self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
  }

  pub fn combined_coloured(&self, desired: bool) -> BitBoard {
    self.combined() & (self.colour ^ desired)
  }
}
