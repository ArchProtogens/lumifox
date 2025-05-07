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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::hash::Hash;

use super::{
  bitboard::BitBoard,
  pieces::{Piece, PieceType},
};
use crate::constants::BOARD_SIZE;

pub struct Board {
  pub pieces: [Piece; BOARD_SIZE],
  pub combined: BitBoard,
  pub active_white: bool,
  pub castling: [Option<u8>; 4],
  pub halfmoves: usize,
  pub fullmoves: usize,
}

impl Default for Board {
  fn default() -> Self {
    Self::new()
  }
}

impl Board {
  pub fn new() -> Self {
    Self {
      pieces: [Piece(0); BOARD_SIZE],
      combined: BitBoard::new(),
      active_white: true,
      castling: [None; 4],
      halfmoves: 0,
      fullmoves: 1,
    }
  }

  pub fn reset(&mut self) {
    self.pieces = [Piece(0); BOARD_SIZE];
    self.combined = BitBoard::new();
    self.active_white = true;
    self.castling = [None; 4];
    self.halfmoves = 0;
    self.fullmoves = 1;
  }

  pub fn from_fen(fen: &str) -> Self {
    let mut board = Self::new();
    let mut index = 0;

    let mut fen_parts = fen.split_whitespace();
    let fen_pieces = fen_parts.next().unwrap_or("");

    for c in fen_pieces.chars() {
      let lc = c.to_ascii_lowercase();
      let colour = if c.is_ascii_uppercase() {
        super::pieces::WHITE
      } else {
        super::pieces::BLACK
      };
      match lc {
        '1'..='8' => {
          index += c.to_digit(10).unwrap() as usize;
        }
        'p' => {
          board.pieces[index] = Piece::new(super::pieces::PieceType::Pawn, colour);
          index += 1;
        }
        'r' => {
          board.pieces[index] = Piece::new(super::pieces::PieceType::Rook, colour);
          index += 1;
        }
        'n' => {
          board.pieces[index] = Piece::new(super::pieces::PieceType::Knight, colour);
          index += 1;
        }
        'b' => {
          board.pieces[index] = Piece::new(super::pieces::PieceType::Bishop, colour);
          index += 1;
        }
        'q' => {
          board.pieces[index] = Piece::new(super::pieces::PieceType::Queen, colour);
          index += 1;
        }
        'k' => {
          board.pieces[index] = Piece::new(super::pieces::PieceType::King, colour);
          index += 1;
        }
        '/' => {}
        _ => {}
      }
    }

    let active_colour = match fen_parts.next().unwrap_or("") {
      "w" => super::pieces::WHITE,
      "b" => super::pieces::BLACK,
      _ => panic!("Invalid active color in FEN string"),
    };
    board.active_white = active_colour == super::pieces::WHITE;

    let castling = fen_parts.next().unwrap_or("");
    // We now need to find the pieces that own the castling rights
    // We take for example 'K' and find from the king piece the index
    // of the rook piece, making sure they are in the same row and on the same side
    for c in castling.chars() {
      match c.to_ascii_uppercase() {
        'K' => {
          let king_index = board
            .pieces
            .iter()
            .position(|p| {
              p.piece_type() == PieceType::King && p.is_black() == c.is_ascii_lowercase()
            })
            .unwrap_or(0);
          // Now we need to find the rook piece on the left side, therefore we start by finding all rooks,
          // we then find both of their indexes and check if they are on the same row then we check if they are on the correct side
          let rook_index = board
            .pieces
            .iter()
            .enumerate()
            .filter(|(idx, p)| {
              p.piece_type() == PieceType::Rook
                && !p.is_black()
                && *idx / 8 == king_index / 8
                && *idx < king_index
            })
            .map(|(idx, _)| idx)
            .next();
          if let Some(rook_index) = rook_index {
            board.castling[0] = Some(rook_index as u8);
          }
        }
        'Q' => {
          let king_index = board
            .pieces
            .iter()
            .position(|p| {
              p.piece_type() == PieceType::King && p.is_black() == c.is_ascii_lowercase()
            })
            .unwrap_or(0);
          let rook_index = board
            .pieces
            .iter()
            .enumerate()
            .filter(|(idx, p)| {
              p.piece_type() == PieceType::Rook
                && !p.is_black()
                && *idx / 8 == king_index / 8
                && *idx > king_index
            })
            .map(|(idx, _)| idx)
            .next();
          if let Some(rook_index) = rook_index {
            board.castling[1] = Some(rook_index as u8);
          }
        }
        _ => {}
      }
    }

    let _last_move = fen_parts.next().unwrap_or("-");
    let halfmove = fen_parts.next().unwrap_or("0");
    board.halfmoves = halfmove.parse::<usize>().unwrap_or(0);
    let fullmove = fen_parts.next().unwrap_or("1");
    board.fullmoves = fullmove.parse::<usize>().unwrap_or(1);

    board
  }

  pub fn to_fen(&self) -> String {
    let mut fen = String::new();
    let mut empty_count = 0;

    for i in 0..64 {
      if self.pieces[i].piece_type() == PieceType::Empty {
        empty_count += 1;
      } else {
        if empty_count > 0 {
          fen.push_str(&empty_count.to_string());
          empty_count = 0;
        }
        fen.push(self.pieces[i].to_fen_char());
      }
      if (i + 1) % 8 == 0 && i != 63 {
        if empty_count > 0 {
          fen.push_str(&empty_count.to_string());
          empty_count = 0;
        }
        fen.push('/');
      }
    }

    if empty_count > 0 {
      fen.push_str(&empty_count.to_string());
    }

    fen.push(' ');
    fen.push(if self.active_white { 'w' } else { 'b' });
    fen.push(' ');

    for c in &self.castling {
      match c {
        Some(_) => {
          // if the castle piece is left to the king, we add 'K' to the right we add 'Q'
          let rook_index = c.unwrap();
          let rook_x = (rook_index % 8) as usize;
          let king_index = self
            .pieces
            .iter()
            .position(|p| p.piece_type() == PieceType::King && p.is_black() == self.active_white)
            .unwrap_or(0);
          let king_x = king_index % 8;
          if rook_x < king_x {
            fen.push('K');
          } else {
            fen.push('Q');
          }
        }
        None => fen.push('-'),
      }
    }

    fen.push(' ');
    fen.push('-');
    fen.push(' ');
    fen.push_str(&self.halfmoves.to_string());
    fen.push(' ');
    fen.push_str(&self.fullmoves.to_string());

    fen
  }

  #[cfg(feature = "fast_hash")]
  pub fn get_hash(&self) -> u64 {
    use std::hash::Hasher;

    let mut state = fxhash::FxHasher::default();
    self.hash(&mut state);
    state.finish()
  }
}

impl Hash for Board {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for piece in &self.pieces {
      piece.hash(state);
    }
    self.active_white.hash(state);
    self.halfmoves.hash(state);
    self.fullmoves.hash(state);
  }
}

#[cfg(test)]
mod tests {
  use super::super::pieces::PieceType;
  use super::*;

  #[test]
  fn test_starting_position() {
    let board = Board::new();
    assert!(board.active_white);
    assert_eq!(board.halfmoves, 0);
    assert_eq!(board.fullmoves, 1);
    assert!(board.castling.iter().all(|c| c.is_none()));
  }

  #[test]
  fn test_fen_parsing() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1";
    let board = Board::from_fen(fen);
    // Active color
    assert!(board.active_white);
    // Pieces
    let white_rook_idx = 56;
    assert_eq!(board.pieces[white_rook_idx].piece_type(), PieceType::Rook);
    assert!(!board.pieces[white_rook_idx].is_black());
    // Moves counts
    assert_eq!(board.halfmoves, 0);
    assert_eq!(board.fullmoves, 1);
    // Only white castling rights are set (code doesn't handle black castling)
    assert!(board.castling[0].is_some());
    assert!(board.castling[1].is_some());
    assert!(board.castling[2].is_none());
    assert!(board.castling[3].is_none());
  }

  #[test]
  fn test_fen_no_castling_and_black_to_move() {
    let fen = "8/8/8/8/8/8/8/8 b - - 10 5";
    let board = Board::from_fen(fen);
    assert!(!board.active_white);
    assert!(board.castling.iter().all(|c| c.is_none()));
    assert_eq!(board.halfmoves, 10);
    assert_eq!(board.fullmoves, 5);
  }

  #[test]
  #[should_panic(expected = "Invalid active color in FEN string")]
  fn test_invalid_active_color() {
    Board::from_fen("8/8/8/8/8/8/8/8 x - - 0 1");
  }

  #[test]
  fn test_to_fen() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1");
    let fen = board.to_fen();
    assert_eq!(
      fen,
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ-- - 0 1"
    );
  }
}
