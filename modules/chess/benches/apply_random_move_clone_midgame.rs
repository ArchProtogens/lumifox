#![feature(test)]

extern crate test;

use test::Bencher;

use lumifox_chess::model::gamedata::GameData;
use lumifox_chess::movegen::generate_moves;

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
