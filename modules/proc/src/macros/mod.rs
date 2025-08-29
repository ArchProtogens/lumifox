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

//! Chess declarative macros organized by functionality.
//!
//! This module contains compile-time chess utilities using macro_rules!:
//! - FEN string parsing and validation
//! - Square, bitboard, and move notation literals
//! - Position creation and move list utilities
//! - Chess opening lookup and search

pub mod fen;
pub mod literals;
pub mod openings;
pub mod positions;
