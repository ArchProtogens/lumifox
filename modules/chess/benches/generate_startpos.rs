#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::gamedata::GameData;
use lumifox_chess::movegen::generate_moves;

#[bench]
fn bench_generate_startpos(b: &mut Bencher) {
  let gd = GameData::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
  b.iter(|| {
    let (_moves, _count) = generate_moves(&gd.board);
    test::black_box(_count);
  });
}
