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

//! Simple demonstration of the exact usage you requested.

use lumifox_chess_proc::opening;

fn main() {
  // Your exact requested usage (now case-insensitive with parsed SAN moves):
  let sicilian = opening!("Sicilian Defense");
  let ruy_lopez = opening!("Ruy Lopez");

  // These work too due to case-insensitive lookup:
  let sicilian_lower = opening!("sicilian defense");
  let ruy_lopez_upper = opening!("RUY LOPEZ");

  println!("Sicilian Defense: {} - {}", sicilian.eco, sicilian.pgn);
  println!("  SAN Moves: {:?}", sicilian.moves);
  println!("Ruy Lopez: {} - {}", ruy_lopez.eco, ruy_lopez.pgn);
  println!("  SAN Moves: {:?}", ruy_lopez.moves);

  println!("\nCase-insensitive examples:");
  println!(
    "sicilian defense: {} - {}",
    sicilian_lower.eco, sicilian_lower.pgn
  );
  println!("  SAN Moves: {:?}", sicilian_lower.moves);
  println!(
    "RUY LOPEZ: {} - {}",
    ruy_lopez_upper.eco, ruy_lopez_upper.pgn
  );
  println!("  SAN Moves: {:?}", ruy_lopez_upper.moves);

  // Verify they're the same
  assert_eq!(sicilian.eco, sicilian_lower.eco);
  assert_eq!(ruy_lopez.eco, ruy_lopez_upper.eco);
  assert_eq!(sicilian.moves, sicilian_lower.moves);
  assert_eq!(ruy_lopez.moves, ruy_lopez_upper.moves);

  println!("\n✅ Case doesn't matter and moves are parsed as SAN strings!");

  // Show individual move access
  println!("\nFirst 3 SAN moves of Sicilian Defense:");
  for (i, move_str) in sicilian.moves.iter().take(3).enumerate() {
    println!("  {}. {} (SAN)", i + 1, move_str);
  }

  println!("\nNote: These are SAN (Standard Algebraic Notation) strings like 'e4', 'Nf3'.");
  println!("To convert to PieceMove objects, you'd need a SAN parser with board context.");
}
