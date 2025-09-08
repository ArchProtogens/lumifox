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

// Bit 0-5:   From Square (6 bits)
// Bit 6-11:  To Square (6 bits)
// Bit 12-13: Promotion Type (2 bits: 00=Q, 01=R, 10=B, 11=N). Only valid if IsPromotion flag is set.
// Bit 14:    IsPromotion (1 bit)
// Bit 15:    IsCapture (1 bit)

use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] // Added traits for easier use with arrays/debugging
pub struct PieceMove(u16);

impl Debug for PieceMove {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if *self == PieceMove::NULL {
      write!(f, "NULL")
    } else {
      let to_file = (self.to_square() % 8 + b'a') as char; // Get the file (0-7)
      let to_rank = (self.to_square() / 8 + b'1') as char; // Get the rank (0-7)
      let from_file = (self.from_square() % 8 + b'a') as char; // Get the file (0-7)
      let from_rank = (self.from_square() / 8 + b'1') as char; // Get the rank (0-7)
      write!(
        f,
        "PieceMove({}{} -> {}{}{} - {:?})",
        from_file,
        from_rank,
        to_file,
        to_rank,
        if self.is_capture() { " (Capture)" } else { "" },
        self.promotion_type()
      )
    }
  }
}

// Constants for bit masks and shifts
const FROM_SQUARE_MASK: u16 = 0x3F; // 0b0000_0000_0011_1111
const TO_SQUARE_MASK: u16 = 0x3F; // 0b0000_0000_0011_1111
const FROM_SQUARE_SHIFT: u8 = 0;
const TO_SQUARE_SHIFT: u8 = 6;

const PROMOTION_TYPE_MASK: u16 = 0x3; // 0b11
const PROMOTION_TYPE_SHIFT: u8 = 12;

const IS_PROMOTION_FLAG: u16 = 1 << 14; // 0b0100_0000_0000_0000
const IS_CAPTURE_FLAG: u16 = 1 << 15; // 0b1000_0000_0000_0000

// Enum for the 2-bit promotion type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionType {
  Queen = 0,
  Rook = 1,
  Bishop = 2,
  Knight = 3,
}

impl PieceMove {
  pub const NULL: PieceMove = PieceMove(0); // Represents an invalid or null move

  /// Creates a new PieceMove.
  ///
  /// # Arguments
  /// * `from` - The starting square (0-63).
  /// * `to` - The destination square (0-63).
  /// * `is_capture` - True if the move is a capture.
  /// * `promotion_type` - Optional promotion type. If Some, `is_promotion` flag will be set.
  pub fn new(from: u8, to: u8, is_capture: bool, promotion_type: Option<PromotionType>) -> Self {
    debug_assert!(from < 64, "From square must be between 0 and 63");
    debug_assert!(to < 64, "To square must be between 0 and 63");
    debug_assert!(
      from != to,
      "From and to squares must be different for a valid move"
    );

    let mut move_value: u16 = 0;

    // Pack from and to squares
    move_value |= (from as u16) << FROM_SQUARE_SHIFT; // Bits 0-5
    move_value |= (to as u16) << TO_SQUARE_SHIFT; // Bits 6-11

    // Pack flags
    if is_capture {
      move_value |= IS_CAPTURE_FLAG; // Bit 15
    }

    if let Some(promo_type) = promotion_type {
      move_value |= IS_PROMOTION_FLAG; // Bit 14
      move_value |= ((promo_type as u16) & PROMOTION_TYPE_MASK) << PROMOTION_TYPE_SHIFT; // Bits 12-13
    }

    PieceMove(move_value)
  }

  /// Creates a new Castling move.
  /// Castling moves are special and do not fit the general capture/promotion scheme.
  /// You might need specific flags for these if they are represented in PieceMove.
  /// For this example, let's assume castling has its own fixed pattern or specific flag.
  /// A common way is to make the `from` and `to` squares encode castling directly,
  /// e.g., King e1 to g1 for Kingside, e1 to c1 for Queenside.
  ///
  /// For simplicity, given the current bit allocation, castling might be distinguished
  /// by specific (from, to) pairs and then checked in `is_castling()`.
  pub fn new_castling(king_from: u8, king_to: u8) -> Self {
    // You might define special flags or combinations of (from, to) for castling.
    // For example, an empty promotion_type and capture flag might signify a "special" move
    // that's then identified as castling based on (king_from, king_to)
    // Or, if you need a specific flag, you'd need more bits in your u16 or a different packing.
    // For now, let's assume a "plain" move for the packed bits, and
    // you'd check `is_kingside_castling()` or `is_queenside_castling()` by looking at from/to.
    PieceMove::new(king_from, king_to, false, None) // Castling is not a capture or promotion
  }

  /// Creates a new Two-Square Pawn Advance move.
  /// Similar to castling, this could be a specific flag if you have more bits,
  /// or identified by (from, to) logic in is_two_square_advance().
  pub fn new_two_square_advance(from: u8, to: u8) -> Self {
    // This is a plain move, not a capture or promotion.
    // The fact it's a two-square advance is determined by the piece type (pawn)
    // and the (from, to) square difference (e.g., from rank 2 to rank 4 for white).
    // This flag is crucial for en passant tracking.
    PieceMove::new(from, to, false, None)
  }

  /// Creates a new En Passant capture move.
  /// En passant is a capture, but special.
  /// It's common to have a dedicated flag for it if bits allow, or combine.
  /// Here, it's a capture, but the `is_en_passant` status would be determined
  /// by the move generator and verified during move execution.
  /// If you want a dedicated flag here, you'd need another bit in your u16.
  pub fn new_en_passant(from: u8, to: u8) -> Self {
    // En passant is a capture, so set the capture flag.
    // If you need a distinct 'is_en_passant' flag for move representation,
    // you'd need to expand your u16 flags. For now, it's a 'capture'.
    PieceMove::new(from, to, true, None) // It is a capture
  }

  /// Creates a simple PieceMove from from and to squares, assuming no capture and no promotion.
  /// Use this for basic moves where flags need to be set later or are not applicable.
  pub fn simple(from: u8, to: u8) -> Self {
    Self::new(from, to, false, None)
  }

  #[inline] // Hint to the compiler to inline this function for performance
  pub fn from_square(&self) -> u8 {
    ((self.0 >> FROM_SQUARE_SHIFT) & FROM_SQUARE_MASK) as u8
  }

  #[inline]
  pub fn to_square(&self) -> u8 {
    ((self.0 >> TO_SQUARE_SHIFT) & TO_SQUARE_MASK) as u8
  }

  #[inline]
  pub fn is_capture(&self) -> bool {
    (self.0 & IS_CAPTURE_FLAG) != 0
  }

  #[inline]
  pub fn is_promotion(&self) -> bool {
    (self.0 & IS_PROMOTION_FLAG) != 0
  }

  /// Returns the promotion type if the move is a promotion, otherwise None.
  #[inline]
  pub fn promotion_type(&self) -> Option<PromotionType> {
    if self.is_promotion() {
      match ((self.0 >> PROMOTION_TYPE_SHIFT) & PROMOTION_TYPE_MASK) as u8 {
        0 => Some(PromotionType::Queen),
        1 => Some(PromotionType::Rook),
        2 => Some(PromotionType::Bishop),
        3 => Some(PromotionType::Knight),
        _ => unreachable!("Invalid promotion type bits"), // Should not happen with 2-bit mask
      }
    } else {
      None
    }
  }

  // --- Helper methods to identify specific move types based on (from, to) ---
  // These are often needed because not all flags can fit into the move representation.
  // They usually require looking at the PieceType of the piece making the move.

  /// Checks if the move is a pawn's two-square advance (from rank 2 to 4 for White, 7 to 5 for Black).
  /// Requires knowledge of the piece making the move (pawn) and its color.
  pub fn is_two_square_advance(from: u8, to: u8, is_white_pawn_move: bool) -> bool {
    if is_white_pawn_move {
      // White pawn moves from rank 2 to rank 4
      (from / 8 == 1) && (to / 8 == 3) && (from % 8 == to % 8)
    } else {
      // Black pawn moves from rank 7 to rank 5
      (from / 8 == 6) && (to / 8 == 4) && (from % 8 == to % 8)
    }
  }

  /// Checks if the move is kingside castling.
  /// Requires knowledge of the piece making the move (King) and its color.
  pub fn is_kingside_castling(from: u8, to: u8, is_white_king_move: bool) -> bool {
    if is_white_king_move {
      from == 4 && to == 6 // e1 to g1
    } else {
      from == 60 && to == 62 // e8 to g8
    }
  }

  /// Checks if the move is queenside castling.
  /// Requires knowledge of the piece making the move (King) and its color.
  pub fn is_queenside_castling(from: u8, to: u8, is_white_king_move: bool) -> bool {
    if is_white_king_move {
      from == 4 && to == 2 // e1 to c1
    } else {
      from == 60 && to == 58 // e8 to c8
    }
  }

  /// Checks if the move is an en passant capture, determined by a capture flag
  /// and a one-square diagonal move (difference of 7 or 9 in the packed squares).
  #[inline]
  pub fn is_en_passant(&self) -> bool {
    if !self.is_capture() {
      return false;
    }
    let from = self.from_square() as i8;
    let to = self.to_square() as i8;
    let diff = (from - to).abs();
    diff == 7 || diff == 9
  }
}

// Add Default trait for PieceMove for array initialization
impl Default for PieceMove {
  fn default() -> Self {
    PieceMove::NULL
  }
}
