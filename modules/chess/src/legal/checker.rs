/*
 * Legal move checker moved into its own module.
 * This is an unoptimised implementation copied from GameBoard logic.
 */

use crate::legal::attack::is_square_attacked;
use crate::model::gameboard::GameBoard;
use crate::model::gameboard::PieceType;
use crate::model::piecemove::PieceMove;
#[cfg(feature = "precomputed_rays")]
use crate::model::rays::{
  KING_MOVES, KNIGHT_MOVES, PAWN_ATTACK_BLACK, PAWN_ATTACK_WHITE, PAWN_PUSH_BLACK, PAWN_PUSH_WHITE,
};

pub struct LegalChecker<'a> {
  pub board: &'a GameBoard,
}

impl<'a> LegalChecker<'a> {
  pub fn new(board: &'a GameBoard) -> Self {
    Self { board }
  }

  pub fn is_move_legal(&self, piece_move: &PieceMove) -> bool {
    // replicate the original checks from GameBoard::is_move_legal
    if !self.is_correct_turn_piece(piece_move) {
      return false;
    }
    if !self.is_piece_move_valid(piece_move) {
      return false;
    }
    if !self.is_destination_valid(piece_move) {
      return false;
    }
    if !self.are_special_moves_valid(piece_move) {
      return false;
    }
    if !self.does_not_leave_king_in_check(piece_move) {
      return false;
    }
    true
  }

  fn is_correct_turn_piece(&self, piece_move: &PieceMove) -> bool {
    self
      .board
      .colour
      .get_bit(piece_move.from_square())
      .is_some_and(|f| f == self.board.playing)
  }

  fn is_piece_move_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let piece_type = match self.board.get_piece(from) {
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

  fn is_destination_valid(&self, piece_move: &PieceMove) -> bool {
    let to = piece_move.to_square();

    if let Some(_) = self.board.get_piece(to)
      && self
        .board
        .colour
        .get_bit(to)
        .is_some_and(|f| f == self.board.playing)
    {
      return false;
    }

    if let Some(PieceType::King) = self.board.get_piece(to) {
      return false;
    }

    true
  }

  fn is_pawn_move_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let from_rank = from / 8;
    let to_rank = to / 8;
    let from_file = from % 8;
    let to_file = to % 8;

    let is_forward = (self.board.playing && to > from) || (!self.board.playing && from > to);
    let is_capture = self.board.get_piece(to).is_some()
      && self
        .board
        .colour
        .get_bit(to)
        .is_some_and(|f| f != self.board.playing);
    let is_en_passant = piece_move.is_en_passant();
    let is_promotion = piece_move.is_promotion();

    if !is_forward {
      return false;
    }

    // Use precomputed pawn masks when available to validate simple pawn moves quickly.
    #[cfg(feature = "precomputed_rays")]
    {
      // Forward (single) push
      if from_file == to_file {
        let push_mask = if self.board.playing {
          PAWN_PUSH_WHITE[from as usize]
        } else {
          PAWN_PUSH_BLACK[from as usize]
        };
        return (push_mask & (1u64 << to)) != 0
          && self.is_pawn_forward_move_valid(from, to, from_rank, to_rank, is_promotion);
      }

      // Diagonal capture
      if (from_file as i8 - to_file as i8).abs() == 1 {
        let attack_mask = if self.board.playing {
          PAWN_ATTACK_WHITE[from as usize]
        } else {
          PAWN_ATTACK_BLACK[from as usize]
        };
        if (attack_mask & (1u64 << to)) == 0 {
          return false;
        }
        return self.is_pawn_diagonal_move_valid(piece_move, is_capture, is_en_passant, to_rank);
      }
    }

    #[cfg(not(feature = "precomputed_rays"))]
    {
      if from_file == to_file {
        return self.is_pawn_forward_move_valid(from, to, from_rank, to_rank, is_promotion);
      } else if (from_file as i8 - to_file as i8).abs() == 1 {
        return self.is_pawn_diagonal_move_valid(piece_move, is_capture, is_en_passant, to_rank);
      }
    }

    false
  }

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
      if self.board.get_piece(to).is_some() {
        return false;
      }
    } else if diff == 16
      && ((from_rank == 1 && self.board.playing) || (from_rank == 6 && !self.board.playing))
    {
      let mid = if self.board.playing {
        from + 8
      } else {
        from - 8
      };
      if self.board.get_piece(to).is_some() || self.board.get_piece(mid).is_some() {
        return false;
      }
    } else {
      return false;
    }

    self.is_pawn_promotion_valid(to_rank, is_promotion)
  }

  fn is_pawn_diagonal_move_valid(
    &self,
    piece_move: &PieceMove,
    is_capture: bool,
    is_en_passant: bool,
    to_rank: u8,
  ) -> bool {
    if !(is_capture || is_en_passant) {
      return false;
    }
    self.is_pawn_promotion_valid(to_rank, piece_move.is_promotion())
  }

  fn is_pawn_promotion_valid(&self, to_rank: u8, is_promotion: bool) -> bool {
    let should_promote =
      (to_rank == 7 && self.board.playing) || (to_rank == 0 && !self.board.playing);

    if should_promote && !is_promotion {
      return false;
    }
    if !should_promote && is_promotion {
      return false;
    }
    true
  }

  fn is_knight_move_valid(&self, from: u8, to: u8) -> bool {
    #[cfg(feature = "precomputed_rays")]
    {
      // Use precomputed knight move mask
      (KNIGHT_MOVES[from as usize] & (1u64 << to)) != 0
    }
    #[cfg(not(feature = "precomputed_rays"))]
    {
      let dr = (from / 8) as i8 - (to / 8) as i8;
      let df = (from % 8) as i8 - (to % 8) as i8;
      (dr.abs() == 2 && df.abs() == 1) || (dr.abs() == 1 && df.abs() == 2)
    }
  }

  fn is_bishop_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;
    if dr.abs() != df.abs() {
      return false;
    }
    self.board.is_path_clear(from, to)
  }

  fn is_rook_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;
    if dr != 0 && df != 0 {
      return false;
    }
    self.board.is_path_clear(from, to)
  }

  fn is_queen_move_valid(&self, from: u8, to: u8) -> bool {
    let dr = (from / 8) as i8 - (to / 8) as i8;
    let df = (from % 8) as i8 - (to % 8) as i8;
    let is_diagonal = dr.abs() == df.abs();
    let is_straight = dr == 0 || df == 0;
    if !(is_diagonal || is_straight) {
      return false;
    }
    self.board.is_path_clear(from, to)
  }

  fn is_king_move_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    #[cfg(feature = "precomputed_rays")]
    {
      // Quick adjacency test with precomputed king moves
      if (KING_MOVES[from as usize] & (1u64 << to)) != 0 {
        return true;
      }
      // Castling remains a special-case two-square horizontal move
      let dr = (from / 8) as i8 - (to / 8) as i8;
      let df = (from % 8) as i8 - (to % 8) as i8;
      if dr == 0 && df.abs() == 2 {
        return self.is_castling_valid(piece_move);
      }
      false
    }
    #[cfg(not(feature = "precomputed_rays"))]
    {
      let dr = (from / 8) as i8 - (to / 8) as i8;
      let df = (from % 8) as i8 - (to % 8) as i8;
      if dr.abs() <= 1 && df.abs() <= 1 {
        return true;
      }
      if dr == 0 && df.abs() == 2 {
        return self.is_castling_valid(piece_move);
      }
      false
    }
  }

  fn is_castling_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let is_kingside = to == from + 2;
    let (can_k, can_q) = if self.board.playing {
      self.board.casling_right_white()
    } else {
      self.board.casling_right_black()
    };
    if (is_kingside && !can_k) || (!is_kingside && !can_q) {
      return false;
    }
    if !self.are_castling_squares_clear(from, is_kingside) {
      return false;
    }
    self.is_castling_path_safe(from, is_kingside)
  }

  fn are_castling_squares_clear(&self, from: u8, is_kingside: bool) -> bool {
    if is_kingside {
      for sq in [from + 1, from + 2] {
        if self.board.combined().get_bit(sq).unwrap_or(false) {
          return false;
        }
      }
    } else {
      for sq in [from - 1, from - 2, from - 3] {
        if self.board.combined().get_bit(sq).unwrap_or(false) {
          return false;
        }
      }
    }
    true
  }

  fn is_castling_path_safe(&self, from: u8, is_kingside: bool) -> bool {
    let path = if is_kingside {
      [from, from + 1, from + 2]
    } else {
      [from, from - 1, from - 2]
    };
    for &sq in &path {
      if is_square_attacked(self.board, sq) {
        return false;
      }
    }
    true
  }

  fn are_special_moves_valid(&self, piece_move: &PieceMove) -> bool {
    if piece_move.is_en_passant() && self.board.get_piece(piece_move.to_square()).is_none() {
      return self.is_en_passant_valid(piece_move);
    }
    true
  }

  fn is_en_passant_valid(&self, piece_move: &PieceMove) -> bool {
    let from = piece_move.from_square();
    let to = piece_move.to_square();
    let ep_square = self.board.en_passant.to_square();
    if ep_square != to {
      return false;
    }
    let from_file = from % 8;
    let to_file = to % 8;
    let from_rank = from / 8;
    let to_rank = to / 8;
    let correct_forward = if self.board.playing {
      to_rank == from_rank + 1
    } else {
      to_rank + 1 == from_rank
    };
    if (from_file as i8 - to_file as i8).abs() != 1 || !correct_forward {
      return false;
    }
    if self.board.get_piece(to).is_some() {
      return false;
    }
    let captured_pawn_square = if self.board.playing { to - 8 } else { to + 8 };
    if self.board.get_piece(captured_pawn_square) != Some(PieceType::Pawn)
      || self
        .board
        .colour
        .get_bit(captured_pawn_square)
        .is_some_and(|f| f == self.board.playing)
    {
      return false;
    }
    true
  }

  fn does_not_leave_king_in_check(&self, piece_move: &PieceMove) -> bool {
    let mut new_board = *self.board;
    new_board.apply_move_unchecked(piece_move);
    if let Some(king_square) = new_board.find_king(self.board.playing) {
      !is_square_attacked(&new_board, king_square)
    } else {
      false
    }
  }
}
