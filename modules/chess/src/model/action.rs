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

// Internally the action will be represented by a 16 bit integer.
// The bit layout is as follows:
// Bits 0-5   : `from_square` (0-63)
// Bits 6-11  : `to_square` (0-63)
// Bit  12    : `is_promotion_flag` (1 if pawn promotion, 0 otherwise)
// Bit  13    : `is_capture_flag` (1 if a capture, 0 otherwise)
// Bits 14-15 : These two bits have a shared purpose:
//                - If `is_promotion_flag` (bit 12) is 1:
//                  These bits encode the `promotion_piece_type`:
//                    - 00: Knight
//                    - 01: Bishop
//                    - 10: Rook
//                    - 11: Queen
//                - If `is_promotion_flag` (bit 12) is 0:
//                  - Bit 14: `is_en_passant_flag` (1 if en passant, 0 otherwise)
//                  - Bit 15: `is_castling_flag` (1 if castling, 0 otherwise)
//
// Note: If `is_promotion_flag` is 1, then `is_en_passant_flag` and `is_castling_flag` are implicitly 0.

use std::fmt::Debug;

use super::pieces::PieceType;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Action(pub u16);

pub const FROM_MASK: u16 = 0b0000_0000_0011_1111;
pub const DEST_MASK: u16 = 0b0000_1111_1100_0000;
pub const PROMOTION_MASK: u16 = 0b0001_0000_0000_0000;
pub const CAPTURE_MASK: u16 = 0b0010_0000_0000_0000;
pub const EN_PASSANT_MASK: u16 = 0b0100_0000_0000_0000;
pub const CASTLING_MASK: u16 = 0b1000_0000_0000_0000;

impl Debug for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let from = self.from_square();
    let to = self.to_square();
    let is_promotion = self.is_promotion();
    let is_capture = self.is_capture();
    let is_en_passant = self.is_en_passant();
    let is_castling = self.is_castling();
    if is_promotion {
      let promo_type = self.get_promotion().unwrap();
      write!(
        f,
        "Action {{ from: {from}, to: {to}, promotion: {promo_type:?} }}"
      )
    } else {
      write!(
        f,
        "Action {{ from: {from}, to: {to}, capture: {is_capture}, en_passant: {is_en_passant}, castling: {is_castling} }}"
      )
    }
  }
}

impl Action {
  pub fn new(from: u8, to: u8) -> Self {
    let mut action = 0u16;
    action |= (from as u16) & FROM_MASK;
    action |= ((to as u16) << 6) & DEST_MASK;
    Self(action)
  }

  pub fn set_promotion(&mut self, piece_type: PieceType) {
    self.0 |= PROMOTION_MASK;
    self.0 |= (piece_type as u16) << 14;
  }

  pub fn set_capture(&mut self) {
    if self.is_promotion() {
      panic!("Cannot set capture flag on a promotion action");
    }
    self.0 |= CAPTURE_MASK;
  }

  pub fn set_en_passant(&mut self) {
    if self.is_promotion() {
      panic!("Cannot set en passant flag on a promotion action");
    }
    self.0 |= EN_PASSANT_MASK;
  }

  pub fn from_square(&self) -> u8 {
    (self.0 & FROM_MASK) as u8
  }

  pub fn to_square(&self) -> u8 {
    ((self.0 & DEST_MASK) >> 6) as u8
  }

  pub fn is_promotion(&self) -> bool {
    (self.0 & PROMOTION_MASK) != 0
  }

  pub fn get_promotion(&self) -> Option<PieceType> {
    if self.is_promotion() {
      let promo_type = (self.0 >> 14) & 0b11;
      match promo_type {
        0 => Some(PieceType::Knight),
        1 => Some(PieceType::Bishop),
        2 => Some(PieceType::Rook),
        3 => Some(PieceType::Queen),
        _ => None,
      }
    } else {
      None
    }
  }

  pub fn is_capture(&self) -> bool {
    (self.0 & CAPTURE_MASK) != 0
  }

  pub fn is_en_passant(&self) -> bool {
    (self.0 & EN_PASSANT_MASK) != 0
  }

  pub fn is_castling(&self) -> bool {
    (self.0 & CASTLING_MASK) != 0
  }
}
