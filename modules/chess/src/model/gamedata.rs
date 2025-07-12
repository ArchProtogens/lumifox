/*
 * A simple and growing chess library in Rust.
 * Copyright (C) 2025  Clifton Toaster Reid
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{
  errors::FenParseError,
  model::{gameboard::GameBoard, piecemove::PieceMove},
};

pub const MAX_GAME_MOVES: usize = 1024;

#[derive(Clone, Copy, Debug)]
pub struct GameData {
  pub board: GameBoard,
  pub moves: [PieceMove; MAX_GAME_MOVES],
  pub plies: usize,
  pub halfmove_clock: usize,
}

impl Default for GameData {
  fn default() -> Self {
    Self {
      board: Default::default(),
      moves: [PieceMove::NULL; MAX_GAME_MOVES],
      plies: Default::default(),
      halfmove_clock: Default::default(),
    }
  }
}

impl GameData {
  pub fn white_plies(&self) -> usize {
    (self.plies + 1) >> 1
  }

  pub fn black_plies(&self) -> usize {
    self.plies >> 1
  }

  pub fn from_fen(fen: &str) -> Result<Self, FenParseError> {
    let mut parts = fen.split_whitespace();
    let placement = parts.next().ok_or(FenParseError::MalformedFen)?;
    let active_color = parts.next().ok_or(FenParseError::MalformedFen)?;
    let castling = parts.next().ok_or(FenParseError::MalformedFen)?;
    let en_passant = parts.next().ok_or(FenParseError::MalformedFen)?;
    let halfmove_clock = parts.next().ok_or(FenParseError::MalformedFen)?;
    let fullmove_number = parts.next().ok_or(FenParseError::MalformedFen)?;

    if parts.next().is_some() {
      return Err(FenParseError::MalformedFen);
    }

    let mut i = 0;
    let mut squares = 0;
    let mut ranks = 0;

    let mut board = GameBoard::default();

    // 1. Piece placement
    for c in placement.chars() {
      match c {
        '1'..='8' => {
          let empty_squares = c.to_digit(10).unwrap() as usize;
          i += empty_squares;
          squares += empty_squares;
        }
        'P' | 'p' | 'N' | 'n' | 'B' | 'b' | 'R' | 'r' | 'Q' | 'q' | 'K' | 'k' => {
          // Convert FEN board position to square index
          // FEN reads from rank 8 to rank 1, but our bitboard has rank 1 at squares 0-7
          let rank = 7 - (i / 8); // Convert from FEN rank order to bitboard rank order
          let file = i % 8;
          let square_index = (rank * 8 + file) as u8;

          let is_white = c.is_ascii_uppercase();
          let piece_char_lower = c.to_ascii_lowercase();

          match piece_char_lower {
            'p' => {
              board.pawns.set_bit(square_index);
            }
            'n' => {
              board.knights.set_bit(square_index);
            }
            'b' => {
              board.bishops.set_bit(square_index);
            }
            'r' => {
              board.rooks.set_bit(square_index);
            }
            'q' => {
              board.queens.set_bit(square_index);
            }
            'k' => {
              board.kings.set_bit(square_index);
            }
            _ => return Err(FenParseError::InvalidPieceChar), // Should not be reached with exhaustive match
          }

          if is_white {
            board.colour.set_bit(square_index);
          } else {
            board.colour.unset_bit(square_index);
          }
          i += 1;
          squares += 1;
        }
        '/' => {
          // Validate that the current rank has exactly 8 squares
          if squares != 8 {
            return Err(FenParseError::InvalidRankLength);
          }
          // Reset squares_in_current_rank for the new rank
          squares = 0;
          // Increment ranks_processed counter
          ranks += 1;
        }
        _ => return Err(FenParseError::UnexpectedCharacter),
      }
    }
    if ranks != 7 {
      return Err(FenParseError::InvalidRankCount);
    }
    if squares != 8 {
      return Err(FenParseError::InvalidRankLength);
    }

    // 2. Active colour
    if (active_color.len() != 1) || !matches!(active_color, "w" | "b") {
      return Err(FenParseError::InvalidActiveColor);
    }
    match active_color {
      "w" => board.playing = true,
      "b" => board.playing = false,
      _ => return Err(FenParseError::InvalidActiveColor), // Should not be reached with exhaustive match
    }

    // 3. Castling availability
    if castling.len() > 4 {
      return Err(FenParseError::InvalidCastling);
    }
    for c in castling.chars() {
      match c {
        'K' => board.castling |= 0b0001, // White kingside
        'Q' => board.castling |= 0b0010, // White queenside
        'k' => board.castling |= 0b0100, // Black kingside
        'q' => board.castling |= 0b1000, // Black queenside
        '-' => continue,                 // No castling rights
        _ => return Err(FenParseError::InvalidCastlingChar),
      }
    }

    // 4. En passant target square
    if en_passant.len() > 2 || en_passant.is_empty() {
      return Err(FenParseError::InvalidEnPassantSquare);
    }
    if en_passant != "-" {
      let mut chars = en_passant.chars();
      let col = chars.next().ok_or(FenParseError::InvalidEnPassantSquare)?;
      let row = chars.next().ok_or(FenParseError::InvalidEnPassantSquare)?;

      let col_nbr = match col {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => col as u8 - b'a' - 1,
        _ => return Err(FenParseError::InvalidEnPassantSquare),
      };
      let row_nbr = match row {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => 7 - (row as u8 - b'1'),
        _ => return Err(FenParseError::InvalidEnPassantSquare),
      };

      if col_nbr > 7 || row_nbr > 7 {
        return Err(FenParseError::InvalidEnPassantSquare);
      }
      // Validate that en passant square is on rank 3 or 6
      if row_nbr != 2 && row_nbr != 5 {
        return Err(FenParseError::InvalidEnPassantSquare);
      }
      let square_index = row_nbr * 8 + col_nbr;

      if board.en_passant != PieceMove::NULL {
        return Err(FenParseError::InvalidEnPassant);
      }
      board.en_passant = PieceMove::new(0, square_index, false, None);
    }

    // 5. Halfmove clock
    if halfmove_clock.is_empty() {
      return Err(FenParseError::InvalidHalfmoveClock);
    }
    let clock: usize = halfmove_clock
      .parse()
      .map_err(|_| FenParseError::ExpectedNumber)?;

    // 6. Fullmove number
    if fullmove_number.is_empty() {
      return Err(FenParseError::InvalidFullmoveNumber);
    }
    let count: usize = fullmove_number
      .parse()
      .map_err(|_| FenParseError::ExpectedNumber)?;
    if count == 0 {
      return Err(FenParseError::InvalidFullmoveNumber);
    }

    Ok(Self {
      board,
      moves: [PieceMove::NULL; MAX_GAME_MOVES],
      plies: (count - 1) * 2 + if active_color == "b" { 1 } else { 0 },
      halfmove_clock: clock,
    })
  }

  // Add this method to the `impl GameData` block in gamedata.ranks
  #[cfg(feature = "std")]
  pub fn to_fen(&self) -> String {
    let mut fen = String::new();

    // 1. Piece placement
    for rank in 0..8 {
      let mut empty_count = 0;
      for file in 0..8 {
        let square = rank * 8 + file;
        let piece_char = self.get_piece_char(square as u8);

        if let Some(c) = piece_char {
          if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
            empty_count = 0;
          }
          fen.push(c);
        } else {
          empty_count += 1;
        }
      }

      // Add any remaining empty squares at end of rank
      if empty_count > 0 {
        fen.push_str(&empty_count.to_string());
      }

      // Add rank separator (unless last rank)
      if rank < 7 {
        fen.push('/');
      }
    }

    fen.push(' ');

    // 2. Active color
    fen.push(if self.board.playing { 'w' } else { 'b' });
    fen.push(' ');

    // 3. Castling availability
    let mut castling_str = String::new();
    if self.board.castling & 0b0001 != 0 {
      castling_str.push('K');
    }
    if self.board.castling & 0b0010 != 0 {
      castling_str.push('Q');
    }
    if self.board.castling & 0b0100 != 0 {
      castling_str.push('k');
    }
    if self.board.castling & 0b1000 != 0 {
      castling_str.push('q');
    }

    if castling_str.is_empty() {
      fen.push('-');
    } else {
      fen.push_str(&castling_str);
    }
    fen.push(' ');

    // 4. En passant target square
    if self.board.en_passant == PieceMove::NULL {
      fen.push('-');
    } else {
      let sq = self.board.en_passant.to_square();
      let file = (sq % 8) as u8 + 1;
      let rank = 8 - (sq / 8);
      fen.push((b'a' + file) as char);
      fen.push((b'0' + rank as u8) as char);
    }
    fen.push(' ');

    // 5. Halfmove clock
    fen.push_str(&self.halfmove_clock.to_string());
    fen.push(' ');

    // 6. Fullmove number
    let fullmove = (self.plies / 2) + 1;
    fen.push_str(&fullmove.to_string());

    fen
  }

  // Helper function to get piece character at a square
  fn get_piece_char(&self, square: u8) -> Option<char> {
    if self.board.pawns.get_bit(square) {
      Some(if self.board.colour.get_bit(square) {
        'P'
      } else {
        'p'
      })
    } else if self.board.knights.get_bit(square) {
      Some(if self.board.colour.get_bit(square) {
        'N'
      } else {
        'n'
      })
    } else if self.board.bishops.get_bit(square) {
      Some(if self.board.colour.get_bit(square) {
        'B'
      } else {
        'b'
      })
    } else if self.board.rooks.get_bit(square) {
      Some(if self.board.colour.get_bit(square) {
        'R'
      } else {
        'r'
      })
    } else if self.board.queens.get_bit(square) {
      Some(if self.board.colour.get_bit(square) {
        'Q'
      } else {
        'q'
      })
    } else if self.board.kings.get_bit(square) {
      Some(if self.board.colour.get_bit(square) {
        'K'
      } else {
        'k'
      })
    } else {
      None
    }
  }

  #[cfg(feature = "std")]
  pub fn print_board(&self) {
    // Print ranks 8 down to 1
    for rank in (0..8).rev() {
      for file in 0..8 {
        let sq = (rank * 8 + file) as u8;
        if let Some(c) = self.get_piece_char(sq) {
          // White pieces in bright white, black pieces in yellow
          if c.is_ascii_uppercase() {
            print!("\x1b[97m{}\x1b[0m ", c);
          } else {
            print!("\x1b[33m{}\x1b[0m ", c);
          }
        } else {
          // Empty square
          print!(". ");
        }
      }
      println!();
    }
  }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
  use super::*;
  use crate::errors::FenParseError;

  /// Helper function to test FEN round-tripping.
  /// It parses a FEN, generates a new FEN from the result,
  /// and asserts they are identical.
  fn fen_roundtrip_test(fen: &str) {
    let gamedata = GameData::from_fen(fen).expect(&format!("FEN parsing failed for: {}", fen));
    let new_fen = gamedata.to_fen();
    assert_eq!(fen, new_fen);
  }

  #[test]
  fn test_fen_roundtrip_startpos() {
    fen_roundtrip_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
  }

  #[test]
  fn test_fen_roundtrip_kiwipete() {
    // A complex mid-game position
    fen_roundtrip_test("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  }

  #[test]
  fn test_fen_roundtrip_en_passant_white() {
    // Position with en passant square e3 available to black
    fen_roundtrip_test("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e3 0 2");
  }

  #[test]
  fn test_fen_roundtrip_en_passant_black() {
    // Position with en passant square f6 available to white
    fen_roundtrip_test("rnbqkbnr/pppp2pp/8/4pp2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
  }

  #[test]
  fn test_fen_roundtrip_black_to_move() {
    fen_roundtrip_test("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
  }

  #[test]
  fn test_fen_roundtrip_no_castling() {
    fen_roundtrip_test("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b - - 1 1");
  }

  #[test]
  fn test_fen_roundtrip_endgame() {
    fen_roundtrip_test("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 55");
  }

  #[test]
  fn test_fen_roundtrip_only_kings() {
    fen_roundtrip_test("8/k7/8/8/8/8/7K/8 w - - 0 1");
  }

  // --- Tests for Invalid FENs ---

  #[test]
  fn test_from_fen_invalid_piece() {
    assert_eq!(
      GameData::from_fen("rnbqkbnr/ppppTppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap_err(),
      FenParseError::UnexpectedCharacter
    );
  }

  #[test]
  fn test_from_fen_invalid_rank_length_too_long() {
    // 9 pawns on a rank
    assert_eq!(
      GameData::from_fen("rnbqkbnr/ppppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap_err(),
      FenParseError::InvalidRankLength
    );
  }

  #[test]
  fn test_from_fen_invalid_rank_length_too_short() {
    // rank with 7 squares
    assert_eq!(
      GameData::from_fen("rnbqkbnr/ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap_err(),
      FenParseError::InvalidRankLength
    );
  }

  #[test]
  fn test_from_fen_invalid_rank_count() {
    // Missing a rank
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap_err(),
      FenParseError::InvalidRankCount
    );
  }

  #[test]
  fn test_from_fen_invalid_active_color() {
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1").unwrap_err(),
      FenParseError::InvalidActiveColor
    );
  }

  #[test]
  fn test_from_fen_invalid_castling_char() {
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQXkq - 0 1").unwrap_err(),
      FenParseError::InvalidCastling
    );
  }

  #[test]
  fn test_from_fen_invalid_en_passant_square() {
    // Invalid square "i9"
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq i9 0 1").unwrap_err(),
      FenParseError::InvalidEnPassantSquare
    );
  }

  #[test]
  fn test_from_fen_invalid_en_passant_rank() {
    // En passant can only be on rank 3 or 6
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e4 0 1").unwrap_err(),
      FenParseError::InvalidEnPassantSquare
    );
  }

  #[test]
  fn test_from_fen_invalid_halfmove_clock() {
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - a 1").unwrap_err(),
      FenParseError::ExpectedNumber
    );
  }

  #[test]
  fn test_from_fen_invalid_fullmove_number() {
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 b").unwrap_err(),
      FenParseError::ExpectedNumber
    );
  }

  #[test]
  fn test_from_fen_zero_fullmove_number() {
    // Fullmove number must be 1 or greater
    assert_eq!(
      GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0").unwrap_err(),
      FenParseError::InvalidFullmoveNumber
    );
  }
}
