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
use std::ops::{BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
  pub fn new() -> Self {
    BitBoard(0)
  }

  pub fn set(&mut self, square: u64) {
    self.0 |= 1 << square;
  }

  pub fn clear(&mut self, square: u64) {
    self.0 &= !(1 << square);
  }

  pub fn toggle(&mut self, square: u64) {
    self.0 ^= 1 << square;
  }

  pub fn is_set(&self, square: u64) -> bool {
    (self.0 & (1 << square)) != 0
  }
}

impl BitOr for BitBoard {
  type Output = Self;

  #[inline]
  fn bitor(self, rhs: Self) -> Self::Output {
    BitBoard(self.0 | rhs.0)
  }
}

impl BitOrAssign for BitBoard {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bitboard_set() {
    let mut board = BitBoard::new();
    board.set(3);
    assert_eq!(board.0, 1 << 3);
  }

  #[test]
  fn test_bitboard_clear() {
    let mut board = BitBoard::new();
    board.set(3);
    board.clear(3);
    assert_eq!(board.0, 0);
  }

  #[test]
  fn test_bitboard_toggle() {
    let mut board = BitBoard::new();
    board.set(3);
    board.toggle(3);
    assert_eq!(board.0, 0);
    board.toggle(3);
    assert_eq!(board.0, 1 << 3);
  }

  #[test]
  fn test_bitboard_is_set() {
    let mut board = BitBoard::new();
    board.set(3);
    assert!(board.is_set(3));
    assert!(!board.is_set(4));
  }
}
