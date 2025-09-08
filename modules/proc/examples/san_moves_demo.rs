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

//! Demonstration showing how to work with SAN move strings from openings.
//!
//! Note: The moves are provided as SAN (Standard Algebraic Notation) strings
//! like "e4", "Nf3", "Bb5". To convert these to PieceMove objects, you would
//! need a SAN parser that considers board state, which isn't currently available
//! in the chess engine.

use lumifox_chess_proc::opening;

// Helper function to identify move types from SAN strings
fn analyze_san_move(san: &str) -> String {
  if san == "O-O" {
    return "Kingside castling".to_string();
  }
  if san == "O-O-O" {
    return "Queenside castling".to_string();
  }

  let chars: Vec<char> = san.chars().collect();

  // Check for capture
  let is_capture = san.contains('x');

  // Check for check/checkmate
  let has_check = san.ends_with('+');
  let has_checkmate = san.ends_with('#');

  // Determine piece type
  let piece = match chars.first() {
    Some('K') => "King",
    Some('Q') => "Queen",
    Some('R') => "Rook",
    Some('B') => "Bishop",
    Some('N') => "Knight",
    Some(c) if c.is_ascii_lowercase() => "Pawn",
    _ => "Unknown",
  };

  let mut description = format!("{} move", piece);

  if is_capture {
    description.push_str(" (capture)");
  }
  if has_check {
    description.push_str(" (check)");
  }
  if has_checkmate {
    description.push_str(" (checkmate)");
  }

  description
}

fn main() {
  println!("=== SAN Move Analysis Demo ===\n");

  let sicilian = opening!("Sicilian Defense");
  println!("Sicilian Defense ({}):", sicilian.eco);
  println!("PGN: {}", sicilian.pgn);
  println!("Parsed moves:");

  for (i, move_str) in sicilian.moves.iter().enumerate() {
    let analysis = analyze_san_move(move_str);
    let move_num = (i / 2) + 1;
    let color = if i % 2 == 0 { "White" } else { "Black" };
    println!(
      "  {}. {} ({}): {} - {}",
      move_num, color, move_str, analysis, move_str
    );
  }

  println!("\n{}", "=".repeat(50));

  let ruy_lopez = opening!("Ruy Lopez");
  println!("\nRuy Lopez ({}):", ruy_lopez.eco);
  println!("PGN: {}", ruy_lopez.pgn);
  println!("Move sequence:");

  for (i, move_str) in ruy_lopez.moves.iter().enumerate() {
    let move_num = (i / 2) + 1;
    if i % 2 == 0 {
      print!("{}. {}", move_num, move_str);
    } else {
      println!(" {}", move_str);
    }
  }

  // Show how you might work with specific moves
  println!("\n\nAnalyzing specific moves:");
  println!(
    "First move ({}): {}",
    sicilian.moves[0],
    analyze_san_move(sicilian.moves[0])
  );
  println!(
    "Second move ({}): {}",
    sicilian.moves[1],
    analyze_san_move(sicilian.moves[1])
  );

  // Show a more complex opening
  println!("\n{}", "=".repeat(50));
  let dragon = opening!("Sicilian Defense: Dragon Variation");
  println!("\nSicilian Dragon ({}):", dragon.eco);
  println!("Total moves: {}", dragon.moves.len());
  println!("Move breakdown:");

  for (i, move_str) in dragon.moves.iter().enumerate() {
    let move_num = (i / 2) + 1;
    println!(
      "  {}{} {}",
      move_num,
      if i % 2 == 0 { "." } else { "..." },
      move_str
    );
  }

  println!("\n=== Demo Complete ===");
  println!("\nNote: These are SAN (Standard Algebraic Notation) strings.");
  println!("To convert to PieceMove objects, you would need a SAN parser");
  println!("that considers the current board position for disambiguation.");
}
