use lumifox_chess::model::gamedata::GameData;
use lumifox_chess_proc::{bitboard, fen, san, sq};

fn main() {
  println!("Testing declarative chess macros!");

  // Test square macro
  const E4: u8 = sq!("e4");
  const A1: u8 = sq!("a1");
  const H8: u8 = sq!("h8");
  println!("Squares: e4={}, a1={}, h8={}", E4, A1, H8);

  // Test bitboard macro
  let center_squares = bitboard!("e4", "e5", "d4", "d5");
  let corners = bitboard!("a1", "a8", "h1", "h8");
  println!("Center squares bitboard: {:016x}", center_squares.raw());
  println!("Corner squares bitboard: {:016x}", corners.raw());

  // Test move macro
  let king_pawn = san!("e2e4");
  let promotion = san!("e7e8q");
  let knight_move = san!("g1f3");
  println!("King pawn move: {:?}", king_pawn);
  println!("Promotion move: {:?}", promotion);
  println!("Knight move: {:?}", knight_move);

  // Test FEN macro
  let _start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
  println!("Starting position created successfully");

  println!("All macros working correctly!");
}
