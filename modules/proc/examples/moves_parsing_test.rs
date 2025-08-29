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

//! Test example showing PGN parsing into individual moves.

use lumifox_chess_proc::opening;

fn main() {
  println!("=== PGN Moves Parsing Demo ===\n");

  // Test various openings to see their parsed moves
  let sicilian = opening!("Sicilian Defense");
  println!("Sicilian Defense ({}):", sicilian.eco);
  println!("  PGN: {}", sicilian.pgn);
  println!("  Moves: {:?}", sicilian.moves);
  println!("  Move count: {}", sicilian.moves.len());
  println!();

  let ruy_lopez = opening!("Ruy Lopez");
  println!("Ruy Lopez ({}):", ruy_lopez.eco);
  println!("  PGN: {}", ruy_lopez.pgn);
  println!("  Moves: {:?}", ruy_lopez.moves);
  println!("  Move count: {}", ruy_lopez.moves.len());
  println!();

  let kings_indian = opening!("King's Indian Defense");
  println!("King's Indian Defense ({}):", kings_indian.eco);
  println!("  PGN: {}", kings_indian.pgn);
  println!("  Moves: {:?}", kings_indian.moves);
  println!("  Move count: {}", kings_indian.moves.len());
  println!();

  // Test a longer opening line
  let dragon = opening!("Sicilian Defense: Dragon Variation");
  println!("Sicilian Dragon ({}):", dragon.eco);
  println!("  PGN: {}", dragon.pgn);
  println!("  Moves: {:?}", dragon.moves);
  println!("  Move count: {}", dragon.moves.len());
  println!();

  // Show how to access individual moves
  println!("=== Individual Move Access ===");
  println!("Sicilian Defense moves:");
  for (i, move_str) in sicilian.moves.iter().enumerate() {
    println!("  {}. {}", i + 1, move_str);
  }

  println!("\n=== Demo Complete ===");
}
