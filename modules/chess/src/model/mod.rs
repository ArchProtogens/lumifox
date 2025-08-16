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

//! Types and utilities for representing chess state.
//!
//! This module contains the core data structures used throughout the crate:
//! - `bitboard` — compact bitboard helpers and masks
//! - `gameboard` — the primary GameBoard structure and helpers (startpos, FEN)
//! - `gamedata` — additional metadata for positions
//! - `piecemove` — compact move representation used by the move generator
//! - `rays` — precomputed directional ray bitboards used by sliding pieces
//!
//! These types are intentionally low-level and designed for performance.

pub mod bitboard;
pub mod gameboard;
pub mod gamedata;
pub mod piecemove;
pub mod rays;
