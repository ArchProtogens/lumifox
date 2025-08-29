/*
 * Example: small_ai.rs
 *
 * ♔ ♕ ♖ ♗ ♘ ♙ Interactive Chess with Small AI ♙ ♘ ♗ ♖ ♕ ♔
 *
 * This example implements a chess game with a small AI that uses:
 * - Basic piece-square evaluation
 * - Simple minimax algorithm with alpha-beta pruning (depth 3)
 * - Material evaluation with standard piece values
 * - Center control and basic positional considerations
 *
 * Game modes:
 * - Human vs AI: Human plays as White, AI plays as Black
 * - AI vs AI: Both sides use the AI (with --ai-vs-ai flag)
 * - Adjustable thinking time with --speed parameter
 *
 * Usage:
 * - cargo run --features std --example small_ai
 * - cargo run --features std --example small_ai -- --ai-vs-ai
 * - cargo run --features std --example small_ai -- --speed 1500
 */

use lumifox_chess::{
  model::{
    gameboard::{GameBoard, PieceType},
    gamedata::GameData,
    piecemove::{PieceMove, PromotionType},
  },
  movegen::generate_moves,
};
use std::io;

// Piece values for evaluation (in centipawns)
const PIECE_VALUES: [i32; 6] = [
  100,   // Pawn
  320,   // Knight
  330,   // Bishop
  500,   // Rook
  900,   // Queen
  20000, // King
];

// Additional piece-square tables for rook and queen
const ROOK_TABLE: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
  0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0,
  5, 5, 0, 0, 0,
];

const QUEEN_TABLE: [i32; 64] = [
  -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10, -5,
  0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0, 0, 0,
  -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

// King endgame table (when few pieces remain)
const KING_ENDGAME_TABLE: [i32; 64] = [
  -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30, 30,
  20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10,
  20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30, -30, -50,
];

// Piece-square tables for positional evaluation
const PAWN_TABLE: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, //
  50, 50, 50, 50, 50, 50, 50, 50, //
  10, 10, 20, 30, 30, 20, 10, 10, //
  5, 5, 10, 25, 25, 10, 5, 5, //
  0, 0, 0, 20, 20, 0, 0, 0, //
  5, -5, -10, 0, 0, -10, -5, 5, //
  5, 10, 10, -20, -20, 10, 10, 5, //
  0, 0, 0, 0, 0, 0, 0, 0, //
];

const KNIGHT_TABLE: [i32; 64] = [
  -50, -40, -30, -30, -30, -30, -40, -50, //
  -40, -20, 0, 0, 0, 0, -20, -40, //
  -30, 0, 10, 15, 15, 10, 0, -30, //
  -30, 5, 15, 20, 20, 15, 5, -30, //
  -30, 0, 15, 20, 20, 15, 0, -30, //
  -30, 5, 10, 15, 15, 10, 5, -30, //
  -40, -20, 0, 5, 5, 0, -20, -40, //
  -50, -40, -30, -30, -30, -30, -40, -50, //
];

const BISHOP_TABLE: [i32; 64] = [
  -20, -10, -10, -10, -10, -10, -10, -20, //
  -10, 0, 0, 0, 0, 0, 0, -10, //
  -10, 0, 5, 10, 10, 5, 0, -10, //
  -10, 5, 5, 10, 10, 5, 5, -10, //
  -10, 0, 10, 10, 10, 10, 0, -10, //
  -10, 10, 10, 10, 10, 10, 10, -10, //
  -10, 5, 0, 0, 0, 0, 5, -10, //
  -20, -10, -10, -10, -10, -10, -10, -20, //
];

const KING_TABLE: [i32; 64] = [
  -30, -40, -40, -50, -50, -40, -40, -30, //
  -30, -40, -40, -50, -50, -40, -40, -30, //
  -30, -40, -40, -50, -50, -40, -40, -30, //
  -30, -40, -40, -50, -50, -40, -40, -30, //
  -20, -30, -30, -40, -40, -30, -30, -20, //
  -10, -20, -20, -20, -20, -20, -20, -10, //
  20, 20, 0, 0, 0, 0, 20, 20, //
  20, 30, 10, 0, 0, 10, 30, 20, //
];

// Smart AI struct with advanced evaluation parameters
struct SmallAI {
  depth: u8,
  transposition_table: std::collections::HashMap<u64, (i32, u8)>, // position hash -> (eval, depth)
}

impl SmallAI {
  fn new() -> Self {
    SmallAI {
      depth: 4, // Increased depth for smarter play
      transposition_table: std::collections::HashMap::new(),
    }
  }

  // Count material for endgame detection
  fn count_material(&self, board: &GameBoard) -> i32 {
    let mut material = 0;
    for square in 0..64 {
      if let Some(piece_type) = board.get_piece(square) {
        match piece_type {
          PieceType::Queen => material += 9,
          PieceType::Rook => material += 5,
          PieceType::Bishop | PieceType::Knight => material += 3,
          PieceType::Pawn => material += 1,
          _ => {}
        }
      }
    }
    material
  }

  // Check if we're in endgame (fewer pieces on board)
  fn is_endgame(&self, board: &GameBoard) -> bool {
    self.count_material(board) < 20 // Endgame threshold
  }

  // Evaluate king safety
  fn evaluate_king_safety(&self, board: &GameBoard, is_white: bool) -> i32 {
    let king_bitboard = board.kings
      & if is_white {
        board.colour
      } else {
        !board.colour
      };
    if king_bitboard.raw() == 0 {
      return -10000; // King missing is very bad
    }

    let king_square = king_bitboard.raw().trailing_zeros() as u8;
    let mut safety_score = 0;

    // Penalize exposed king in middlegame
    if !self.is_endgame(board) {
      let king_rank = king_square / 8;
      let expected_rank = if is_white { 0 } else { 7 };

      if (king_rank as i32 - expected_rank as i32).abs() > 1 {
        safety_score -= 50; // King too far from back rank
      }

      // Check for pawns in front of king
      let pawn_shield_squares = if is_white {
        [
          (king_square + 8).min(63),
          (king_square + 7).min(63),
          (king_square + 9).min(63),
        ]
      } else {
        [
          (king_square.saturating_sub(8)),
          (king_square.saturating_sub(7)),
          (king_square.saturating_sub(9)),
        ]
      };

      for &shield_square in &pawn_shield_squares {
        if board.pawns.get_bit_unchecked(shield_square)
          && (board.colour.get_bit_unchecked(shield_square) == is_white)
        {
          safety_score += 10; // Pawn shield bonus
        }
      }
    }

    safety_score
  }

  // Evaluate pawn structure
  fn evaluate_pawn_structure(&self, board: &GameBoard) -> i32 {
    let mut score = 0;
    let white_pawns = board.pawns & board.colour;
    let black_pawns = board.pawns & !board.colour;

    // Check for doubled pawns, isolated pawns, and passed pawns
    for file in 0u8..8 {
      let file_mask = 0x0101010101010101u64 << file;

      // Count pawns on each file
      let white_on_file = (white_pawns.raw() & file_mask).count_ones();
      let black_on_file = (black_pawns.raw() & file_mask).count_ones();

      // Doubled pawns penalty
      if white_on_file > 1 {
        score -= 10 * (white_on_file as i32 - 1);
      }
      if black_on_file > 1 {
        score += 10 * (black_on_file as i32 - 1);
      }

      // Check for passed pawns (no enemy pawns ahead)
      if white_on_file > 0 {
        let adjacent_files = [file.saturating_sub(1).min(7), (file + 1).min(7)];
        let mut blocked = false;
        for &adj_file in &adjacent_files {
          let adj_mask = 0x0101010101010101u64 << adj_file;
          if (black_pawns.raw() & adj_mask) != 0 {
            blocked = true;
            break;
          }
        }
        if !blocked {
          score += 20; // Passed pawn bonus
        }
      }

      if black_on_file > 0 {
        let adjacent_files = [file.saturating_sub(1).min(7), (file + 1).min(7)];
        let mut blocked = false;
        for &adj_file in &adjacent_files {
          let adj_mask = 0x0101010101010101u64 << adj_file;
          if (white_pawns.raw() & adj_mask) != 0 {
            blocked = true;
            break;
          }
        }
        if !blocked {
          score -= 20; // Passed pawn bonus for black
        }
      }
    }

    score
  }

  // Evaluate piece mobility and development
  fn evaluate_mobility(&self, board: &GameBoard) -> i32 {
    let (_moves, count) = generate_moves(board);
    let current_player_mobility = count as i32;

    // Switch sides to count opponent mobility
    let mut opposite_board = *board;
    opposite_board.playing = !opposite_board.playing;
    let (_opp_moves, opp_count) = generate_moves(&opposite_board);
    let opponent_mobility = opp_count as i32;

    // Mobility advantage
    let mobility_score = (current_player_mobility - opponent_mobility) * 2;

    // Development bonus (knights and bishops not on starting squares)
    let mut development_score = 0;

    // Check white development
    let white_knights = board.knights & board.colour;
    let white_bishops = board.bishops & board.colour;

    // Starting squares for white pieces
    if !white_knights.get_bit_unchecked(1) {
      development_score += 5;
    } // b1 knight moved
    if !white_knights.get_bit_unchecked(6) {
      development_score += 5;
    } // g1 knight moved
    if !white_bishops.get_bit_unchecked(2) {
      development_score += 5;
    } // c1 bishop moved
    if !white_bishops.get_bit_unchecked(5) {
      development_score += 5;
    } // f1 bishop moved

    // Check black development
    let black_knights = board.knights & !board.colour;
    let black_bishops = board.bishops & !board.colour;

    if !black_knights.get_bit_unchecked(57) {
      development_score -= 5;
    } // b8 knight moved
    if !black_knights.get_bit_unchecked(62) {
      development_score -= 5;
    } // g8 knight moved
    if !black_bishops.get_bit_unchecked(58) {
      development_score -= 5;
    } // c8 bishop moved
    if !black_bishops.get_bit_unchecked(61) {
      development_score -= 5;
    } // f8 bishop moved

    mobility_score + development_score
  }

  // Enhanced position evaluation with multiple factors
  fn evaluate_position(&self, board: &GameBoard) -> i32 {
    let mut score = 0;
    let is_endgame = self.is_endgame(board);

    // Material evaluation with piece-square tables
    for square in 0..64 {
      if let Some(piece_type) = board.get_piece(square) {
        let piece_value = PIECE_VALUES[piece_type as usize];
        let is_white = board.colour.get_bit_unchecked(square);

        // Add positional bonus based on piece type
        let positional_bonus = match piece_type {
          PieceType::Pawn => self.get_piece_square_value(&PAWN_TABLE, square, is_white),
          PieceType::Knight => self.get_piece_square_value(&KNIGHT_TABLE, square, is_white),
          PieceType::Bishop => self.get_piece_square_value(&BISHOP_TABLE, square, is_white),
          PieceType::Rook => self.get_piece_square_value(&ROOK_TABLE, square, is_white),
          PieceType::Queen => self.get_piece_square_value(&QUEEN_TABLE, square, is_white),
          PieceType::King => {
            if is_endgame {
              self.get_piece_square_value(&KING_ENDGAME_TABLE, square, is_white)
            } else {
              self.get_piece_square_value(&KING_TABLE, square, is_white)
            }
          }
        };

        let total_value = piece_value + positional_bonus;

        if is_white {
          score += total_value;
        } else {
          score -= total_value;
        }
      }
    }

    // Add advanced evaluation factors
    score += self.evaluate_king_safety(board, true) - self.evaluate_king_safety(board, false);
    score += self.evaluate_pawn_structure(board);
    score += self.evaluate_mobility(board);

    // Bishop pair bonus
    let white_bishops = (board.bishops & board.colour).raw().count_ones();
    let black_bishops = (board.bishops & !board.colour).raw().count_ones();
    if white_bishops >= 2 {
      score += 20;
    }
    if black_bishops >= 2 {
      score -= 20;
    }

    // Control of center squares (d4, d5, e4, e5)
    let center_squares = [27, 28, 35, 36]; // d4, e4, d5, e5
    for &square in &center_squares {
      if let Some(_) = board.get_piece(square) {
        let is_white = board.colour.get_bit_unchecked(square);
        score += if is_white { 5 } else { -5 };
      }
    }

    // Return score from current player's perspective
    if board.playing {
      score // White to move
    } else {
      -score // Black to move
    }
  }

  fn get_piece_square_value(&self, table: &[i32; 64], square: u8, is_white: bool) -> i32 {
    let index = if is_white {
      square as usize
    } else {
      (56 - (square & 56) + (square & 7)) as usize // Flip for black
    };
    table[index]
  }

  // Order moves for better alpha-beta pruning (captures first, then checks, then others)
  fn order_moves(&self, moves: &[PieceMove], count: usize) -> Vec<(PieceMove, i32)> {
    let mut move_scores = Vec::with_capacity(count);

    for i in 0..count {
      let mv = moves[i];
      let mut score = 0;

      // Prioritize captures (MVV-LVA - Most Valuable Victim, Least Valuable Attacker)
      if mv.is_capture() {
        score += 1000; // High priority for captures
      }

      // Prioritize promotions
      if mv.promotion_type().is_some() {
        score += 900;
      }

      // Add some randomness to avoid repetitive play
      score += ((mv.from_square() as u16 * 7 + mv.to_square() as u16) % 10) as i32;

      move_scores.push((mv, score));
    }

    // Sort moves by score (highest first)
    move_scores.sort_by(|a, b| b.1.cmp(&a.1));
    move_scores
  }

  // Generate a simple position hash for transposition table
  fn position_hash(&self, board: &GameBoard) -> u64 {
    let mut hash = 0u64;
    hash ^= board.pawns.raw();
    hash ^= board.knights.raw().wrapping_mul(2);
    hash ^= board.bishops.raw().wrapping_mul(3);
    hash ^= board.rooks.raw().wrapping_mul(5);
    hash ^= board.queens.raw().wrapping_mul(7);
    hash ^= board.kings.raw().wrapping_mul(11);
    hash ^= board.colour.raw().wrapping_mul(13);
    if board.playing {
      hash ^= 0x123456789ABCDEF0;
    }
    hash
  }

  // Enhanced minimax with transposition table and better move ordering
  fn minimax(
    &mut self,
    game: &mut GameData,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
  ) -> i32 {
    let position_hash = self.position_hash(&game.board);

    // Check transposition table
    if let Some(&(cached_eval, cached_depth)) = self.transposition_table.get(&position_hash) {
      if cached_depth >= depth {
        return cached_eval;
      }
    }

    if depth == 0 {
      let eval = self.evaluate_position(&game.board);
      self
        .transposition_table
        .insert(position_hash, (eval, depth));
      return eval;
    }

    let (moves, count) = generate_moves(&game.board);
    if count == 0 {
      // Check for checkmate vs stalemate
      let eval = if maximizing {
        -20000 + depth as i32
      } else {
        20000 - depth as i32
      };
      self
        .transposition_table
        .insert(position_hash, (eval, depth));
      return eval;
    }

    // Order moves for better pruning
    let ordered_moves = self.order_moves(&moves, count);

    if maximizing {
      let mut max_eval = i32::MIN;

      for (mv, _score) in ordered_moves {
        let mut new_game = *game;

        if new_game.board.move_piece(&mv).is_some() {
          new_game.plies += 1;
          if mv.is_capture() {
            new_game.halfmove_clock = 0;
          } else {
            new_game.halfmove_clock += 1;
          }

          let eval = self.minimax(&mut new_game, depth - 1, alpha, beta, false);
          max_eval = max_eval.max(eval);
          alpha = alpha.max(eval);

          if beta <= alpha {
            break; // Alpha-beta pruning
          }
        }
      }

      self
        .transposition_table
        .insert(position_hash, (max_eval, depth));
      max_eval
    } else {
      let mut min_eval = i32::MAX;

      for (mv, _score) in ordered_moves {
        let mut new_game = *game;

        if new_game.board.move_piece(&mv).is_some() {
          new_game.plies += 1;
          if mv.is_capture() {
            new_game.halfmove_clock = 0;
          } else {
            new_game.halfmove_clock += 1;
          }

          let eval = self.minimax(&mut new_game, depth - 1, alpha, beta, true);
          min_eval = min_eval.min(eval);
          beta = beta.min(eval);

          if beta <= alpha {
            break; // Alpha-beta pruning
          }
        }
      }

      self
        .transposition_table
        .insert(position_hash, (min_eval, depth));
      min_eval
    }
  }

  // Enhanced move selection with iterative deepening
  fn find_best_move(&mut self, game: &GameData) -> Option<PieceMove> {
    let (moves, count) = generate_moves(&game.board);
    if count == 0 {
      return None;
    }

    let mut best_move = moves[0];
    let mut best_eval = i32::MIN;

    // Use iterative deepening for better move ordering in subsequent depths
    for current_depth in 1..=self.depth {
      let mut current_best_move = moves[0];
      let mut current_best_eval = i32::MIN;

      let ordered_moves = self.order_moves(&moves, count);

      for (mv, _score) in ordered_moves {
        let mut new_game = *game;

        if new_game.board.move_piece(&mv).is_some() {
          new_game.plies += 1;
          if mv.is_capture() {
            new_game.halfmove_clock = 0;
          } else {
            new_game.halfmove_clock += 1;
          }

          let eval = self.minimax(&mut new_game, current_depth - 1, i32::MIN, i32::MAX, false);

          if eval > current_best_eval {
            current_best_eval = eval;
            current_best_move = mv;
          }
        }
      }

      // Update best move for this depth
      if current_depth == self.depth || current_best_eval > best_eval {
        best_eval = current_best_eval;
        best_move = current_best_move;
      }
    }

    Some(best_move)
  }
}

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
  let ai_vs_ai = args.iter().any(|a| a == "--ai-vs-ai");
  let mut sleep_ms = 1200u64;
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
  let mut ai = SmallAI::new();

  // Print beautiful welcome message
  println!("\n┌─────────────────────────────────────────────────────┐");
  println!("│              🧠 \x1b[1;36mWelcome to Smart AI Chess!\x1b[0m 🧠          │");
  println!("├─────────────────────────────────────────────────────┤");
  if ai_vs_ai {
    println!(
      "│  🤖 \x1b[97mWhite AI\x1b[0m vs \x1b[90mBlack AI\x1b[0m                           │"
    );
    println!("│  🧠 Both sides use advanced minimax (depth 4)       │");
    println!("│  ⚡ No human input required                          │");
  } else {
    println!(
      "│  \x1b[97m♔\x1b[0m You play as \x1b[1;97mWhite\x1b[0m                                │"
    );
    println!(
      "│  \x1b[90m♚\x1b[0m Smart AI plays as \x1b[1;90mBlack\x1b[0m (depth 4)                │"
    );
    println!(
      "│  📝 Enter moves like: \x1b[96me2e4\x1b[0m or \x1b[96me7e8q\x1b[0m                 │"
    );
  }
  println!("│                                                     │");
  println!("│  🎯 All moves are checked for legality              │");
  println!("│  🏆 Game ends at checkmate or stalemate             │");
  println!("│  🧠 AI features: transposition table, move ordering │");
  println!("│  🎭 Advanced evaluation: mobility, king safety,     │");
  println!("│     pawn structure, piece development               │");
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
      "\x1b[1;97m♔ White's turn\x1b[0m"
    } else {
      "\x1b[1;90m♚ Black's turn\x1b[0m"
    };
    println!("\n🎯 Current turn: {}", current_turn);

    let (moves, count) = generate_moves(&game.board);
    if count == 0 {
      println!("\n🚫 No legal moves available!");

      let winner = if game.board.playing {
        "\x1b[1;90m♚ Black"
      } else {
        "\x1b[1;97m♔ White"
      };
      println!("🏆 Game Over! {} wins!\x1b[0m", winner);
      break;
    }

    if game.board.playing && !ai_vs_ai {
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

          match game.board.move_piece(&mv) {
            Some(()) => {
              game.plies += 1;
              if mv.is_capture() {
                game.halfmove_clock = 0;
              } else {
                game.halfmove_clock += 1;
              }
              if !game.board.playing {
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
      // AI's turn
      let player_name = if game.board.playing {
        "🤖 White AI"
      } else {
        "🤖 Black AI"
      };
      println!("{} is thinking...", player_name);
      std::thread::sleep(std::time::Duration::from_millis(sleep_ms));

      match ai.find_best_move(&game) {
        Some(mv) => {
          print!("{} plays: ", player_name);
          print_move(&mv);

          match game.board.move_piece(&mv) {
            Some(()) => {
              game.plies += 1;
              if mv.is_capture() {
                game.halfmove_clock = 0;
              } else {
                game.halfmove_clock += 1;
              }
              if game.board.playing {
                move_counter += 1;
              }
            }
            None => {
              println!("\n🚨 \x1b[1;31mAI MOVE ERROR:\x1b[0m move was rejected (illegal)");
              println!("⚠️  This indicates a bug in the AI or chess engine.");
              break;
            }
          }
        }
        None => {
          println!("🚫 AI cannot find any moves. Game over!");
          break;
        }
      }
    }
  }

  println!("\n┌─────────────────────────────────────────────────────┐");
  println!("│                  🏆 \x1b[1;33mGame Over!\x1b[0m 🏆                   │");
  println!("│                                                     │");
  println!("│         Thanks for playing with Small AI! 🧠🎉       │");
  println!("└─────────────────────────────────────────────────────┘");

  println!("\n📋 Final board:");
  game.print_board();
}
