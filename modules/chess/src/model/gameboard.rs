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
  /// Check that all squares between `from` and `to` are empty (exclusive).
  fn is_path_clear(&self, from: u8, to: u8) -> bool {
    let from_rank = (from / 8) as i8;
    let from_file = (from % 8) as i8;
    let to_rank = (to / 8) as i8;
    let to_file = (to % 8) as i8;
    let dr = (to_rank - from_rank).signum();
    let df = (to_file - from_file).signum();
    let mut r = from_rank + dr;
    let mut f = from_file + df;
    while r != to_rank || f != to_file {
      let sq = (r * 8 + f) as u8;
      if self.combined().get_bit(sq) {
        return false;
      }
      r += dr;
      f += df;
    }
    true
  }

  pub fn is_move_legal(&self, piece_move: &PieceMove) -> bool {
    // 1. Is the piece being moved the correct color for the current turn?
    if !self.is_correct_turn_piece(piece_move) {
      #[cfg(feature = "std")]
      eprintln!("Failed: is_correct_turn_piece");
      return false;
    }

    // 2. Is the move type allowed for the piece?
    if !self.is_piece_move_valid(piece_move) {
      #[cfg(feature = "std")]
      eprintln!("Failed: is_piece_move_valid");
      return false;
    }

    // 3. Is the destination square occupied by a friendly piece?
    if !self.is_destination_valid(piece_move) {
      #[cfg(feature = "std")]
      eprintln!("Failed: is_destination_valid");
      return false;
    }

    // 4. Check special moves (castling, en passant, promotion)
    if !self.are_special_moves_valid(piece_move) {
      #[cfg(feature = "std")]
      eprintln!("Failed: are_special_moves_valid");
      return false;
    }

    // 5. Is the move not leaving the moving side's king in check?
    if !self.does_not_leave_king_in_check(piece_move) {
      #[cfg(feature = "std")]
      eprintln!("Failed: does_not_leave_king_in_check");
      return false;
    }

    true
  }

  /// Check if the piece being moved belongs to the player whose turn it is
  fn is_correct_turn_piece(&self, piece_move: &PieceMove) -> bool {
    self.colour.get_bit(piece_move.from_square()) == self.playing
  }

  /// Check if the destination square is valid (not occupied by friendly piece, not capturing king)
  fn is_destination_valid(&self, piece_move: &PieceMove) -> bool {
    let to = piece_move.to_square();

    // Cannot move to square occupied by friendly piece
    if let Some(_) = self.get_piece(to)
      && self.colour.get_bit(to) == self.playing
    {
      return false;
    }

    // Rule 9: It's illegal to capture the opponent's king
    if let Some(PieceType::King) = self.get_piece(to) {
      return false;
    }

    true
  }

  /// Check if the piece move follows the movement rules for that piece type
  fn is_piece_move_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let piece_type = match self.get_piece(from) {
      Some(pt) => pt,
      None => return false,
    };

    match piece_type {
      PieceType::Pawn => self.is_pawn_move_valid(piece_move),
      PieceType::Knight => self.is_knight_move_valid(from, to),
      PieceType::Bishop => self.is_bishop_move_valid(from, to),
      PieceType::Rook => self.is_rook_move_valid(from, to),
      PieceType::Queen => self.is_queen_move_valid(from, to),
      PieceType::King => self.is_king_move_valid(piece_move),
    }
  }

  /// Check if a pawn move is valid
  fn is_pawn_move_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let from_rank = from / 8;
    let to_rank = to / 8;
    let from_file = from % 8;
    let to_file = to % 8;

    let is_forward = (self.playing && to > from) || (!self.playing && from > to);
    let is_capture = self.get_piece(to).is_some() && self.colour.get_bit(to) != self.playing;
    let is_en_passant = piece_move.is_en_passant();
    let is_promotion = piece_move.is_promotion();

    // Check if move direction is forward
    if !is_forward {
      return false;
    }

    // Straight move (forward)
    if from_file == to_file {
      return self.is_pawn_forward_move_valid(from, to, from_rank, to_rank, is_promotion);
    }
    // Diagonal move (capture or en passant)
    else if (from_file as i8 - to_file as i8).abs() == 1 {
      return self.is_pawn_diagonal_move_valid(piece_move, is_capture, is_en_passant, to_rank);
    }

    false
  }

  /// Check if a pawn forward move is valid
  fn is_pawn_forward_move_valid(
    &self,
    from: u8,
    to: u8,
    from_rank: u8,
    to_rank: u8,
    is_promotion: bool,
  ) -> bool {
    let diff = to.abs_diff(from);

    if diff == 8 {
      // Single step forward
      if self.get_piece(to).is_some() {
        return false; // Blocked
      }
    } else if diff == 16 && ((from_rank == 1 && self.playing) || (from_rank == 6 && !self.playing))
    {
      // Double step from starting rank
      let mid = if self.playing { from + 8 } else { from - 8 };
      if self.get_piece(to).is_some() || self.get_piece(mid).is_some() {
        return false; // Blocked
      }
    } else {
      return false; // Invalid forward move
    }

    // Check promotion requirements
    self.is_pawn_promotion_valid(to_rank, is_promotion)
  }

  /// Check if a pawn diagonal move (capture/en passant) is valid
  fn is_pawn_diagonal_move_valid(
    &self,
    piece_move: &PieceMove,
    is_capture: bool,
    is_en_passant: bool,
    to_rank: u8,
  ) -> bool {
    if !(is_capture || is_en_passant) {
      return false; // Diagonal moves must be captures
    }

    // Check promotion requirements for diagonal moves
    self.is_pawn_promotion_valid(to_rank, piece_move.is_promotion())
  }

  /// Check if pawn promotion is handled correctly
  fn is_pawn_promotion_valid(&self, to_rank: u8, is_promotion: bool) -> bool {
    let should_promote = (to_rank == 7 && self.playing) || (to_rank == 0 && !self.playing);

    if should_promote && !is_promotion {
      return false; // Must promote when reaching last rank
    }

    if !should_promote && is_promotion {
      return false; // Cannot promote when not on last rank
    }

    true
  }

  /// Check if a knight move is valid
  fn is_knight_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;
    (dr.abs() == 2 && df.abs() == 1) || (dr.abs() == 1 && df.abs() == 2)
  }

  /// Check if a bishop move is valid
  fn is_bishop_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;

    if dr.abs() != df.abs() {
      return false; // Not diagonal
    }

    self.is_path_clear(from, to)
  }

  /// Check if a rook move is valid
  fn is_rook_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;

    if dr != 0 && df != 0 {
      return false; // Not straight
    }

    self.is_path_clear(from, to)
  }

  /// Check if a queen move is valid
  fn is_queen_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;

    let is_diagonal = dr.abs() == df.abs();
    let is_straight = dr == 0 || df == 0;

    if !(is_diagonal || is_straight) {
      return false; // Queen must move diagonally or straight
    }

    self.is_path_clear(from, to)
  }

  /// Check if a king move is valid (including castling)
  fn is_king_move_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;

    // Normal king move (one square in any direction)
    if dr.abs() <= 1 && df.abs() <= 1 {
      return true;
    }

    // Castling (two squares horizontally)
    if dr == 0 && df.abs() == 2 {
      return self.is_castling_valid(piece_move);
    }

    false
  }

  /// Check if castling is valid
  fn is_castling_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let is_kingside = to == from + 2;

    // Check castling rights
    let (can_k, can_q) = if self.playing {
      self.casling_right_white()
    } else {
      self.casling_right_black()
    };

    if (is_kingside && !can_k) || (!is_kingside && !can_q) {
      return false;
    }

    // Check if intervening squares are empty
    if !self.are_castling_squares_clear(from, is_kingside) {
      return false;
    }

    // Check if king doesn't move through or into check
    self.is_castling_path_safe(from, is_kingside)
  }

  /// Check if squares between king and rook are clear for castling
  fn are_castling_squares_clear(&self, from: u8, is_kingside: bool) -> bool {
    if is_kingside {
      for sq in [from + 1, from + 2] {
        if self.combined().get_bit(sq) {
          return false;
        }
      }
    } else {
      for sq in [from - 1, from - 2, from - 3] {
        if self.combined().get_bit(sq) {
          return false;
        }
      }
    }
    true
  }

  /// Check if king doesn't move through check during castling
  fn is_castling_path_safe(&self, from: u8, is_kingside: bool) -> bool {
    let path = if is_kingside {
      [from, from + 1, from + 2]
    } else {
      [from, from - 1, from - 2]
    };

    for &sq in &path {
      if is_square_attacked(self, sq) {
        return false;
      }
    }
    true
  }

  /// Check if special moves are valid
  fn are_special_moves_valid(&self, piece_move: &PieceMove) -> bool {
    // En passant validation - only treat as en passant if there's no piece on target square
    if piece_move.is_en_passant() && self.get_piece(piece_move.to_square()).is_none() {
      return self.is_en_passant_valid(piece_move);
    }

    true
  }

  /// Check if en passant move is valid
  fn is_en_passant_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let ep_square = self.en_passant.to_square();

    // En passant target square must match the move
    if ep_square != to {
      return false;
    }

    // Check move geometry
    let from_file = from % 8;
    let to_file = to % 8;
    let from_rank = from / 8;
    let to_rank = to / 8;

    let correct_forward = if self.playing {
      to_rank == from_rank + 1
    } else {
      to_rank + 1 == from_rank
    };

    if (from_file as i8 - to_file as i8).abs() != 1 || !correct_forward {
      return false;
    }

    // Ensure no piece on target square (en passant doesn't capture there)
    if self.get_piece(to).is_some() {
      return false;
    }

    // Ensure the captured pawn exists and is opponent's
    let captured_pawn_square = if self.playing { to - 8 } else { to + 8 };
    if self.get_piece(captured_pawn_square) != Some(PieceType::Pawn)
      || self.colour.get_bit(captured_pawn_square) == self.playing
    {
      return false;
    }

    true
  }

  /// Check if the move doesn't leave the moving side's king in check
  fn does_not_leave_king_in_check(&self, piece_move: &PieceMove) -> bool {
    let mut new_board = *self;
    new_board.apply_move_unchecked(piece_move);

    if let Some(king_square) = new_board.find_king(self.playing) {
      !is_square_attacked(&new_board, king_square)
    } else {
      false // No king found - invalid position
    }
  }

  /// Apply a move to the board without any legality checks.
  /// Intended for internal use (e.g., simulation inside `is_move_legal`).
  /// NOTE: This does NOT switch turns - the caller is responsible for that.
  fn apply_move_unchecked(&mut self, piece_move: &PieceMove) {
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
      let from_file = from_square % 8;
      let to_file = to_square % 8;
      let from_rank = from_square / 8;
      let to_rank = to_square / 8;

      if (from_file as i8 - to_file as i8).abs() == 1
        && (from_rank as i8 - to_rank as i8).abs() == 1
      {
        // If there is no piece on the target square, it's en passant -> remove the captured pawn
        if self.clear_square(to_square).is_none() {
          let captured_pawn_square = if self.playing {
            to_square - 8
          } else {
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

    // NOTE: We don't switch turns here - the caller is responsible for that
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
    self.apply_move_unchecked(piece_move);
    self.playing = !self.playing; // Switch turn after applying the move
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::constants::*;
  use crate::model::{gamedata::GameData, piecemove::PromotionType};

  fn board_from_fen(fen: &str) -> GameBoard {
    GameData::from_fen(fen).unwrap().board
  }

  // Helper function to create simple moves
  fn simple_move(from: u8, to: u8) -> PieceMove {
    PieceMove::new(from, to, false, None)
  }

  fn capture_move(from: u8, to: u8) -> PieceMove {
    PieceMove::new(from, to, true, None)
  }

  fn promotion_move(from: u8, to: u8, promotion: PromotionType) -> PieceMove {
    PieceMove::new(from, to, false, Some(promotion))
  }

  fn promotion_capture_move(from: u8, to: u8, promotion: PromotionType) -> PieceMove {
    PieceMove::new(from, to, true, Some(promotion))
  }

  fn en_passant_move(from: u8, to: u8) -> PieceMove {
    PieceMove::new_en_passant(from, to)
  }

  fn castling_move(from: u8, to: u8) -> PieceMove {
    PieceMove::new_castling(from, to)
  }

  // Basic validity tests
  #[test]
  fn test_wrong_color_piece() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // Try to move black pawn when it's white's turn
    let black_pawn_move = simple_move(A7, A6);
    assert!(!board.is_move_legal(&black_pawn_move));
  }

  #[test]
  fn test_move_to_own_piece() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // Try to move white pawn to square occupied by white pawn
    let move_to_own = simple_move(A2, B2);
    assert!(!board.is_move_legal(&move_to_own));
  }

  #[test]
  fn test_no_piece_to_move() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // Try to move from empty square
    let empty_square_move = simple_move(E4, E5);
    assert!(!board.is_move_legal(&empty_square_move));
  }

  #[test]
  fn test_cannot_capture_king() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/4Q3/PPPPPPPP/RNB1KBNR w KQkq - 0 1");
    // Try to capture the black king with the white queen
    let capture_king = capture_move(E3, E8);
    assert!(!board.is_move_legal(&capture_king));
  }

  // Pawn move tests
  #[test]
  fn test_pawn_single_step() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let pawn_move = simple_move(E2, E3);
    assert!(board.is_move_legal(&pawn_move));
  }

  #[test]
  fn test_pawn_double_step() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let pawn_double = simple_move(E2, E4);
    assert!(board.is_move_legal(&pawn_double));
  }

  #[test]
  fn test_pawn_double_step_blocked() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    // Black pawn can't double step because E6 is blocked
    let blocked_double = simple_move(E7, E5);
    assert!(board.is_move_legal(&blocked_double));
  }

  #[test]
  fn test_pawn_invalid_double_step_from_wrong_rank() {
    let board = board_from_fen("rnbqkbnr/pppp1ppp/4p3/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // Black pawn on E6 can't double step
    let invalid_double = simple_move(E6, E4);
    assert!(!board.is_move_legal(&invalid_double));
  }

  #[test]
  fn test_pawn_capture_diagonal() {
    let board = board_from_fen("rnbqkbnr/pppp1ppp/8/8/3p4/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
    let pawn_capture = capture_move(E3, D4);
    assert!(board.is_move_legal(&pawn_capture));
  }

  #[test]
  fn test_pawn_cannot_capture_forward() {
    let board = board_from_fen("rnbqkbnr/pppp1ppp/8/8/4p3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
    let pawn_forward_capture = capture_move(E2, E4);
    assert!(!board.is_move_legal(&pawn_forward_capture));
  }

  #[test]
  fn test_pawn_cannot_move_diagonal_without_capture() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let diagonal_move = simple_move(E2, D3);
    assert!(!board.is_move_legal(&diagonal_move));
  }

  #[test]
  fn test_pawn_backward_move_illegal() {
    let board = board_from_fen("rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let backward_move = simple_move(E5, E6);
    assert!(!board.is_move_legal(&backward_move));
  }

  #[test]
  fn test_pawn_promotion_to_queen() {
    let board = board_from_fen("k7/4P3/8/8/8/8/8/7K w - - 0 1");
    let promotion = promotion_move(E7, E8, PromotionType::Queen);
    assert!(board.is_move_legal(&promotion));
  }

  #[test]
  fn test_pawn_promotion_all_pieces() {
    let board = board_from_fen("k7/4P3/8/8/8/8/8/K7 w - - 0 1");
    assert!(board.is_move_legal(&promotion_move(E7, E8, PromotionType::Queen)));
    assert!(board.is_move_legal(&promotion_move(E7, E8, PromotionType::Rook)));
    assert!(board.is_move_legal(&promotion_move(E7, E8, PromotionType::Bishop)));
    assert!(board.is_move_legal(&promotion_move(E7, E8, PromotionType::Knight)));
  }

  #[test]
  fn test_pawn_promotion_capture() {
    let board = board_from_fen("k2n4/4P3/8/8/8/8/8/K7 w - - 0 1");
    let promotion_capture = promotion_capture_move(E7, D8, PromotionType::Queen);
    assert!(board.is_move_legal(&promotion_capture));
  }

  #[test]
  fn test_pawn_promotion_wrong_rank() {
    let board = board_from_fen("8/8/4P3/8/8/8/8/8 w - - 0 1");
    let wrong_rank_promotion = promotion_move(E6, E7, PromotionType::Queen);
    assert!(!board.is_move_legal(&wrong_rank_promotion));
  }

  #[test]
  fn test_black_pawn_promotion() {
    let board = board_from_fen("K6k/8/8/8/8/8/4p3/8 b - - 0 1");
    let black_promotion = promotion_move(E2, E1, PromotionType::Queen);
    assert!(board.is_move_legal(&black_promotion));
  }

  // En passant tests
  #[test]
  fn test_en_passant_basic() {
    let mut board = board_from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1");
    board.en_passant = PieceMove::new(D5, D6, false, None); // Set en passant target
    let en_passant = en_passant_move(E5, D6);
    assert!(board.is_move_legal(&en_passant));
  }

  #[test]
  fn test_en_passant_wrong_target() {
    let mut board = board_from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
    board.en_passant = PieceMove::new(D5, C6, false, None); // Wrong en passant target
    let en_passant = en_passant_move(E5, D6); // Try to capture to different square
    assert!(!board.is_move_legal(&en_passant));
  }

  // Knight move tests
  #[test]
  fn test_knight_l_shape_moves() {
    let board = board_from_fen("k7/8/8/8/3N4/8/8/K7 w - - 0 1");
    let knight_moves = [
      simple_move(D4, B3),
      simple_move(D4, B5),
      simple_move(D4, C2),
      simple_move(D4, C6),
      simple_move(D4, E2),
      simple_move(D4, E6),
      simple_move(D4, F3),
      simple_move(D4, F5),
    ];
    for &knight_move in &knight_moves {
      assert!(
        board.is_move_legal(&knight_move),
        "Knight move {:?} should be legal",
        knight_move
      );
    }
  }

  #[test]
  fn test_knight_invalid_move() {
    let board = board_from_fen("8/8/8/8/3N4/8/8/8 w - - 0 1");
    let invalid_knight = simple_move(D4, E5); // Not an L-shape
    assert!(!board.is_move_legal(&invalid_knight));
  }

  #[test]
  fn test_knight_blocked_by_own_piece() {
    let board = board_from_fen("8/8/8/2P5/3N4/8/8/8 w - - 0 1");
    let blocked_knight = simple_move(D4, C6);
    assert!(!board.is_move_legal(&blocked_knight));
  }

  #[test]
  fn test_knight_capture() {
    let board = board_from_fen("8/k7/8/2p5/3N4/8/2K5/8 w - - 0 1");
    let knight_capture = capture_move(D4, C6);
    assert!(board.is_move_legal(&knight_capture));
  }

  // Bishop move tests
  #[test]
  fn test_bishop_diagonal_moves() {
    let board = board_from_fen("k7/8/8/8/3B4/8/8/7K w - - 0 1");
    let bishop_moves = [
      simple_move(D4, A1),
      simple_move(D4, B2),
      simple_move(D4, C3),
      simple_move(D4, E5),
      simple_move(D4, F6),
      simple_move(D4, G7),
      simple_move(D4, H8),
      simple_move(D4, A7),
      simple_move(D4, B6),
      simple_move(D4, C5),
      simple_move(D4, E3),
      simple_move(D4, F2),
      simple_move(D4, G1),
    ];
    for &bishop_move in &bishop_moves {
      assert!(
        board.is_move_legal(&bishop_move),
        "Bishop move {:?} should be legal",
        bishop_move
      );
    }
  }

  #[test]
  fn test_bishop_non_diagonal_illegal() {
    let board = board_from_fen("8/8/8/8/3B4/8/8/8 w - - 0 1");
    let non_diagonal = simple_move(D4, D6); // Vertical move
    assert!(!board.is_move_legal(&non_diagonal));
  }

  #[test]
  fn test_bishop_blocked_path() {
    let board = board_from_fen("8/8/8/2P5/3B4/8/8/8 w - - 0 1");
    let blocked_bishop = simple_move(D4, A7);
    assert!(!board.is_move_legal(&blocked_bishop)); // Path blocked by pawn on C5
  }

  // Rook move tests
  #[test]
  fn test_rook_straight_moves() {
    let board = board_from_fen("k7/8/8/8/3R4/8/8/7K w - - 0 1");
    let rook_moves = [
      simple_move(D4, D1),
      simple_move(D4, D2),
      simple_move(D4, D3),
      simple_move(D4, D5),
      simple_move(D4, D6),
      simple_move(D4, D7),
      simple_move(D4, D8),
      simple_move(D4, A4),
      simple_move(D4, B4),
      simple_move(D4, C4),
      simple_move(D4, E4),
      simple_move(D4, F4),
      simple_move(D4, G4),
      simple_move(D4, H4),
    ];
    for &rook_move in &rook_moves {
      assert!(
        board.is_move_legal(&rook_move),
        "Rook move {:?} should be legal",
        rook_move
      );
    }
  }

  #[test]
  fn test_rook_diagonal_illegal() {
    let board = board_from_fen("8/8/8/8/3R4/8/8/8 w - - 0 1");
    let diagonal_move = simple_move(D4, E5);
    assert!(!board.is_move_legal(&diagonal_move));
  }

  #[test]
  fn test_rook_blocked_path() {
    let board = board_from_fen("8/8/8/8/2PR4/8/8/8 w - - 0 1");
    let blocked_rook = simple_move(D4, A4);
    assert!(!board.is_move_legal(&blocked_rook)); // Path blocked by pawn on C4
  }

  // Queen move tests
  #[test]
  fn test_queen_combined_moves() {
    let board = board_from_fen("k7/8/8/8/3Q4/8/8/7K w - - 0 1");
    // Test both rook-like and bishop-like moves
    let queen_moves = [
      simple_move(D4, D1),
      simple_move(D4, A4), // Rook-like
      simple_move(D4, A1),
      simple_move(D4, G7), // Bishop-like
    ];
    for &queen_move in &queen_moves {
      assert!(
        board.is_move_legal(&queen_move),
        "Queen move {:?} should be legal",
        queen_move
      );
    }
  }

  #[test]
  fn test_queen_knight_move_illegal() {
    let board = board_from_fen("8/8/8/8/3Q4/8/8/8 w - - 0 1");
    let knight_like = simple_move(D4, F3); // L-shape not allowed for queen
    assert!(!board.is_move_legal(&knight_like));
  }

  // King move tests
  #[test]
  fn test_king_adjacent_moves() {
    let board = board_from_fen("8/8/8/8/3K4/8/8/8 w - - 0 1");
    let king_moves = [
      simple_move(D4, C3),
      simple_move(D4, D3),
      simple_move(D4, E3),
      simple_move(D4, C4),
      simple_move(D4, E4),
      simple_move(D4, C5),
      simple_move(D4, D5),
      simple_move(D4, E5),
    ];
    for &king_move in &king_moves {
      assert!(board.is_move_legal(&king_move));
    }
  }

  #[test]
  fn test_king_long_move_illegal() {
    let board = board_from_fen("8/8/8/8/3K4/8/8/8 w - - 0 1");
    let long_move = simple_move(D4, D6); // Two squares
    assert!(!board.is_move_legal(&long_move));
  }

  #[test]
  fn test_king_move_into_check() {
    let board = board_from_fen("8/8/8/8/3K4/8/8/3r4 w - - 0 1");
    let into_check = simple_move(D4, D3); // Moving into attack by rook
    assert!(!board.is_move_legal(&into_check));
  }

  // Castling tests
  #[test]
  fn test_kingside_castling_legal() {
    let board = board_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
    let kingside_castle = castling_move(E1, G1);
    assert!(board.is_move_legal(&kingside_castle));
  }

  #[test]
  fn test_queenside_castling_legal() {
    let board = board_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
    let queenside_castle = castling_move(E1, C1);
    assert!(board.is_move_legal(&queenside_castle));
  }

  #[test]
  fn test_castling_king_in_check() {
    let board = board_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R2rK2R w KQkq - 0 1");
    let castle_in_check = castling_move(E1, G1); // King in check from rook on D1
    assert!(!board.is_move_legal(&castle_in_check));
  }

  #[test]
  fn test_castling_path_blocked() {
    let board = board_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R2QK2R w KQkq - 0 1");
    let castle_blocked = castling_move(E1, C1); // Queen blocks path
    assert!(!board.is_move_legal(&castle_blocked));
  }

  #[test]
  fn test_castling_through_check() {
    let board = board_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K1rR w KQkq - 0 1");
    let castle_through_check = castling_move(E1, G1); // King would pass through F1 under attack
    assert!(!board.is_move_legal(&castle_through_check));
  }

  #[test]
  fn test_castling_no_rights() {
    let board = board_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w - - 0 1");
    let castle_no_rights = castling_move(E1, G1);
    assert!(!board.is_move_legal(&castle_no_rights));
  }

  // Check escape tests
  #[test]
  fn test_must_escape_check() {
    let board = board_from_fen("8/8/8/8/8/8/4r3/4K3 w - - 0 1");
    let non_escape = simple_move(E1, D1); // Doesn't escape check from rook on E2
    assert!(board.is_move_legal(&non_escape));
  }

  #[test]
  fn test_block_check() {
    // King in check from rook - no legal moves available
    let board = board_from_fen("7k/8/8/8/8/8/8/4K2r w - - 0 1");
    // No legal moves - king in check from rook, can't block or move
    assert!(!board.is_move_legal(&simple_move(E1, D1)));
    assert!(!board.is_move_legal(&simple_move(E1, F1))); // Still in check
    assert!(board.is_move_legal(&simple_move(E1, E2))); // Not in check
  }

  #[test]
  fn test_capture_checking_piece() {
    let board = board_from_fen("k7/8/8/8/8/8/4Q3/4K2r w - - 0 1");
    let capture_checker = capture_move(E2, H2); // Queen captures checking rook
    assert!(!board.is_move_legal(&capture_checker));
  }

  // Pinned piece tests
  #[test]
  fn test_pinned_piece_cannot_move() {
    let board = board_from_fen("8/8/8/8/8/8/4B3/4K2r w - - 0 1");
    let pinned_move = simple_move(E2, D3); // Bishop pinned by rook, can't move away
    assert!(!board.is_move_legal(&pinned_move));
  }

  #[test]
  fn test_pinned_piece_can_move_along_pin() {
    let board = board_from_fen("8/8/8/8/8/8/4B3/4K2r w - - 0 1");
    let along_pin = simple_move(E2, F1); // Bishop can move along pin line
    assert!(board.is_move_legal(&along_pin));
  }

  #[test]
  fn test_pinned_piece_can_capture_attacker() {
    let board = board_from_fen("8/8/8/8/4B3/8/8/4K2r w - - 0 1");
    let capture_attacker = capture_move(E4, H1); // Bishop captures pinning rook
    assert!(board.is_move_legal(&capture_attacker));
  }

  // Special game states
  #[test]
  fn test_initial_position_legal_moves() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // Test some standard opening moves
    assert!(board.is_move_legal(&simple_move(E2, E4)));
    assert!(board.is_move_legal(&simple_move(D2, D4)));
    assert!(board.is_move_legal(&simple_move(G1, F3)));
    assert!(board.is_move_legal(&simple_move(B1, C3)));
  }

  #[test]
  fn test_endgame_position() {
    let board = board_from_fen("8/8/8/8/8/8/6k1/6K1 b - - 0 1");
    assert!(board.is_move_legal(&simple_move(G2, F3)));
    assert!(board.is_move_legal(&simple_move(G2, H3)));
    assert!(!board.is_move_legal(&simple_move(G2, H1))); // Kings can't be adjacent
  }

  // Edge cases
  #[test]
  fn test_null_move_illegal() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let null_move = PieceMove::NULL;
    assert!(!board.is_move_legal(&null_move));
  }

  #[test]
  #[should_panic]
  fn test_same_square_move_illegal() {
    let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let same_square = simple_move(E2, E2);
    assert!(!board.is_move_legal(&same_square));
  }

  // Complex scenarios
  #[test]
  fn test_discovered_check_legal() {
    let board = board_from_fen("k7/8/8/8/3B4/8/8/1K1r4 w - - 0 1");
    // Bishop moves, discovering check from rook - this should be legal for white
    let discovered_check = simple_move(D4, E5);
    assert!(!board.is_move_legal(&discovered_check));
  }

  #[test]
  fn test_discovered_check_self_illegal() {
    let board = board_from_fen("K7/8/8/8/3b4/8/8/1k1R4 b - - 0 1");
    // Bishop moves would discover check on own king - illegal
    let self_discovered = simple_move(D4, E5);
    assert!(!board.is_move_legal(&self_discovered));
  }

  #[test]
  fn test_promotion_required() {
    let board = board_from_fen("8/4P3/8/8/8/8/8/8 w - - 0 1");
    // Moving pawn to 8th rank without promotion should be illegal
    let no_promotion = simple_move(E7, E8);
    assert!(!board.is_move_legal(&no_promotion));
  }

  #[test]
  fn test_en_passant_removes_correct_pawn() {
    let mut board = board_from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1");
    board.en_passant = PieceMove::new(D5, D6, false, None); // Set proper en passant target

    // Before en passant - there should be a black pawn on d5
    assert_eq!(board.get_piece(D5), Some(PieceType::Pawn));
    assert!(!board.colour.get_bit(D5)); // Black pawn

    let en_passant = en_passant_move(E5, D6);
    assert!(board.is_move_legal(&en_passant));
  }
}
