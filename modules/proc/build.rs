use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Opening {
  eco: String,
  name: String,
  pgn: String,
}

/// Parse PGN string into a vector of moves
fn parse_pgn_moves(pgn: &str) -> Vec<String> {
  // Split by spaces and filter out move numbers, comments, and annotations
  pgn
    .split_whitespace()
    .filter_map(|token| {
      let token = token.trim();

      // Skip empty tokens
      if token.is_empty() {
        return None;
      }

      // Skip move numbers (e.g., "1.", "2.", "10.")
      if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return None;
      }

      // Skip comments and annotations (anything in parentheses or brackets)
      if token.starts_with('(') || token.starts_with('[') || token.starts_with('{') {
        return None;
      }

      // Clean up any trailing punctuation that isn't part of the move
      let clean_move = token
        .trim_end_matches(')')
        .trim_end_matches(']')
        .trim_end_matches('}')
        .trim_end_matches(',')
        .trim_end_matches(';');

      // Only include actual moves (not empty after cleaning)
      if !clean_move.is_empty() && clean_move.chars().any(|c| c.is_ascii_alphabetic()) {
        Some(clean_move.to_string())
      } else {
        None
      }
    })
    .collect()
}

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("openings.rs");

  // Download and parse all TSV files
  let mut openings = HashMap::new();

  for letter in ['a', 'b', 'c', 'd', 'e'] {
    let url = format!(
      "https://github.com/lichess-org/chess-openings/raw/refs/heads/master/{}.tsv",
      letter
    );

    println!("cargo:warning=Downloading {}", url);

    let response = reqwest::blocking::get(&url).expect("Failed to download TSV file");

    let content = response.text().expect("Failed to read response as text");

    let mut reader = csv::ReaderBuilder::new()
      .delimiter(b'\t')
      .from_reader(content.as_bytes());

    for result in reader.deserialize() {
      let opening: Opening = result.expect("Failed to parse TSV row");
      // Use uppercase name as key for case-insensitive lookup
      openings.insert(opening.name.to_uppercase(), opening);
    }
  }

  // Generate the Rust code
  let mut file = File::create(&dest_path).unwrap();

  writeln!(file, "use std::collections::HashMap;").unwrap();
  writeln!(file, "use once_cell::sync::Lazy;").unwrap();
  writeln!(file, "").unwrap();
  writeln!(file, "#[derive(Debug, Clone)]").unwrap();
  writeln!(file, "pub struct Opening {{").unwrap();
  writeln!(file, "    pub eco: &'static str,").unwrap();
  writeln!(file, "    pub name: &'static str,").unwrap();
  writeln!(file, "    pub pgn: &'static str,").unwrap();
  writeln!(file, "    pub moves: &'static [&'static str],").unwrap();
  writeln!(file, "}}").unwrap();
  writeln!(file, "").unwrap();

  writeln!(
    file,
    "pub static OPENINGS: Lazy<HashMap<&'static str, Opening>> = Lazy::new(|| {{"
  )
  .unwrap();
  writeln!(file, "    let mut map = HashMap::new();").unwrap();

  for (name, opening) in &openings {
    let moves = parse_pgn_moves(&opening.pgn);
    writeln!(file, "    map.insert({:?}, Opening {{", name).unwrap();
    writeln!(file, "        eco: {:?},", opening.eco).unwrap();
    writeln!(file, "        name: {:?},", opening.name).unwrap();
    writeln!(file, "        pgn: {:?},", opening.pgn).unwrap();
    writeln!(file, "        moves: &{:?},", moves).unwrap();
    writeln!(file, "    }});").unwrap();
  }

  writeln!(file, "    map").unwrap();
  writeln!(file, "}});").unwrap();

  println!("cargo:warning=Generated {} openings", openings.len());
}
