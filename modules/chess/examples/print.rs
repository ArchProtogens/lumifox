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
      (from % 8 + b'a') as char,
      from / 8 + 1,
      (to % 8 + b'a') as char,
      to / 8 + 1
    );
    if let Some(promotion) = piece_move.promotion_type() {
      println!("Promotion to {promotion:?}");
    }
  }
}

fn main() {
  let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

  let mut game = GameData::from_fen(fen).expect("Failed to parse FEN string");

  println!("Initial board state from FEN: {fen}\n");
  game.print_board();

  println!(
    "\nCurrent turn: {}",
    if game.board.playing { "White" } else { "Black" }
  );

  let mut moves = generate_moves(&game.board);

  println!("In total {} can be played with the pawns.\n", moves.1);
  println!("Such moves include:");
  for piece_move in moves.0.iter().take(moves.1.min(5)) {
    print_move(piece_move);
  }
  if moves.1 > 5 {
    println!("... and more.\n");
  }

  println!("Running 5 random moves from the generated list:\n");

  let mut capture = false;

  for _ in 0..10 {
    // If there is a capture, chose the first one
    if let Some(capture_move) = moves.0.iter().find(|m| m.is_capture()) {
      print!("Capture move: ");
      print_move(capture_move);
      capture = true;
      game.board.move_piece(capture_move);
    } else {
      let rnd_id = rand::random::<u32>() as usize % moves.1;
      let random_move = moves
        .0
        .get(rnd_id)
        .expect("Random move index out of bounds");

      let piece_type = game.board.get_piece(random_move.from_square()).unwrap();
      print!("Random move of {piece_type:?}: ");
      print_move(random_move);
      game.board.move_piece(random_move);
    }

    println!("\nBoard state after random move:\n");
    game.print_board();
    println!(
      "\nCurrent turn: {}\n",
      if game.board.playing { "White" } else { "Black" }
    );

    moves = generate_moves(&game.board);
  }

  if capture {
    println!("A capture was made during the random moves.");
  } else {
    println!("No captures were made during the random moves.");
  }
}
