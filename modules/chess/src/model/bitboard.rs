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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use core::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

#[derive(Clone, Copy, Debug)]
pub struct BitBoard {
  data: u64,
}

impl BitBoard {
  /// Create a new bitboard
  pub fn new(data: u64) -> Self {
    Self { data }
  }

  /// Get the raw bits value
  pub fn raw(&self) -> u64 {
    self.data
  }

  pub fn set_bit(&mut self, index: u8) {
    if index < 64 {
      self.data |= 1 << index;
    } else {
      panic!("Index out of bounds: {}", index);
    }
  }

  pub fn unset_bit(&mut self, index: u8) {
    if index < 64 {
      self.data &= !(1 << index);
    } else {
      panic!("Index out of bounds: {}", index);
    }
  }

  pub fn get_bit(&self, index: u8) -> bool {
    if index < 64 {
      (self.data & (1 << index)) != 0
    } else {
      panic!("Index out of bounds: {}", index);
    }
  }

  pub const EMPTY: Self = Self { data: 0 };
  pub const ALL_SQUARES: Self = Self { data: u64::MAX };
}

impl BitOr for BitBoard {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    Self::new(self.data | rhs.data)
  }
}

impl BitAnd for BitBoard {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self::Output {
    Self::new(self.data & rhs.data)
  }
}

impl BitAnd<u64> for BitBoard {
  type Output = Self;
  fn bitand(self, rhs: u64) -> Self::Output {
    Self::new(self.data & rhs)
  }
}

impl BitXor<bool> for BitBoard {
  type Output = Self;
  fn bitxor(self, rhs: bool) -> Self::Output {
    if rhs {
      Self::new(self.data ^ u64::MAX)
    } else {
      self
    }
  }
}

impl Not for BitBoard {
  type Output = Self;

  fn not(self) -> Self::Output {
    let inverted = !self.data;
    Self::new(inverted)
  }
}

impl Shl<u8> for BitBoard {
  type Output = Self;
  fn shl(self, rhs: u8) -> Self::Output {
    if rhs > 63 {
      panic!("Shift amount out of bounds: {}", rhs);
    }
    Self::new(self.data << rhs)
  }
}

impl Shr<u8> for BitBoard {
  type Output = Self;

  fn shr(self, rhs: u8) -> Self::Output {
    if rhs > 63 {
      panic!("Shift amount out of bounds: {}", rhs);
    }
    Self::new(self.data >> rhs)
  }
}

impl From<BitBoard> for u64 {
  fn from(val: BitBoard) -> Self {
    val.data
  }
}

pub struct BitBoardIter {
  data: u64,
}

impl Iterator for BitBoardIter {
  type Item = u8; // Represents the square index (0-63)

  fn next(&mut self) -> Option<Self::Item> {
    if self.data == 0 {
      None
    } else {
      let square = self.data.trailing_zeros() as u8;
      self.data &= self.data - 1; // Clear the least significant bit
      Some(square)
    }
  }
}

impl IntoIterator for BitBoard {
  type Item = u8;
  type IntoIter = BitBoardIter;

  fn into_iter(self) -> Self::IntoIter {
    BitBoardIter { data: self.data }
  }
}

/// Directions for shifting bitboards on the 8×8 board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Up = -8,
  Down = 8,
  Left = -1,
  Right = 1,
  UpLeft = -9,
  UpRight = -7,
  DownLeft = 7,
  DownRight = 9,
}

impl From<Direction> for i8 {
  fn from(val: Direction) -> Self {
    match val {
      Direction::Up => -8,
      Direction::Down => 8,
      Direction::Left => -1,
      Direction::Right => 1,
      Direction::UpLeft => -9,
      Direction::UpRight => -7,
      Direction::DownLeft => 7,
      Direction::DownRight => 9,
    }
  }
}
