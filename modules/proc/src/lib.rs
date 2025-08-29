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

//! # Chess Declarative Macros
//!
//! This crate provides compile-time chess utilities for the Lumifox chess library
//! using declarative macros (`macro_rules!`):
//!
//! ## Position Creation
//! - `fen!()` - Parse FEN strings with validation
//! - `position!()` - Visual board creation with piece placement
//!
//! ## Literals
//! - `sq!()` - Square notation to indices
//! - `bitboard!()` - Create bitboards from square lists
//! - `san!()` - UCI move notation parsing
//! - `move_list!()` - Create move lists for testing
//!
//! ## Opening Database
//! - `opening!()` - Look up chess openings by name (case-insensitive, PGN parsed into SAN move strings)
//! - `opening_list!()` - Get all available opening names
//! - `opening_search!()` - Search openings by partial name match
//!
//! ## Example Usage
//!
//! ```rust
//! use lumifox_chess_proc::{fen, sq, bitboard, san, move_list, position, opening, opening_search};
//! use lumifox_chess::model::gamedata::GameData;
//!
//! // Parse starting position with FEN
//! let start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
//!
//! // Create position visually
//! let visual_pos = position! {
//!     "rnbqkbnr"
//!     "pppppppp"
//!     "........"
//!     "........"
//!     "........"
//!     "........"
//!     "PPPPPPPP"
//!     "RNBQKBNR"
//!     ; to_move: White
//! };
//!
//! // Create square indices
//! const E4: u8 = sq!("e4");
//!
//! // Create bitboards
//! let center_squares = bitboard!("e4", "e5", "d4", "d5");
//!
//! // Parse individual moves
//! let king_pawn = san!("e2e4");
//! let promotion = san!("e7e8q");
//!
//! // Create move lists for testing
//! let expected_moves = move_list!["e2e4", "d7d5", "e4xd5"];
//!
//! // Look up chess openings (case-insensitive, with parsed SAN moves)
//! let sicilian = opening!("Sicilian Defense");
//! let ruy_lopez = opening!("ruy lopez");  // lowercase works too
//! println!("Sicilian ECO: {}, SAN moves: {:?}", sicilian.eco, sicilian.moves);
//!
//! // Search for openings
//! let all_sicilian = opening_search!("Sicilian");
//! println!("Found {} Sicilian variations", all_sicilian.len());
//! ```

pub mod macros;
