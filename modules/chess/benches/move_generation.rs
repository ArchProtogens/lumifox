#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::bitboard::BitBoard;
use lumifox_chess::model::gamedata::GameData;
use lumifox_chess::movegen::generate_moves;

/// Bench: generate all pseudo-legal moves from the starting position
#[bench]
fn bench_generate_startpos(b: &mut Bencher) {
  let gd = GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
  b.iter(|| {
    let (_moves, _count) = generate_moves(&gd.board);
    test::black_box(_count);
  });
}

/// Bench: generate moves for a complex midgame position
#[bench]
fn bench_generate_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  b.iter(|| {
    let (_moves, _count) = generate_moves(&gd.board);
    test::black_box(_count);
  });
}

/// Bench: run legal checker over generated moves from the starting position
#[bench]
fn bench_legal_check_startpos(b: &mut Bencher) {
  let gd = GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
  b.iter(|| {
    let (moves, _count) = generate_moves(&gd.board);
    let mut legal_count = 0usize;
    for mv in &moves {
      if gd.board.is_move_legal(mv) {
        legal_count += 1;
      }
    }
    test::black_box(legal_count);
  });
}

/// Bench: run legal checker over generated moves from a complex midgame position
#[bench]
fn bench_legal_check_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  b.iter(|| {
    let (moves, _count) = generate_moves(&gd.board);
    let mut legal_count = 0usize;
    for mv in &moves {
      if gd.board.is_move_legal(mv) {
        legal_count += 1;
      }
    }
    test::black_box(legal_count);
  });
}

/// Bench: FEN parsing performance
#[bench]
fn bench_fen_parse(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  b.iter(|| {
    let gd = GameData::from_fen(fen).unwrap();
    test::black_box(gd);
  });
}

/// Bench: bitboard iteration and simple ops
#[bench]
fn bench_bitboard_iter(b: &mut Bencher) {
  // all squares set
  let bb = BitBoard::ALL_SQUARES;
  b.iter(|| {
    let mut s = 0u64;
    for sq in bb {
      s = s.wrapping_add(sq as u64);
    }
    test::black_box(s);
  });
}
