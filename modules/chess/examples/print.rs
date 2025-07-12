use lumifox_chess::{
  model::{gamedata::GameData, piecemove::PieceMove},
  movegen::generate_moves,
};

fn print_move(piece_move: &PieceMove) {
  if *piece_move == PieceMove::NULL {
    println!("NULL move");
  } else {
    let from = piece_move.from_square();
    let to = piece_move.to_square();

    println!(
      "Move from \x1b[96m{}{}\x1b[0m to \x1b[96m{}{}\x1b[0m",
      (from / 8 + 1 + b'a') as char,
      from % 8 + 1,
      (to / 8 + 1 + b'a') as char,
      to % 8 + 1
    );
    if let Some(promotion) = piece_move.promotion_type() {
      println!("Promotion to {:?}", promotion);
    }
  }
}

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

  println!("In total {} can be played with the pawns.\n", moves.1);
  println!("Such moves include:");
  for piece_move in moves.0.iter().take(moves.1.min(5)) {
    print_move(piece_move);
  }
  if moves.1 > 5 {
    println!("... and more moves.");
  }
}
