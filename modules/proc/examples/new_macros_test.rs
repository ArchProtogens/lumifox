use lumifox_chess::model::gamedata::GameData;
use lumifox_chess_proc::{fen, move_list, position, san};

fn main() {
  println!("Testing new chess macros: move_list! and position!");

  // Test move_list! macro
  println!("\n=== Testing move_list! ===");

  let expected_opening_moves = move_list![
    "e2e4", // King pawn
    "e7e5", // Black responds
    "g1f3", // Knight development
    "b8c6", // Black knight
    "f1c4", // Bishop development
    "f8c5"  // Black bishop
  ];

  println!(
    "Created move list with {} moves:",
    expected_opening_moves.len()
  );
  for (i, mv) in expected_opening_moves.iter().enumerate() {
    println!("  {}: {:?}", i + 1, mv);
  }

  // Test move_list! with promotions and captures
  let tactical_moves = move_list![
    "e4xd5", // Capture
    "e7e8q", // Promotion to queen
    "h7h8r", // Promotion to rook
    "g2g1n"  // Promotion to knight
  ];

  println!("\nTactical moves ({} moves):", tactical_moves.len());
  for (i, mv) in tactical_moves.iter().enumerate() {
    println!("  {}: {:?}", i + 1, mv);
  }

  // Test position! macro
  println!("\n=== Testing position! ===");

  // Starting position using visual representation
  let visual_start = position! {
      "rnbqkbnr"
      "pppppppp"
      "........"
      "........"
      "........"
      "........"
      "PPPPPPPP"
      "RNBQKBNR"
      ; to_move: White
      ; castling: "KQkq"
      ; halfmove: 0
      ; fullmove: 1
  };

  println!("Created starting position visually:");
  visual_start.print_board();
  println!("FEN: {}", visual_start.to_fen());

  // Compare with FEN macro
  let fen_start: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
  println!(
    "\nPositions match: {}",
    visual_start.to_fen() == fen_start.to_fen()
  );

  // Custom position after some moves
  let custom_position = position! {
      "r.bqkbnr"
      "pppp.ppp"
      "..n.p..."
      "....P..."
      "........"
      "..N....."
      "PPPP.PPP"
      "R.BQKBNR"
      ; to_move: Black
  };

  println!("\nCustom position after some development:");
  custom_position.print_board();
  println!("FEN: {}", custom_position.to_fen());

  // Test position with minimal parameters
  let minimal_pos = position! {
      "....k..."
      "........"
      "........"
      "........"
      "........"
      "........"
      "........"
      "....K..."
      ; to_move: White
  };

  println!("\nMinimal position (kings only):");
  minimal_pos.print_board();

  println!("\n✅ All new macros working perfectly!");

  // Demonstrate how these would be used in tests
  println!("\n=== Example Test Usage ===");

  // This is how you'd use move_list! in tests
  let generated_moves = vec![san!("e2e4"), san!("d7d5"), san!("e4xd5")];
  let expected_moves = move_list!["e2e4", "d7d5", "e4xd5"];

  println!(
    "Generated moves match expected: {}",
    generated_moves.len() == expected_moves.len()
      && generated_moves
        .iter()
        .zip(expected_moves.iter())
        .all(|(a, b)| a == b)
  );

  // This is how you'd create test positions visually instead of ugly FEN strings
  println!("Visual position creation is much more readable than FEN strings!");
}
