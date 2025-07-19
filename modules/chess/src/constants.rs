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

pub static NUMBER_OF_PIECES: usize = 6;
pub static BOARD_SIZE: usize = 64;

pub const BLACK: u8 = 0b0000_1000; // Bit 3 for color (1 for black, 0 for white)
pub const MOVED: u8 = 0b0001_0000; // Bit 4 for has_moved
pub const PROMO: u8 = 0b0010_0000; // Bit 5 for is_promoted
pub const EMPTY: u8 = 0; // Represents a completely empty square
pub const WHITE: u8 = 0; // Bit 3 for color (0 for white)

pub const FILE_A: u64 = 0x0101_0101_0101_0101;
pub const FILE_B: u64 = 0x0202_0202_0202_0202;
pub const FILE_C: u64 = 0x0404_0404_0404_0404;
pub const FILE_D: u64 = 0x0808_0808_0808_0808;
pub const FILE_E: u64 = 0x1010_1010_1010_1010;
pub const FILE_F: u64 = 0x2020_2020_2020_2020;
pub const FILE_G: u64 = 0x4040_4040_4040_4040;
pub const FILE_H: u64 = 0x8080_8080_8080_8080;
pub const RANK_1: u64 = 0x0000_0000_0000_00FF;
pub const RANK_2: u64 = 0x0000_0000_0000_FF00;
pub const RANK_3: u64 = 0x0000_0000_00FF_0000;
pub const RANK_4: u64 = 0x0000_0000_FF00_0000;
pub const RANK_5: u64 = 0x0000_00FF_0000_0000;
pub const RANK_6: u64 = 0x0000_FF00_0000_0000;
pub const RANK_7: u64 = 0x00FF_0000_0000_0000;
pub const RANK_8: u64 = 0xFF00_0000_0000_0000;

pub const FROM_MASK: u16 = 0b0000_0000_0011_1111;
pub const DEST_MASK: u16 = 0b0000_1111_1100_0000;
pub const PROMOTION_MASK: u16 = 0b0001_0000_0000_0000;
pub const CAPTURE_MASK: u16 = 0b0010_0000_0000_0000;
pub const EN_PASSANT_MASK: u16 = 0b0100_0000_0000_0000;
pub const CASTLING_MASK: u16 = 0b1000_0000_0000_0000;

// Individual square definitions
pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;

pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;

pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;

pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;

pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;

pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;

pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;

pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;
