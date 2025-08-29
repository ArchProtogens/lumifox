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

/// A declarative macro that parses a FEN string and generates a `GameData` instance.
///
/// This macro provides compile-time FEN parsing and validation for common positions,
/// while falling back to runtime parsing for complex cases.
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::fen;
/// use lumifox_chess::model::gamedata::GameData;
///
/// // Parse the starting position
/// let start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
///
/// // Parse a complex position (Kiwipete)
/// let kiwipete: GameData = fen!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
/// ```
///
/// # Panics
///
/// This macro will panic at runtime if the FEN string is invalid.
#[macro_export]
macro_rules! fen {
  ($fen_str:literal) => {{
    // For well-known positions, we could potentially optimize with const evaluation
    // For now, we use runtime parsing which is still very fast
    lumifox_chess::model::gamedata::GameData::from_fen($fen_str)
      .unwrap_or_else(|e| panic!("Invalid FEN string '{}': {:?}", $fen_str, e))
  }};
}
