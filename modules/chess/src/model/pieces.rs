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

// Internally the pieces will be represented by an 8 bit integer,
// with the following bit layout:
// bit  0–2  : piece type (0=empty, 1=pawn, …, 6=king)
// bit  3    : color (0=white, 1=black)
// bit  4    : has_moved (for castling rights)
// bit  5    : is_promoted (for clarity, if you want)
// bits 6–7  : reserved for future use

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

pub const EMPTY: u8 = 0; // Represents a completely empty square
pub const WHITE: u8 = 0; // Bit 3 for color (0 for white)

// Define flags based on the documented bit layout
const PIECE_TYPE_MASK: u8 = 0b0000_0111; // Bits 0-2 for piece type
pub const BLACK: u8 = 0b0000_1000; // Bit 3 for color (1 for black, 0 for white)
pub const MOVED: u8 = 0b0001_0000; // Bit 4 for has_moved
pub const PROMO: u8 = 0b0010_0000; // Bit 5 for is_promoted

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
  Empty = 0, // Matches EMPTY if no other flags are set
  Pawn = 1,
  Knight = 2,
  Bishop = 3,
  Rook = 4,
  Queen = 5,
  King = 6,
}

impl fmt::Debug for PieceType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = match self {
      PieceType::Empty => "Empty",
      PieceType::Pawn => "Pawn",
      PieceType::Knight => "Knight",
      PieceType::Bishop => "Bishop",
      PieceType::Rook => "Rook",
      PieceType::Queen => "Queen",
      PieceType::King => "King",
    };
    write!(f, "PieceType({name})")
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub u8);

impl fmt::Debug for Piece {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_empty_square() {
      return write!(f, "Piece(Empty)");
    }
    let piece_type = self.piece_type();
    let color = if self.is_black() { "Black" } else { "White" };
    let mut flags = Vec::new();
    if self.has_moved() {
      flags.push("moved");
    }
    if self.is_promoted() {
      flags.push("promoted");
    }
    write!(
      f,
      "Piece({} {:?}{})",
      color,
      piece_type,
      if flags.is_empty() {
        "".to_string()
      } else {
        format!(" ({})", flags.join(", "))
      }
    )
  }
}

impl BitAnd for Piece {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self::Output {
    Piece(self.0 & rhs.0)
  }
}

impl BitAndAssign for Piece {
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 &= rhs.0;
  }
}

impl BitOr for Piece {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    Piece(self.0 | rhs.0)
  }
}

impl BitOrAssign for Piece {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

impl Piece {
  pub fn new(piece_type: PieceType, color_flag: u8) -> Self {
    // color_flag should be Piece::BLACK or Piece::WHITE
    Piece(piece_type as u8 | color_flag)
  }

  pub fn empty() -> Self {
    Piece(EMPTY)
  }

  pub fn is_empty_square(&self) -> bool {
    // Checks if the square is completely empty (no piece, no flags)
    self.0 == EMPTY
  }

  pub fn piece_type(&self) -> PieceType {
    let type_val = self.0 & PIECE_TYPE_MASK;
    match type_val {
      0 => PieceType::Empty,
      1 => PieceType::Pawn,
      2 => PieceType::Knight,
      3 => PieceType::Bishop,
      4 => PieceType::Rook,
      5 => PieceType::Queen,
      6 => PieceType::King,
      _ => panic!("Invalid piece type value: {type_val}"), // Should not happen with valid pieces
    }
  }

  pub fn is_black(&self) -> bool {
    (self.0 & BLACK) != 0
  }

  pub fn has_moved(&self) -> bool {
    (self.0 & MOVED) != 0
  }

  pub fn is_promoted(&self) -> bool {
    (self.0 & PROMO) != 0
  }

  pub fn promote(&mut self, new_piece_type: PieceType) {
    // Ensure the new type is not Empty or King for promotion
    if new_piece_type == PieceType::Empty || new_piece_type == PieceType::King {
      // Or handle error appropriately
      panic!("Invalid promotion piece type");
    }
    // Clear current piece type bits and set new piece type and promotion flag
    self.0 = (self.0 & !PIECE_TYPE_MASK) | (new_piece_type as u8) | PROMO;
  }

  pub fn set_moved(&mut self) {
    self.0 |= MOVED;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_piece() {
    let p = Piece::empty();
    assert!(p.is_empty_square());
    assert_eq!(p.piece_type(), PieceType::Empty);
    assert!(!p.is_black());
    assert!(!p.has_moved());
    assert!(!p.is_promoted());
  }

  #[test]
  fn test_piece_new_and_properties() {
    let white_pawn = Piece::new(PieceType::Pawn, WHITE);
    assert!(!white_pawn.is_empty_square());
    assert_eq!(white_pawn.piece_type(), PieceType::Pawn);
    assert!(!white_pawn.is_black());

    let black_knight = Piece::new(PieceType::Knight, BLACK);
    assert_eq!(black_knight.piece_type(), PieceType::Knight);
    assert!(black_knight.is_black());
  }

  #[test]
  fn test_set_moved() {
    let mut p = Piece::new(PieceType::Rook, WHITE);
    assert!(!p.has_moved());
    p.set_moved();
    assert!(p.has_moved());
  }

  #[test]
  fn test_promote() {
    let mut p = Piece::new(PieceType::Pawn, WHITE);
    p.promote(PieceType::Queen);
    assert_eq!(p.piece_type(), PieceType::Queen);
    assert!(p.is_promoted());
  }

  #[test]
  #[should_panic]
  fn test_invalid_promotion() {
    let mut p = Piece::new(PieceType::Pawn, WHITE);
    p.promote(PieceType::Empty);
  }

  #[test]
  fn test_bit_ops() {
    let p1 = Piece::new(PieceType::Bishop, BLACK);
    let p2 = Piece(MOVED);
    let combined = p1 | p2;
    assert_eq!(combined.piece_type(), PieceType::Bishop);
    assert!(combined.is_black());
    assert!(combined.has_moved());

    let masked = combined & Piece(!(MOVED));
    assert!(!masked.has_moved());
  }

  #[test]
  fn test_debug_format() {
    let mut p = Piece::new(PieceType::Queen, BLACK);
    p.set_moved();
    let s = format!("{:?}", p);
    assert!(s.contains("Black"));
    assert!(s.contains("Queen"));
    assert!(s.contains("moved"));
  }
}
