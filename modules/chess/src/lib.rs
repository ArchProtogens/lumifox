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

#![cfg_attr(not(any(test, feature = "std")), no_std)]

//! Lumifox Chess — high-performance chess primitives
//!
//! This crate provides low-level, high-performance chess primitives suitable for
//! building engines, UCI adapters, or analysis tools. It focuses on compact
//! bitboard representations, efficient move generation, and no_std friendliness
//! when the `std` feature is disabled.
//!
//! Key modules
//! - `model` — board and piece representations (bitboards, moves, game state)
//! - `movegen` — move generation for all piece types (fast, allocation-free)
//! - `legal` — move legality checks and attack detection
//! - `constants` — shared constants such as square indices and masks
//! - `errors` — crate-specific error types
//!
//! Example
//! ```rust
//! use lumifox_chess::model::gameboard::GameBoard;
//! use lumifox_chess::movegen::generate_moves;
//!
//! // Create a starting position and generate moves (API is intentionally low-level)
//! let board = GameBoard::START_POS;
//! let (moves, count) = generate_moves(&board);
//! assert!(count > 0);
//! ```
//!
//! For higher-level documentation and usage examples see the crate README at
//! <https://github.com/ArchProtogens/lumifox/tree/main/modules/chess>

pub mod constants;
pub mod errors;
pub mod legal;
pub mod model;
pub mod movegen;
