use lumifox_chess::model::{gamedata::GameData, piecemove::PieceMove};
use lumifox_chess_proc::fen;

fn print_fen(game_data: &GameData) {
  println!("FEN: {}", game_data.to_fen());
  println!("Board: \n");
  game_data.print_board();
  println!();
  println!(
    "Plies: w{} - b{}",
    game_data.white_plies(),
    game_data.black_plies()
  );
  println!("Moves: ");
  // Print moves from the list until you reach the first NULL move
  for mv in game_data.moves {
    if mv == PieceMove::NULL {
      break;
    }
    print!("{:#?} ", mv);
  }
  println!();
}

fn main() {
  // Starting position
  let start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
  print_fen(&start_pos);

  // Kiwipete - a common test position
  let kiwipete: GameData =
    fen!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  print_fen(&kiwipete);

  // Position with en passant target
  let en_passant_pos: GameData =
    fen!("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");
  print_fen(&en_passant_pos);

  // A midgame position (mixed pieces)
  let midgame: GameData = fen!("r4rk1/1pp1qppp/p1n1pn2/3p4/3P4/2N1PN2/PPQ2PPP/2KR3R b - - 2 14");
  print_fen(&midgame);

  println!("All FEN examples parsed successfully.");
}
