#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::bitboard::BitBoard;

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
