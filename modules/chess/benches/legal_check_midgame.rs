#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::gamedata::GameData;
use lumifox_chess::movegen::generate_moves;

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
