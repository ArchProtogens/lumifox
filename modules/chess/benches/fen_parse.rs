#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::gamedata::GameData;

#[bench]
fn bench_fen_parse(b: &mut Bencher) {
  let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
  b.iter(|| {
    let gd = GameData::from_fen(fen).unwrap();
    test::black_box(gd);
  });
}
