/*
 * Example: legal_play.rs
 *
 * ♔ ♕ ♖ ♗ ♘ ♙ Interactive Chess Game ♙ ♘ ♗ ♖ ♕ ♔
 *
 * This example implements a beautiful interactive chess game where:
 * - The human plays as White
 * - The AI (random moves) plays as Black
 * - Starts from the standard initial position
 * - Human inputs moves in algebraic notation (e.g., "e2e4" or "e7e8q" for promotion)
 * - Move legality is strictly checked
 * - The game continues until checkmate or stalemate
 * - Beautiful colored output and board display
 *
 * Usage: cargo run --features std --example legal_play
 */

use lumifox_chess::{
  model::{
    gamedata::GameData,
    piecemove::{PieceMove, PromotionType},
  },
  movegen::generate_moves,
};
use std::io;

fn print_move(piece_move: &PieceMove) {
  if *piece_move == PieceMove::NULL {
    println!("🚫 NULL move");
  } else {
    let from = piece_move.from_square();
    let to = piece_move.to_square();

    let from_file = (from % 8 + b'a') as char;
    let from_rank = from / 8 + 1;
    let to_file = (to % 8 + b'a') as char;
    let to_rank = to / 8 + 1;

    print!(
      "♟️  \x1b[96m{}{}\x1b[0m → \x1b[96m{}{}\x1b[0m",
      from_file, from_rank, to_file, to_rank
    );

    if piece_move.is_capture() {
      print!(" \x1b[91m⚔️ (capture)\x1b[0m");
    }

    if let Some(promotion) = piece_move.promotion_type() {
      let piece_symbol = match promotion {
        PromotionType::Queen => "♕",
        PromotionType::Rook => "♖",
        PromotionType::Bishop => "♗",
        PromotionType::Knight => "♘",
      };
      print!(
        " \x1b[95m👑 (promote to {} {:?})\x1b[0m",
        piece_symbol, promotion
      );
    }
    println!();
  }
}

fn alg_to_square(alg: &str) -> Option<u8> {
  if alg.len() != 2 {
    return None;
  }
  let mut chars = alg.chars();
  let file_char = chars.next()?;
  let rank_char = chars.next()?;

  let file = match file_char {
    'a'..='h' => file_char as u8 - b'a',
    _ => return None,
  };
  let rank = match rank_char {
    '1'..='8' => rank_char as u8 - b'1',
    _ => return None,
  };

  Some(rank * 8 + file)
}

fn main() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  let random_mode = args.iter().any(|a| a == "--random");
  let mut sleep_ms = 800u64;
  if let Some(speed_idx) = args.iter().position(|a| a == "--speed") {
    if let Some(val) = args.get(speed_idx + 1) {
      if let Ok(ms) = val.parse::<u64>() {
        sleep_ms = ms;
      } else {
        eprintln!("Invalid value for --speed: {} (must be an integer)", val);
      }
    } else {
      eprintln!("--speed requires a value in milliseconds");
    }
  }

  let mut game = GameData::START_POS;

  // Print beautiful welcome message
  println!("\n┌─────────────────────────────────────────────────────┐");
  println!("│                 🏁 \x1b[1;36mWelcome to Chess!\x1b[0m 🏁             │");
  println!("├─────────────────────────────────────────────────────┤");
  if random_mode {
    println!("│  \x1b[97m♔\x1b[0m Both sides play random moves!                    │");
    println!("│  \x1b[90m♚\x1b[0m No human input required.                         │");
  } else {
    println!(
      "│  \x1b[97m♔\x1b[0m You play as \x1b[1;97mWhite\x1b[0m                                │"
    );
    println!(
      "│  \x1b[90m♚\x1b[0m AI plays as \x1b[1;90mBlack\x1b[0m (random moves)                 │"
    );
    println!(
      "│  📝 Enter moves like: \x1b[96me2e4\x1b[0m or \x1b[96me7e8q\x1b[0m                 │"
    );
  }
  println!("│                                                     │");
  println!("│  🎯 All moves are checked for legality              │");
  println!("│  🏆 Game ends at checkmate or stalemate             │");
  println!("└─────────────────────────────────────────────────────┘\n");

  let mut move_counter = 1;

  loop {
    // Print move number and board
    println!("═══════════════════════════════════════════════════════");
    println!(
      "                    \x1b[1;33m📋 Move #{}\x1b[0m",
      move_counter
    );
    println!("═══════════════════════════════════════════════════════");

    game.print_board();

    let current_turn = if game.board.playing {
      "\x1b[1;97m♔ White (Your turn)\x1b[0m"
    } else {
      "\x1b[1;90m♚ Black (AI's turn)\x1b[0m"
    };
    println!("\n🎯 Current turn: {}", current_turn);

    let (moves, count) = generate_moves(&game.board);
    if count == 0 {
      // Simple check detection - look for any opponent pieces attacking current player's king
      println!("\n🚫 No legal moves available!");

      // For a more detailed check detection, we'd need to implement a proper in_check function
      // For now, let's just indicate game over
      let winner = if game.board.playing {
        "\x1b[1;90m♚ Black"
      } else {
        "\x1b[1;97m♔ White"
      };
      println!("🏆 Game Over! {} wins!\x1b[0m", winner);
      break;
    }

    if game.board.playing && !random_mode {
      // Human's turn (White)
      println!("📝 Enter your move (e.g., \x1b[96me2e4\x1b[0m or \x1b[96me7e8q\x1b[0m): ");
      print!("   ➤ ");

      let mut input = String::new();
      if io::stdin().read_line(&mut input).is_err() {
        println!("❌ Invalid input. Try again.");
        continue;
      }
      let input = input.trim();

      if input.len() < 4 || input.len() > 5 {
        println!("❌ Invalid move format. Use 4-5 characters (e.g., e2e4).");
        continue;
      }

      let from_str = &input[0..2];
      let to_str = &input[2..4];
      let promo_char = if input.len() == 5 {
        Some(input.chars().nth(4).unwrap())
      } else {
        None
      };

      let from = match alg_to_square(from_str) {
        Some(sq) => sq,
        None => {
          println!("❌ Invalid from square '{}'. Use a1-h8 format.", from_str);
          continue;
        }
      };
      let to = match alg_to_square(to_str) {
        Some(sq) => sq,
        None => {
          println!("❌ Invalid to square '{}'. Use a1-h8 format.", to_str);
          continue;
        }
      };

      let promotion = match promo_char {
        None => None,
        Some('q') => Some(PromotionType::Queen),
        Some('r') => Some(PromotionType::Rook),
        Some('b') => Some(PromotionType::Bishop),
        Some('n') => Some(PromotionType::Knight),
        _ => {
          println!(
            "❌ Invalid promotion piece '{}'. Use q, r, b, or n.",
            promo_char.unwrap()
          );
          continue;
        }
      };

      // Find matching legal move
      let mut selected_move = None;
      for &mv in moves.iter().take(count) {
        if mv.from_square() == from && mv.to_square() == to && mv.promotion_type() == promotion {
          selected_move = Some(mv);
          break;
        }
      }

      match selected_move {
        Some(mv) => {
          print!("✅ You play: ");
          print_move(&mv);

          // Attempt to apply the move; move_piece returns Option<()> (None if illegal)
          match game.board.move_piece(&mv) {
            Some(()) => {
              game.plies += 1;
              if mv.is_capture() {
                game.halfmove_clock = 0;
              } else {
                game.halfmove_clock += 1;
              }
              if !game.board.playing {
                // After human move, it becomes AI's turn
                move_counter += 1;
              }
            }
            None => {
              println!("\n🚨 \x1b[1;31mINTERNAL ERROR:\x1b[0m move was rejected (illegal)");
              println!("⚠️  This indicates a bug in the chess engine.");
              println!("🔧 Please report this issue. Continuing game...\n");
              continue;
            }
          }
        }
        None => {
          println!("\x1b[91m❌ Illegal move! That move is not allowed.\x1b[0m");
          println!("💡 Tip: Make sure the piece can legally move there.");
          continue;
        }
      }
    } else {
      // AI's turn (Black) - random legal move
      println!("🤖 AI is thinking...");
      std::thread::sleep(std::time::Duration::from_millis(sleep_ms)); // Dramatic pause

      let rnd_id = rand::random::<u32>() as usize % count;
      let mv = moves[rnd_id];
      print!("🤖 AI plays: ");
      print_move(&mv);

      // Use direct move_piece for AI moves (returns Option<()>). If it fails,
      // try other legal moves. move_piece now returns None for illegal moves.
      match game.board.move_piece(&mv) {
        Some(()) => {
          game.plies += 1;
          if mv.is_capture() {
            game.halfmove_clock = 0;
          } else {
            game.halfmove_clock += 1;
          }
        }
        None => {
          println!("\n🚨 \x1b[1;31mAI MOVE ERROR:\x1b[0m move was rejected (illegal)");
          println!("⚠️  This indicates a bug in the move generation.");
          println!("🔄 AI will try a different move...\n");

          // Try to find a safe move from the remaining legal moves. No need to
          // call `is_move_legal` first because `move_piece` already performs
          // legality checks and returns None if the move is illegal.
          let mut found_safe_move = false;
          for &test_mv in moves.iter().take(count) {
            if test_mv != mv {
              if let Some(()) = game.board.move_piece(&test_mv) {
                print!("🤖 AI plays (retry): ");
                print_move(&test_mv);
                game.plies += 1;
                if test_mv.is_capture() {
                  game.halfmove_clock = 0;
                } else {
                  game.halfmove_clock += 1;
                }
                found_safe_move = true;
                break;
              }
            }
          }

          if !found_safe_move {
            println!("🚫 AI cannot find any safe moves. Ending game.");
            break;
          }
        }
      }
    }
  }

  println!("\n┌─────────────────────────────────────────────────────┐");
  println!("│                  🏆 \x1b[1;33mGame Over!\x1b[0m 🏆                   │");
  println!("│                                                     │");
  println!("│             Thanks for playing chess! 🎉            │");
  println!("└─────────────────────────────────────────────────────┘");

  println!("\n📋 Final board:");
  game.print_board();
}
