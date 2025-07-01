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

pub struct PieceMove(u16);

// Bit 0-5 - From Square
// Bit 6-11 - To Square
// Bit 12-15 - Metadata (e.g. promotion, capture, en passant, castling)

pub enum MoveMetadata {
  None = 0,
  PromotionQueen = 1,
  PromotionRook = 2,
  PromotionBishop = 3,
  PromotionKnight = 4,
  CastlingKingside = 5,
  CastlingQueenside = 6,
  EnPassant = 7,
  TwoSquareAdvance = 8,
  Capture = 9,
}

impl PieceMove {
  pub const NULL: PieceMove = PieceMove(0);

  pub fn new(from: u8, to: u8, metadata: MoveMetadata) -> Self {
    assert!(
      from < 64 && to < 64,
      "Square indices must be between 0 and 63"
    );
    assert!(from != to, "From and to squares must be different");

    let meta = metadata as u16;

    assert!(meta <= 9, "Metadata must be a valid MoveMetadata variant");

    let move_value = (from as u16) | ((to as u16) << 6) | ((meta) << 12);

    PieceMove(move_value)
  }

  pub fn from_square(&self) -> u8 {
    (self.0 & 0x3F) as u8
  }

  pub fn to_square(&self) -> u8 {
    ((self.0 >> 6) & 0x3F) as u8
  }

  pub fn metadata(&self) -> MoveMetadata {
    match (self.0 >> 12) & 0x0F {
      0 => MoveMetadata::None,
      1 => MoveMetadata::PromotionQueen,
      2 => MoveMetadata::PromotionRook,
      3 => MoveMetadata::PromotionBishop,
      4 => MoveMetadata::PromotionKnight,
      5 => MoveMetadata::CastlingKingside,
      6 => MoveMetadata::CastlingQueenside,
      7 => MoveMetadata::EnPassant,
      8 => MoveMetadata::TwoSquareAdvance,
      9 => MoveMetadata::Capture,
      _ => panic!("Invalid metadata value"),
    }
  }
}

impl Default for PieceMove {
  fn default() -> Self {
    PieceMove::NULL
  }
}
