#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::gamedata::GameData;
use lumifox_chess::movegen::generate_moves;

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
