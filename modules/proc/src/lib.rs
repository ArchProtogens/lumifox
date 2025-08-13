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

use proc_macro::TokenStream;
use quote::quote;
use syn::{
  LitStr, Token,
  parse::{Parse, ParseStream},
  parse_macro_input,
  punctuated::Punctuated,
};

/// A procedural macro that parses a FEN string at compile time and generates
/// a `GameData` instance.
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::fen;
/// use lumifox_chess::model::gamedata::GameData;
///
/// // Parse the starting position
/// let start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
///
/// // Parse a more complex position
/// let kiwipete: GameData = fen!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
/// ```
///
/// # Panics
///
/// This macro will cause a compile-time error if the FEN string is invalid.
#[proc_macro]
pub fn fen(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as LitStr);
  let fen_string = input.value();

  // Parse the FEN at compile time to validate it
  let game_data = match lumifox_chess::model::gamedata::GameData::from_fen(&fen_string) {
    Ok(data) => data,
    Err(e) => {
      return syn::Error::new(input.span(), format!("Invalid FEN string: {e:?}"))
        .to_compile_error()
        .into();
    }
  };

  // Extract the components we need to reconstruct the GameData at runtime
  let board = &game_data.board;
  let plies = game_data.plies;
  let halfmove_clock = game_data.halfmove_clock;

  // Extract bitboard values
  let pawns_bits = board.pawns.raw();
  let knights_bits = board.knights.raw();
  let bishops_bits = board.bishops.raw();
  let rooks_bits = board.rooks.raw();
  let queens_bits = board.queens.raw();
  let kings_bits = board.kings.raw();
  let colour_bits = board.colour.raw();
  let playing = board.playing;
  let castling = board.castling;

  // Extract en passant information
  let en_passant_from = board.en_passant.from_square();
  let en_passant_to = board.en_passant.to_square();
  let en_passant_is_capture = board.en_passant.is_capture();
  // Generate the en passant move - use NULL if it's the default NULL move
  let en_passant_move = if board.en_passant == lumifox_chess::model::piecemove::PieceMove::NULL {
    quote! { PieceMove::NULL }
  } else {
    let en_passant_promotion = match board.en_passant.promotion_type() {
      Some(lumifox_chess::model::piecemove::PromotionType::Queen) => {
        quote! { Some(lumifox_chess::model::piecemove::PromotionType::Queen) }
      }
      Some(lumifox_chess::model::piecemove::PromotionType::Rook) => {
        quote! { Some(lumifox_chess::model::piecemove::PromotionType::Rook) }
      }
      Some(lumifox_chess::model::piecemove::PromotionType::Bishop) => {
        quote! { Some(lumifox_chess::model::piecemove::PromotionType::Bishop) }
      }
      Some(lumifox_chess::model::piecemove::PromotionType::Knight) => {
        quote! { Some(lumifox_chess::model::piecemove::PromotionType::Knight) }
      }
      None => quote! { None },
    };
    quote! { PieceMove::new(#en_passant_from, #en_passant_to, #en_passant_is_capture, #en_passant_promotion) }
  };

  // Generate the code to create the GameData at runtime
  let expanded = quote! {
      {
          use lumifox_chess::model::{
              gamedata::{GameData, MAX_GAME_MOVES},
              gameboard::GameBoard,
              bitboard::BitBoard,
              piecemove::PieceMove,
          };

          GameData {
              board: GameBoard {
                  pawns: BitBoard::new(#pawns_bits),
                  knights: BitBoard::new(#knights_bits),
                  bishops: BitBoard::new(#bishops_bits),
                  rooks: BitBoard::new(#rooks_bits),
                  queens: BitBoard::new(#queens_bits),
                  kings: BitBoard::new(#kings_bits),
                  colour: BitBoard::new(#colour_bits),
                  playing: #playing,
                  castling: #castling,
                  en_passant: #en_passant_move,
              },
              moves: [PieceMove::NULL; MAX_GAME_MOVES],
              plies: #plies,
              halfmove_clock: #halfmove_clock,
          }
      }
  };

  TokenStream::from(expanded)
}

// Helper to parse comma-separated square literals
struct SquareList(Punctuated<LitStr, Token![,]>);
impl Parse for SquareList {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    // parse comma-separated string literals
    let list = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
    Ok(SquareList(list))
  }
}

/// Compile-time square literal: e.g. sq!("e4") -> u8 index  
#[proc_macro]
pub fn sq(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as LitStr);
  let s = input.value();
  if s.len() != 2 {
    return syn::Error::new(input.span(), "Invalid square literal")
      .to_compile_error()
      .into();
  }
  let bytes = s.as_bytes();
  let file = bytes[0];
  let rank = bytes[1];
  if file < b'a' || file > b'h' || rank < b'1' || rank > b'8' {
    return syn::Error::new(input.span(), "Square out of range a1..h8")
      .to_compile_error()
      .into();
  }
  let file_idx = file - b'a';
  let rank_idx = rank - b'1';
  let idx = rank_idx as u8 * 8 + file_idx as u8;
  TokenStream::from(quote! { #idx })
}
/// Compile-time bitboard from list of squares: e.g. bitboard!("a1","h8")
#[proc_macro]
pub fn bitboard(input: TokenStream) -> TokenStream {
  let SquareList(sqs) = parse_macro_input!(input as SquareList);
  let mut bb: u64 = 0;
  for lit in sqs.iter() {
    let s = lit.value();
    if s.len() != 2 {
      return syn::Error::new(lit.span(), "Invalid square literal")
        .to_compile_error()
        .into();
    }
    let b = s.as_bytes();
    let file = b[0];
    let rank = b[1];
    if file < b'a' || file > b'h' || rank < b'1' || rank > b'8' {
      return syn::Error::new(lit.span(), "Square out of range a1..h8")
        .to_compile_error()
        .into();
    }
    let file_idx = file - b'a';
    let rank_idx = rank - b'1';
    let idx = rank_idx as u8 * 8 + file_idx as u8;
    bb |= 1u64 << idx;
  }
  TokenStream::from(quote! { lumifox_chess::model::bitboard::BitBoard::new(#bb) })
}

/// Compile-time UCI-style move literal: e.g. san!("e2e4"), optional promotion like "e7e8q"
#[proc_macro]
pub fn san(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as LitStr);
  let s = input.value();
  let b = s.as_bytes();
  if b.len() < 4 {
    return syn::Error::new(input.span(), "Invalid move literal")
      .to_compile_error()
      .into();
  }
  // parse from and to
  let parse_sq = |off: usize, span| {
    let file = b[off];
    let rank = b[off + 1];
    if file < b'a' || file > b'h' || rank < b'1' || rank > b'8' {
      return Err(syn::Error::new(span, "Square out of range").to_compile_error());
    }
    let idx = (rank - b'1') as u8 * 8 + (file - b'a') as u8;
    Ok(idx)
  };
  let from = match parse_sq(0, input.span()) {
    Ok(v) => v,
    Err(err) => return err.into(),
  };
  let to = match parse_sq(2, input.span()) {
    Ok(v) => v,
    Err(err) => return err.into(),
  };
  // promotion
  let promo = if b.len() == 5 {
    match b[4] as char {
      'q' | 'Q' => quote! { Some(lumifox_chess::model::piecemove::PromotionType::Queen) },
      'r' | 'R' => quote! { Some(lumifox_chess::model::piecemove::PromotionType::Rook) },
      'b' | 'B' => quote! { Some(lumifox_chess::model::piecemove::PromotionType::Bishop) },
      'n' | 'N' => quote! { Some(lumifox_chess::model::piecemove::PromotionType::Knight) },
      _ => {
        return syn::Error::new(input.span(), "Invalid promotion")
          .to_compile_error()
          .into();
      }
    }
  } else {
    quote! { None }
  };
  TokenStream::from(quote! {
    lumifox_chess::model::piecemove::PieceMove::new(
      #from,
      #to,
      false,
      #promo,
    )
  })
}
