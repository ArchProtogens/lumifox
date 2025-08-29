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

/// Helper const function to convert square notation to index
pub const fn square_to_index(square: &str) -> u8 {
  let bytes = square.as_bytes();
  if bytes.len() != 2 {
    panic!("Square notation must be exactly 2 characters");
  }

  let file = bytes[0];
  let rank = bytes[1];

  if file < b'a' || file > b'h' {
    panic!("File must be a-h");
  }
  if rank < b'1' || rank > b'8' {
    panic!("Rank must be 1-8");
  }

  let file_idx = file - b'a';
  let rank_idx = rank - b'1';
  rank_idx * 8 + file_idx
}

/// Compile-time square literal: e.g. sq!("e4") -> u8 index
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::sq;
///
/// const E4: u8 = sq!("e4");  // 28
/// const A1: u8 = sq!("a1");  // 0
/// const H8: u8 = sq!("h8");  // 63
/// ```
#[macro_export]
macro_rules! sq {
  ($square:literal) => {{
    const SQUARE_INDEX: u8 = $crate::macros::literals::square_to_index($square);
    SQUARE_INDEX
  }};
}

/// Compile-time bitboard from list of squares: e.g. bitboard!("a1", "h8")
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::bitboard;
/// use lumifox_chess::model::bitboard::BitBoard;
///
/// let center_squares = bitboard!("e4", "e5", "d4", "d5");
/// let corners = bitboard!("a1", "a8", "h1", "h8");
/// ```
#[macro_export]
macro_rules! bitboard {
    ($($square:literal),* $(,)?) => {{
        let mut bits: u64 = 0;
        $(
            let square_idx = $crate::macros::literals::square_to_index($square);
            bits |= 1u64 << square_idx;
        )*
        lumifox_chess::model::bitboard::BitBoard::new(bits)
    }};
}

/// Helper const function to parse UCI-style move notation
pub const fn parse_uci_move(uci: &str) -> (u8, u8, bool, Option<u8>) {
  let bytes = uci.as_bytes();
  if bytes.len() < 4 {
    panic!("UCI move must be at least 4 characters");
  }

  // Determine whether this is capture notation (e4xd5) or regular UCI (e2e4)
  let is_capture;
  let from: u8;
  let to: u8;
  let promotion_idx: usize;

  if bytes.len() >= 5 && bytes[2] == b'x' {
    // Capture notation: e4xd5
    let from_file = bytes[0];
    let from_rank = bytes[1];
    let to_file = bytes[3];
    let to_rank = bytes[4];

    if from_file < b'a' || from_file > b'h' || from_rank < b'1' || from_rank > b'8' {
      panic!("Invalid from square in capture notation");
    }
    if to_file < b'a' || to_file > b'h' || to_rank < b'1' || to_rank > b'8' {
      panic!("Invalid to square in capture notation");
    }

    from = (from_rank - b'1') * 8 + (from_file - b'a');
    to = (to_rank - b'1') * 8 + (to_file - b'a');
    is_capture = true;
    promotion_idx = 5;
  } else {
    // Regular UCI notation: e2e4
    let from_file = bytes[0];
    let from_rank = bytes[1];
    let to_file = bytes[2];
    let to_rank = bytes[3];

    if from_file < b'a' || from_file > b'h' || from_rank < b'1' || from_rank > b'8' {
      panic!("Invalid from square");
    }
    if to_file < b'a' || to_file > b'h' || to_rank < b'1' || to_rank > b'8' {
      panic!("Invalid to square");
    }

    from = (from_rank - b'1') * 8 + (from_file - b'a');
    to = (to_rank - b'1') * 8 + (to_file - b'a');
    is_capture = false;
    promotion_idx = 4;
  }

  // Parse promotion if present
  let promotion = if bytes.len() > promotion_idx {
    match bytes[promotion_idx] {
      b'q' | b'Q' => Some(0), // Queen
      b'r' | b'R' => Some(1), // Rook
      b'b' | b'B' => Some(2), // Bishop
      b'n' | b'N' => Some(3), // Knight
      _ => panic!("Invalid promotion piece"),
    }
  } else {
    None
  };

  (from, to, is_capture, promotion)
}
/// Compile-time UCI-style move literal: e.g. san!("e2e4"), optional promotion like "e7e8q"
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::san;
/// use lumifox_chess::model::piecemove::PieceMove;
///
/// let king_pawn = san!("e2e4");
/// let promotion = san!("e7e8q");
/// let knight_move = san!("g1f3");
/// ```
#[macro_export]
macro_rules! san {
  ($uci:literal) => {{
    const PARSED: (u8, u8, bool, Option<u8>) = $crate::macros::literals::parse_uci_move($uci);
    let promotion = match PARSED.3 {
      Some(0) => Some(lumifox_chess::model::piecemove::PromotionType::Queen),
      Some(1) => Some(lumifox_chess::model::piecemove::PromotionType::Rook),
      Some(2) => Some(lumifox_chess::model::piecemove::PromotionType::Bishop),
      Some(3) => Some(lumifox_chess::model::piecemove::PromotionType::Knight),
      _ => None,
    };
    lumifox_chess::model::piecemove::PieceMove::new(PARSED.0, PARSED.1, PARSED.2, promotion)
  }};
}
