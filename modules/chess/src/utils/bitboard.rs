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

use crate::model::{bitboard::BitBoard, board::Board};

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

pub trait RayCast {
  fn ray_cast(&self, from: u8, shift: Direction) -> Vec<u8>;
  fn ray_attack(&self, from: u8, shift: Direction) -> u8;
}

impl RayCast for Board {
  fn ray_cast(&self, from: u8, shift: Direction) -> Vec<u8> {
    let mut actions = Vec::new();
    let mut current = from;

    while let Some(next) = current.checked_add(shift as u8) {
      if self.is_squared_occupied(next) || next > 63 {
        break;
      }
      actions.push(next);
      current = next;
    }
    actions
  }

  fn ray_attack(&self, from: u8, shift: Direction) -> u8 {
    let mut current = from;

    while let Some(next) = current.checked_add(shift as u8) {
      if self.is_squared_occupied(next) || next > 63 {
        break;
      }
      current = next;
    }

    current
  }
}

impl RayCast for BitBoard {
  fn ray_cast(&self, from: u8, shift: Direction) -> Vec<u8> {
    let mut actions = Vec::new();
    let mut current = from;

    while let Some(next) = current.checked_add(shift as u8) {
      if self.is_set(next as u64) || next > 63 {
        break;
      }
      actions.push(next);
      current = next;
    }
    actions
  }

  fn ray_attack(&self, from: u8, shift: Direction) -> u8 {
    let mut current = from;

    while let Some(next) = current.checked_add(shift as u8) {
      if self.is_set(next as u64) || next > 63 {
        break;
      }
      current = next;
    }

    current
  }
}
