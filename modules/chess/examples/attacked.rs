/*
 * Example: attacked.rs
 *
 * This example demonstrates chess position analysis by:
 * 1. Parsing a FEN string to create a chess position
 * 2. Identifying all pieces that are under attack
 * 3. Showing the specific attacking moves for each attacked piece
 * 4. Displaying comprehensive move analysis for the current player
 *
 * Usage: cargo run --features std --example attacked "<FEN_STRING>"
 * Example: cargo run --features std --example attacked "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
 */

use lumifox_chess::{
  legal::attack::is_square_attacked,
  model::{gameboard::PieceType, gamedata::GameData, piecemove::PieceMove},
  movegen::generate_moves,
};
use std::env;

fn square_to_algebraic(square: u8) -> String {
  let file = (square % 8 + b'a') as char;
  let rank = square / 8 + 1;
  format!("{}{}", file, rank)
}

fn print_attacked_piece(square: u8, piece_type: PieceType, is_white: bool) {
  let color_str = if is_white { "White" } else { "Black" };
  let piece_str = format!("{:?}", piece_type);
  println!(
    "\x1b[91m{} {} on {} is attacked!\x1b[0m",
    color_str,
    piece_str.to_lowercase(),
    square_to_algebraic(square)
  );
}

fn print_attack_moves(game: &GameData, attacked_square: u8) {
  // Generate all possible moves for the current player (the attacker)
  let (moves, count) = generate_moves(&game.board);

  // Find moves that attack the specified square
  let mut attack_moves = Vec::new();
  for i in 0..count {
    let mv = moves[i];
    if mv.to_square() == attacked_square {
      attack_moves.push(mv);
    }
  }

  if !attack_moves.is_empty() {
    println!(
      "  Attack moves targeting {}:",
      square_to_algebraic(attacked_square)
    );
    for mv in attack_moves {
      let from_sq = square_to_algebraic(mv.from_square());
      let to_sq = square_to_algebraic(mv.to_square());
      let from_piece = game.board.get_piece(mv.from_square());

      if let Some(piece_type) = from_piece {
        print!(
          "    \x1b[96m{:?}\x1b[0m {} → {}",
          piece_type, from_sq, to_sq
        );

        if mv.is_capture() {
          print!(" \x1b[93m(capture)\x1b[0m");
        }

        if let Some(promotion) = mv.promotion_type() {
          print!(" \x1b[95m(promote to {:?})\x1b[0m", promotion);
        }

        println!();
      }
    }
  }
}

fn is_castling_move(mv: &PieceMove, game: &GameData) -> bool {
  let from = mv.from_square();
  let to = mv.to_square();

  // Check if it's a king move and matches castling pattern
  if let Some(PieceType::King) = game.board.get_piece(from) {
    let is_white = game.board.colour.get_bit_unchecked(from);
    PieceMove::is_kingside_castling(from, to, is_white)
      || PieceMove::is_queenside_castling(from, to, is_white)
  } else {
    false
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("Usage: {} <FEN_STRING>", args[0]);
    eprintln!(
      "Example: {} \"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\"",
      args[0]
    );
    std::process::exit(1);
  }

  let fen = &args[1];

  // Parse the FEN string
  let game = match GameData::from_fen(fen) {
    Ok(game) => game,
    Err(e) => {
      eprintln!("Error parsing FEN string: {:?}", e);
      std::process::exit(1);
    }
  };

  println!("Analyzing position from FEN: \x1b[94m{}\x1b[0m\n", fen);

  // Print the board
  println!("Board position:");
  game.print_board();

  println!(
    "\nCurrent turn: \x1b[93m{}\x1b[0m",
    if game.board.playing { "White" } else { "Black" }
  );

  // Check every square for pieces and if they are attacked
  let mut attacked_pieces = Vec::new();
  let mut any_attacked = false;

  println!("\n\x1b[92m=== Checking for attacked pieces ===\x1b[0m\n");

  for square in 0..64 {
    if let Some(piece_type) = game.board.get_piece(square) {
      let is_white = game.board.colour.get_bit_unchecked(square);

      // Check if this piece is attacked
      if is_square_attacked(&game.board, square) {
        print_attacked_piece(square, piece_type, is_white);
        attacked_pieces.push((square, piece_type, is_white));
        any_attacked = true;

        // Show the attacking moves
        print_attack_moves(&game, square);
        println!();
      }
    }
  }

  if !any_attacked {
    println!("\x1b[92mNo pieces are currently under attack.\x1b[0m");
  } else {
    println!(
      "\x1b[91m{} piece(s) are under attack!\x1b[0m",
      attacked_pieces.len()
    );

    // Summary of attacked pieces
    println!("\n\x1b[92m=== Summary of attacked pieces ===\x1b[0m");
    for (square, piece_type, is_white) in &attacked_pieces {
      let color_str = if *is_white { "White" } else { "Black" };
      println!(
        "  • {} {:?} on {}",
        color_str,
        piece_type,
        square_to_algebraic(*square)
      );
    }
  }

  // Additional analysis: Show all possible moves for the current player
  println!("\n\x1b[92m=== All possible moves for current player ===\x1b[0m");
  let (moves, count) = generate_moves(&game.board);

  if count == 0 {
    println!("No legal moves available! This could indicate checkmate or stalemate.");
  } else {
    println!("Total moves available: \x1b[93m{}\x1b[0m\n", count);

    let mut captures = 0;
    let mut normal_moves = 0;
    let mut special_moves = 0;

    for i in 0..count.min(10) {
      // Show first 10 moves as examples
      let mv = moves[i];
      let from_sq = square_to_algebraic(mv.from_square());
      let to_sq = square_to_algebraic(mv.to_square());
      let from_piece = game.board.get_piece(mv.from_square());

      if let Some(piece_type) = from_piece {
        print!("  {:?} {} → {}", piece_type, from_sq, to_sq);

        if mv.is_capture() {
          print!(" \x1b[93m(capture)\x1b[0m");
          captures += 1;
        } else {
          normal_moves += 1;
        }

        if let Some(promotion) = mv.promotion_type() {
          print!(" \x1b[95m(promote to {:?})\x1b[0m", promotion);
          special_moves += 1;
        }

        if mv.is_en_passant() {
          print!(" \x1b[96m(en passant)\x1b[0m");
          special_moves += 1;
        }

        if is_castling_move(&mv, &game) {
          print!(" \x1b[94m(castling)\x1b[0m");
          special_moves += 1;
        }

        println!();
      }
    }

    if count > 10 {
      println!("  ... and {} more moves", count - 10);
    }

    // Show statistics
    println!("\n\x1b[92m=== Move Statistics ===\x1b[0m");
    println!("Captures: \x1b[93m{}\x1b[0m", captures);
    println!("Normal moves: \x1b[97m{}\x1b[0m", normal_moves);
    if special_moves > 0 {
      println!(
        "Special moves (promotions/castling/en passant): \x1b[95m{}\x1b[0m",
        special_moves
      );
    }
  }
}
