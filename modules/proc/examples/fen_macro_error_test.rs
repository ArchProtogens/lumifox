//use lumifox_chess::model::gamedata::GameData;
//use lumifox_chess_proc::fen;

fn main() {
  // This example demonstrates the kind of compile-time error produced
  // when an invalid FEN string is used with the `fen!()` macro.
  //
  // The failing line is intentionally commented out so that the example
  // can be built or run without causing a compile-time failure.

  // Uncommenting the following line should produce a compile error:
  // let invalid: GameData = fen!("invalid_fen_string");

  println!("To see the compile-time error, uncomment the invalid fen!() line in this file.");
}
