#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::legal::attack::is_square_attacked;
use lumifox_chess::model::gamedata::GameData;

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
