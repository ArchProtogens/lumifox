#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::gamedata::GameData;
use lumifox_chess::movegen::generate_moves;

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
