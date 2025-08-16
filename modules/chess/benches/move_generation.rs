#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::legal::attack::is_square_attacked;
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

/// Bench: pick a pseudo-random generated move and check if it's legal
#[bench]
fn bench_random_move_check_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  let (moves, _count) = generate_moves(&gd.board);
  // Simple LCG-style PRNG state (deterministic)
  let mut seed: u64 = 0xDEADBEEFu64;
  let len = if moves.is_empty() { 1 } else { moves.len() };
  b.iter(|| {
    // advance PRNG
    seed = seed.wrapping_mul(6364136223846793005u64).wrapping_add(1);
    let idx = ((seed >> 32) as usize) % len;
    let mv = &moves[idx % moves.len()];
    let valid = gd.board.is_move_legal(mv);
    test::black_box(valid);
  });
}

/// Bench: clone the board and attempt to apply a pseudo-random move with `move_piece`
#[bench]
fn bench_apply_random_move_clone_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  let (moves, _count) = generate_moves(&gd.board);
  let mut seed: u64 = 0xCAFEBABEu64;
  let len = if moves.is_empty() { 1 } else { moves.len() };
  b.iter(|| {
    seed = seed.wrapping_mul(6364136223846793005u64).wrapping_add(1);
    let idx = ((seed >> 32) as usize) % len;
    let mv = &moves[idx % moves.len()];
    // clone the board for an isolated application
    let mut board = gd.board; // GameBoard is Copy
    let res = board.move_piece(mv);
    test::black_box(res);
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

/// Bench: scan all 64 squares calling `is_square_attacked` for a complex midgame
#[bench]
fn bench_is_square_attacked_scan_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  b.iter(|| {
    let mut attacked_count = 0usize;
    for sq in 0u8..64u8 {
      if is_square_attacked(&gd.board, sq) {
        attacked_count += 1;
      }
    }
    test::black_box(attacked_count);
  });
}

/// Bench: call `get_piece` over all squares to micro-benchmark piece lookup
#[bench]
fn bench_get_piece_scan_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  b.iter(|| {
    let mut found = 0usize;
    for sq in 0u8..64u8 {
      if gd.board.get_piece(sq).is_some() {
        found += 1;
      }
    }
    test::black_box(found);
  });
}

/// Bench: clone the board and try `move_piece` for every generated move from midgame position
#[bench]
fn bench_try_all_generated_moves_apply_midgame(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  let gd = GameData::from_fen(fen).unwrap();
  let (moves, count) = generate_moves(&gd.board);
  b.iter(|| {
    let mut applied = 0usize;
    for mv in moves.iter().take(count) {
      let mut board = gd.board; // copy
      if board.move_piece(mv).is_some() {
        applied += 1;
      }
    }
    test::black_box(applied);
  });
}
