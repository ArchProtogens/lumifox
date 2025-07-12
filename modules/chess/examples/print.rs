use lumifox_chess::{model::gamedata::GameData, movegen::generate_moves};

fn main() {
  let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

  let board = GameData::from_fen(fen).expect("Failed to parse FEN string");

  println!("Initial board state from FEN: {fen}\n");
  board.print_board();

  println!(
    "\nCurrent turn: {}",
    if board.board.playing {
      "White"
    } else {
      "Black"
    }
  );

  let moves = generate_moves(&board.board);

  println!("In total {} can be played with the pawns", moves.1);
}
