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

#[cfg(test)]
mod tests {
  use lumifox_chess::model::gamedata::GameData;
  use lumifox_chess_proc::fen;

  #[test]
  fn test_fen_macro_starting_position() {
    let start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    assert!(start_pos.board.playing); // White to move
    assert_eq!(start_pos.plies, 0);
    assert_eq!(start_pos.halfmove_clock, 0);
    assert_eq!(start_pos.board.castling, 0b1111); // All castling rights
  }

  #[test]
  fn test_fen_macro_black_to_move() {
    let black_move: GameData =
      fen!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");

    assert!(!black_move.board.playing); // Black to move
    assert_eq!(black_move.plies, 3);
    assert_eq!(black_move.halfmove_clock, 1);
  }

  #[test]
  fn test_fen_macro_en_passant() {
    let en_passant: GameData = fen!("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");

    assert!(en_passant.board.playing); // White to move
    assert_ne!(
      en_passant.board.en_passant,
      lumifox_chess::model::piecemove::PieceMove::NULL
    );
    assert_eq!(en_passant.board.en_passant.to_square(), 43); // d6 = 43
  }

  #[test]
  fn test_fen_macro_no_castling() {
    let no_castling: GameData =
      fen!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b - - 1 1");

    assert!(!no_castling.board.playing); // Black to move
    assert_eq!(no_castling.board.castling, 0); // No castling rights
    assert_eq!(no_castling.halfmove_clock, 1);
  }

  #[test]
  fn test_fen_macro_endgame() {
    let endgame: GameData = fen!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 55");

    assert!(endgame.board.playing); // White to move
    assert_eq!(endgame.board.castling, 0); // No castling rights
    assert_eq!(endgame.halfmove_clock, 0);
    assert_eq!(endgame.plies, 108);
  }
}
