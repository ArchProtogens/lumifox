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

//! Demonstration of case-insensitive opening lookup.

use lumifox_chess_proc::opening;

fn main() {
  println!("=== Case-Insensitive Opening Lookup Demo ===\n");

  // Test different case variations
  println!("1. Testing case-insensitive lookup:");

  let sicilian1 = opening!("Sicilian Defense");
  println!("Original case: {} - {}", sicilian1.eco, sicilian1.pgn);

  let sicilian2 = opening!("sicilian defense");
  println!("Lowercase: {} - {}", sicilian2.eco, sicilian2.pgn);

  let sicilian3 = opening!("SICILIAN DEFENSE");
  println!("Uppercase: {} - {}", sicilian3.eco, sicilian3.pgn);

  let sicilian4 = opening!("SiCiLiAn DeFenSe");
  println!("Mixed case: {} - {}", sicilian4.eco, sicilian4.pgn);

  // Verify they're all the same
  assert_eq!(sicilian1.eco, sicilian2.eco);
  assert_eq!(sicilian2.eco, sicilian3.eco);
  assert_eq!(sicilian3.eco, sicilian4.eco);
  println!("\n✅ All variations return the same opening!");

  println!("\n2. Testing other openings with various cases:");

  let ruy_lopez1 = opening!("Ruy Lopez");
  let ruy_lopez2 = opening!("ruy lopez");
  println!(
    "Ruy Lopez (original): {} - {}",
    ruy_lopez1.eco, ruy_lopez1.pgn
  );
  println!(
    "ruy lopez (lowercase): {} - {}",
    ruy_lopez2.eco, ruy_lopez2.pgn
  );
  assert_eq!(ruy_lopez1.eco, ruy_lopez2.eco);

  let kings_indian1 = opening!("King's Indian Defense");
  let kings_indian2 = opening!("KING'S INDIAN DEFENSE");
  println!(
    "King's Indian (original): {} - {}",
    kings_indian1.eco, kings_indian1.pgn
  );
  println!(
    "KING'S INDIAN (uppercase): {} - {}",
    kings_indian2.eco, kings_indian2.pgn
  );
  assert_eq!(kings_indian1.eco, kings_indian2.eco);

  println!("\n✅ Case-insensitive lookup is working perfectly!");
  println!("\n=== Demo Complete ===");
}
