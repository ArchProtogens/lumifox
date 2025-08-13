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

use crate::{
  legal::attack::is_square_attacked,
  model::piecemove::{PieceMove, PromotionType},
};

use super::bitboard::BitBoard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

#[derive(Clone, Copy, Debug)]
pub struct GameBoard {
  // Boards for each piece type
  pub pawns: BitBoard,
  pub knights: BitBoard,
  pub bishops: BitBoard,
  pub rooks: BitBoard,
  pub queens: BitBoard,
  pub kings: BitBoard,

  // Now for additional metadata
  pub colour: BitBoard, // BitBoard indicating which pieces are white (1) or black (0)
  pub castling: u8,
  pub en_passant: PieceMove,
  pub playing: bool, // true if it's white's turn to play
}

impl Default for GameBoard {
  fn default() -> Self {
    GameBoard {
      pawns: BitBoard::EMPTY,
      knights: BitBoard::EMPTY,
      bishops: BitBoard::EMPTY,
      rooks: BitBoard::EMPTY,
      queens: BitBoard::EMPTY,
      kings: BitBoard::EMPTY,
      colour: BitBoard::EMPTY,
      castling: 0,
      en_passant: PieceMove::NULL,
      playing: true,
    }
  }
}

impl GameBoard {
  pub fn new() -> Self {
    GameBoard::default()
  }

  pub fn reset(&mut self) {
    *self = GameBoard::new();
  }

  pub fn combined(&self) -> BitBoard {
    self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
  }

  pub fn combined_coloured(&self, desired: bool) -> BitBoard {
    self.combined() & (self.colour ^ desired)
  }

  pub fn casling_right_white(&self) -> (bool, bool) {
    (
      (self.castling & 0b0001) != 0, // White kingside
      (self.castling & 0b0010) != 0, // White queenside
    )
  }

  pub fn casling_right_black(&self) -> (bool, bool) {
    (
      (self.castling & 0b0100) != 0, // Black kingside
      (self.castling & 0b1000) != 0, // Black queenside
    )
  }

  pub(crate) fn find_king(&self, is_white: bool) -> Option<u8> {
    let king_board = if is_white {
      self.kings & self.colour
    } else {
      self.kings & !self.colour
    };

    if king_board.raw() != BitBoard::EMPTY.raw() {
      Some(king_board.raw().trailing_zeros() as u8)
    } else {
      None
    }
  }

  pub fn is_move_legal(&self, piece_move: &PieceMove) -> bool {
    let mut new_board = *self;
    new_board.move_piece(piece_move);

    // Check if the moving side's king is attacked after the move
    if let Some(king_square) = new_board.find_king(self.playing)
      && is_square_attacked(&new_board, king_square)
    {
      return false;
    }

    true
  }

  pub fn get_piece(&self, square: u8) -> Option<PieceType> {
    let boards = [
      (self.pawns, PieceType::Pawn),
      (self.knights, PieceType::Knight),
      (self.bishops, PieceType::Bishop),
      (self.rooks, PieceType::Rook),
      (self.queens, PieceType::Queen),
      (self.kings, PieceType::King),
    ];

    boards
      .iter()
      .find(|(bb, _)| bb.get_bit(square))
      .map(|(_, pt)| *pt)
  }

  pub fn clear_square(&mut self, square: u8) -> Option<PieceType> {
    // find out which piece is on `square`
    let piece = self.get_piece(square)?;
    // pick the matching bitboard
    let board = match piece {
      PieceType::Pawn => &mut self.pawns,
      PieceType::Knight => &mut self.knights,
      PieceType::Bishop => &mut self.bishops,
      PieceType::Rook => &mut self.rooks,
      PieceType::Queen => &mut self.queens,
      PieceType::King => &mut self.kings,
    };
    // clear it
    board.unset_bit(square);
    self.colour.unset_bit(square);
    Some(piece)
  }

  pub fn set_square(&mut self, square: u8, piece_type: PieceType, is_white: bool) {
    // Clear the square first
    self.clear_square(square);
    let bitboard = match piece_type {
      PieceType::Pawn => &mut self.pawns,
      PieceType::Knight => &mut self.knights,
      PieceType::Bishop => &mut self.bishops,
      PieceType::Rook => &mut self.rooks,
      PieceType::Queen => &mut self.queens,
      PieceType::King => &mut self.kings,
    };

    bitboard.set_bit(square);
    self.colour.update_bit(square, is_white);
  }

  pub fn move_piece(&mut self, piece_move: &PieceMove) {
    if !self.is_move_legal(piece_move) {
      panic!("Illegal move attempted: {piece_move:?}");
    }

    // Update the board state based on the piece move
    let from_square = piece_move.from_square();
    let to_square = piece_move.to_square();

    // Remove the piece from the from_square
    let piece = self
      .clear_square(from_square)
      .expect("No piece found at from_square");

    // Handle special cases like en passant, promotion, etc.
    if piece == PieceType::Pawn {
      // Handle en passant capture
      // If the move is a diagonal move (difference in file is 1) and there is no piece on the target square,
      // it must be an en passant capture.
      let from_file = from_square % 8;
      let to_file = to_square % 8;
      let from_rank = from_square / 8;
      let to_rank = to_square / 8;

      if (from_file as i8 - to_file as i8).abs() == 1
        && (from_rank as i8 - to_rank as i8).abs() == 1
      {
        // Check if there is no piece on the target square (normal capture would have a piece there)
        if self.clear_square(to_square).is_none() {
          // Remove the captured pawn (en passant)
          let captured_pawn_square = if self.playing {
            // White just moved, so captured pawn is one rank below
            to_square - 8
          } else {
            // Black just moved, so captured pawn is one rank above
            to_square + 8
          };
          self.clear_square(captured_pawn_square);
        }
      }
    }

    if piece_move.is_promotion() {
      // Handle promotion
      let promotion_type = piece_move.promotion_type().expect("Promotion type not set");
      self.set_square(
        to_square,
        match promotion_type {
          PromotionType::Queen => PieceType::Queen,
          PromotionType::Rook => PieceType::Rook,
          PromotionType::Bishop => PieceType::Bishop,
          PromotionType::Knight => PieceType::Knight,
        },
        self.playing,
      );
    } else {
      // Place the piece on the to_square
      self.set_square(to_square, piece, self.playing);
    }

    self.playing = !self.playing; // Switch turn
  }
}
