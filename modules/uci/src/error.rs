/*
 * This file is dual-licensed under the terms of the GNU Lesser General Public License,
 * Version 3 or later, and the BSD 3-Clause License.
 *
 * You are free to use this software under the terms of either licence.
 * See the `LICENCE-LGPL-3.0-or-later.md` and `LICENCE-BSD-3-Clause.md`
 * files in this repository for the full text of each licence.
 *
 * If the files have not been provided, you can find the full text of the licences at:
 * LGPL-3.0-or-later: https://opensource.org/license/lgpl-3-0
 * BSD-3-Clause: https://opensource.org/license/bsd-3-clause
 *
 * Copyright (C) 2025 Clifton Toaster Reid
 */

use lumifox_chess::errors::MoveParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UciError {
  #[error("IO error: {0}")]
  IO(std::io::Error),

  #[error("Parser error: {0}")]
  Parser(String),

  // Use Debug formatting since MoveParseError does not implement Display.
  #[error("Invalid piece move: {0:?}")]
  InvalidPieceMove(MoveParseError),
}

// Convenience conversion so `?` works with functions that return UciError.
impl From<MoveParseError> for UciError {
  fn from(e: MoveParseError) -> Self {
    UciError::InvalidPieceMove(e)
  }
}
