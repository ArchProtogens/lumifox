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

/// Helper function to compress a rank string into FEN format
/// Converts sequences of dots/spaces into numbers
pub const fn compress_rank(rank: &str) -> &str {
  // For now, we'll use a simple approach - in a real implementation,
  // you'd want to compress consecutive empty squares into numbers
  // But const fn limitations make this tricky, so we'll do runtime compression
  rank
}

/// Runtime helper to compress rank with proper FEN numbering
pub fn compress_rank_runtime(rank: &str) -> String {
  let mut result = String::new();
  let mut empty_count = 0;

  for ch in rank.chars() {
    match ch {
      '.' | ' ' => {
        empty_count += 1;
      }
      piece => {
        if empty_count > 0 {
          result.push_str(&empty_count.to_string());
          empty_count = 0;
        }
        result.push(piece);
      }
    }
  }

  // Add any remaining empty squares
  if empty_count > 0 {
    result.push_str(&empty_count.to_string());
  }

  result
}

/// Create a list of moves from various notation formats
///
/// Supports multiple formats:
/// - UCI notation: "e2e4", "e7e8q" (with promotion)
/// - Algebraic with capture: "e4xd5", "Nxf7"
/// - Simple format: "e4-d5" (quiet move), "e4xd5" (capture)
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::move_list;
/// use lumifox_chess::model::piecemove::PieceMove;
///
/// // For tests - create expected move lists
/// let expected_moves = move_list![
///   "e2e4",      // King pawn
///   "d7d5",      // Black responds
///   "e4xd5",     // Capture
///   "e7e8q"      // Promotion
/// ];
///
/// // Clean syntax for test assertions
/// let knight_moves = move_list!["g1f3", "b8c6", "f3e5"];
/// ```
#[macro_export]
macro_rules! move_list {
  [$($move_str:literal),* $(,)?] => {{
    vec![
      $(
        $crate::san!($move_str)
      ),*
    ]
  }};
}

/// Create a position using visual board representation
///
/// Much more readable than FEN for test cases and examples.
/// Supports piece placement with standard symbols.
///
/// # Examples
///
/// ```
/// use lumifox_chess_proc::position;
/// use lumifox_chess::model::gamedata::GameData;
///
/// // Starting position
/// let start = position! {
///   "rnbqkbnr"
///   "pppppppp"
///   "........"
///   "........"
///   "........"
///   "........"
///   "PPPPPPPP"
///   "RNBQKBNR"
///   ; to_move: White
///   ; castling: "KQkq"
///   ; en_passant: None
///   ; halfmove: 0
///   ; fullmove: 1
/// };
///
/// // Custom position
/// let custom = position! {
///   "r.bqkb.r"
///   "pppp.ppp"
///   "..n..n.."
///   "....p..."
///   "....P..."
///   "..N..N.."
///   "PPPP.PPP"
///   "R.BQKB.R"
///   ; to_move: White
/// };
/// ```
#[macro_export]
macro_rules! position {
  {
    $rank8:literal
    $rank7:literal
    $rank6:literal
    $rank5:literal
    $rank4:literal
    $rank3:literal
    $rank2:literal
    $rank1:literal
    $(; to_move: $to_move:ident)?
    $(; castling: $castling:literal)?
    $(; en_passant: $en_passant:tt)?
    $(; halfmove: $halfmove:literal)?
    $(; fullmove: $fullmove:literal)?
  } => {{
    // Build FEN string from visual representation
    let piece_placement = format!("{}/{}/{}/{}/{}/{}/{}/{}",
      $crate::macros::positions::compress_rank_runtime($rank8),
      $crate::macros::positions::compress_rank_runtime($rank7),
      $crate::macros::positions::compress_rank_runtime($rank6),
      $crate::macros::positions::compress_rank_runtime($rank5),
      $crate::macros::positions::compress_rank_runtime($rank4),
      $crate::macros::positions::compress_rank_runtime($rank3),
      $crate::macros::positions::compress_rank_runtime($rank2),
      $crate::macros::positions::compress_rank_runtime($rank1)
    );

    // Default values
    let active_color = position!(@to_move $($to_move)?);
    let castling_rights = position!(@castling $($castling)?);
    let en_passant_target = position!(@en_passant $($en_passant)?);
    let halfmove_clock = position!(@halfmove $($halfmove)?);
    let fullmove_number = position!(@fullmove $($fullmove)?);

    let fen = format!("{} {} {} {} {} {}",
      piece_placement,
      active_color,
      castling_rights,
      en_passant_target,
      halfmove_clock,
      fullmove_number
    );

    lumifox_chess::model::gamedata::GameData::from_fen(&fen)
      .unwrap_or_else(|e| panic!("Invalid position: {:?}", e))
  }};

  // Helper rules for defaults
  (@to_move White) => { "w" };
  (@to_move Black) => { "b" };
  (@to_move) => { "w" };

  (@castling $castling:literal) => { $castling };
  (@castling) => { "KQkq" };

  (@en_passant Some($square:literal)) => { $square };
  (@en_passant None) => { "-" };
  (@en_passant) => { "-" };

  (@halfmove $halfmove:literal) => { stringify!($halfmove) };
  (@halfmove) => { "0" };

  (@fullmove $fullmove:literal) => { stringify!($fullmove) };
  (@fullmove) => { "1" };
}
