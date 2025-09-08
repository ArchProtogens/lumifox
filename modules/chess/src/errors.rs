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

pub enum InvalidMove {
  OutOfBounds,
  InvalidPiece,
  InvalidDestination,
  InvalidAction,
  InvalidPromotion,
  InvalidEnPassant,
  InvalidCastling,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FenParseError {
  /// The FEN string has an incorrect number of fields (expected 6).
  MalformedFen,
  /// Error parsing the piece placement section.
  InvalidPiecePlacement,
  /// An invalid character was found in the piece placement section.
  InvalidPieceChar,
  /// A rank in the piece placement section has an incorrect number of squares.
  InvalidRankLength,
  /// The piece placement section has an incorrect number of ranks.
  InvalidRankCount,
  /// Error parsing the active color field (not 'w' or 'b').
  InvalidActiveColor,
  /// Error parsing the castling availability field.
  InvalidCastling,
  /// An invalid character was found in the castling availability field.
  InvalidCastlingChar,
  /// Error parsing the en passant target square field.
  InvalidEnPassant,
  /// The en passant square is not a valid algebraic notation.
  InvalidEnPassantSquare,
  /// The en passant square doesn't match the board context (no pawn to capture, wrong side to move, etc.).
  InvalidEnPassantContext,
  /// Error parsing the half-move clock (not a valid number).
  InvalidHalfmoveClock,
  /// Error parsing the full-move number (not a valid number).
  InvalidFullmoveNumber,
  /// A numeric value was expected but not found or was unparseable.
  ExpectedNumber,
  /// An unexpected character was encountered during parsing.
  UnexpectedCharacter,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveParseError {
  /// The move string is too short (less than 4 characters).
  TooShort,
  /// Invalid character for the from file.
  InvalidFromFile,
  /// Invalid character for the from rank.
  InvalidFromRank,
  /// Invalid character for the to file.
  InvalidToFile,
  /// Invalid character for the to rank.
  InvalidToRank,
  /// File or rank index is out of bounds (not 0-7).
  OutOfBounds,
  /// Invalid character for the promotion piece.
  InvalidPromotionPiece,
}
