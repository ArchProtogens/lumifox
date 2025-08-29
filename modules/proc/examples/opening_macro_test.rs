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

//! Example demonstrating the opening macro functionality.

use lumifox_chess_proc::{opening, opening_list, opening_search};

fn main() {
  println!("=== Chess Opening Macro Demo ===\n");

  // Look up specific openings
  println!("1. Looking up specific openings:");

  let sicilian = opening!("Sicilian Defense");
  println!("Sicilian Defense:");
  println!("  ECO: {}", sicilian.eco);
  println!("  Name: {}", sicilian.name);
  println!("  Moves: {}", sicilian.pgn);
  println!();

  let ruy_lopez = opening!("Ruy Lopez");
  println!("Ruy Lopez:");
  println!("  ECO: {}", ruy_lopez.eco);
  println!("  Name: {}", ruy_lopez.name);
  println!("  Moves: {}", ruy_lopez.pgn);
  println!();

  // Search for openings
  println!("2. Searching for Sicilian variations:");
  let sicilian_variations = opening_search!("Sicilian");
  println!("Found {} Sicilian variations:", sicilian_variations.len());
  for (name, opening) in sicilian_variations.iter().take(5) {
    println!("  {} ({}): {}", opening.eco, name, opening.pgn);
  }
  if sicilian_variations.len() > 5 {
    println!("  ... and {} more", sicilian_variations.len() - 5);
  }
  println!();

  // Search for King's openings
  println!("3. Searching for King's openings:");
  let kings_openings = opening_search!("King");
  println!("Found {} King's variations:", kings_openings.len());
  for (name, opening) in kings_openings.iter().take(3) {
    println!("  {} ({}): {}", opening.eco, name, opening.pgn);
  }
  println!();

  // Show total count
  let all_openings = opening_list!();
  println!("4. Total openings in database: {}", all_openings.len());

  // Show some random openings
  println!("\n5. Sample openings from the database:");
  for name in all_openings.iter().take(10) {
    let opening = opening!(name);
    println!("  {} ({}): {}", opening.eco, name, opening.pgn);
  }

  println!("\n=== Demo Complete ===");
}
