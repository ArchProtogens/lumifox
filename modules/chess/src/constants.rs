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

pub static NUMBER_OF_PIECES: usize = 6;
pub static BOARD_SIZE: usize = 64;

pub const BLACK: u8 = 0b0000_1000; // Bit 3 for color (1 for black, 0 for white)
pub const MOVED: u8 = 0b0001_0000; // Bit 4 for has_moved
pub const PROMO: u8 = 0b0010_0000; // Bit 5 for is_promoted
pub const EMPTY: u8 = 0; // Represents a completely empty square
pub const WHITE: u8 = 0; // Bit 3 for color (0 for white)

pub const FROM_MASK: u16 = 0b0000_0000_0011_1111;
pub const DEST_MASK: u16 = 0b0000_1111_1100_0000;
pub const PROMOTION_MASK: u16 = 0b0001_0000_0000_0000;
pub const CAPTURE_MASK: u16 = 0b0010_0000_0000_0000;
pub const EN_PASSANT_MASK: u16 = 0b0100_0000_0000_0000;
pub const CASTLING_MASK: u16 = 0b1000_0000_0000_0000;
