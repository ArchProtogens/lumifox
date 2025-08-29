/*
 * A high-performance chess library licensed under the LGPLv3.
 * Copyright (C) 2025 Clifton Toaster Reid
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library. If not, see <https://opensource.org/license/lgpl-3-0>.
 */

//! Chess opening macros for compile-time opening lookup.

// Include the generated openings data
include!(concat!(env!("OUT_DIR"), "/openings.rs"));

/// Macro to look up chess openings by name at compile time.
/// The lookup is case-insensitive, so "Sicilian Defense", "sicilian defense",
/// and "SICILIAN DEFENSE" all work. The PGN is parsed into individual move strings
/// in standard algebraic notation (SAN) like "e4", "Nf3", "Bb5".
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::opening;
///
/// let sicilian = opening!("Sicilian Defense");
/// let ruy_lopez = opening!("ruy lopez");  // case-insensitive
/// let kings_indian = opening!("KING'S INDIAN DEFENSE");  // case-insensitive
///
/// println!("ECO: {}, PGN: {}", sicilian.eco, sicilian.pgn);
/// println!("Moves: {:?}", sicilian.moves);  // ["e4", "c5", "Nf3", ...]
///
/// // Access individual moves (these are in SAN notation)
/// for (i, move_str) in sicilian.moves.iter().enumerate() {
///     println!("{}. {}", i + 1, move_str);
///     // Note: These are SAN strings like "e4", "Nf3", not PieceMove objects
///     // To convert to PieceMove, you'd need to parse them in context of a game
/// }
/// ```
#[macro_export]
macro_rules! opening {
  ($name:expr) => {{
    use $crate::macros::openings::OPENINGS;

    let uppercase_name = $name.to_uppercase();
    OPENINGS
      .get(&uppercase_name as &str)
      .unwrap_or_else(|| {
        panic!(
          "Opening '{}' not found. Available openings can be listed with `opening_list!()`",
          $name
        )
      })
      .clone()
  }};
}

/// Macro to get a list of all available opening names.
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::opening_list;
///
/// let all_openings = opening_list!();
/// println!("Found {} openings", all_openings.len());
/// ```
#[macro_export]
macro_rules! opening_list {
  () => {{
    use $crate::macros::openings::OPENINGS;

    OPENINGS.keys().cloned().collect::<Vec<&'static str>>()
  }};
}

/// Macro to search for openings by partial name match.
///
/// # Examples
///
/// ```rust
/// use lumifox_chess_proc::opening_search;
///
/// let sicilian_variations = opening_search!("Sicilian");
/// let kings_openings = opening_search!("King");
/// ```
#[macro_export]
macro_rules! opening_search {
  ($pattern:expr) => {{
    use $crate::macros::openings::OPENINGS;

    OPENINGS
      .iter()
      .filter(|(name, _)| name.to_lowercase().contains(&$pattern.to_lowercase()))
      .map(|(name, opening)| (*name, opening.clone()))
      .collect::<Vec<(&'static str, $crate::macros::openings::Opening)>>()
  }};
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_opening_macro() {
    let sicilian = opening!("Sicilian Defense");
    assert_eq!(sicilian.eco, "B50");
    assert!(sicilian.pgn.contains("c5"));
    assert!(!sicilian.moves.is_empty());
    assert!(sicilian.moves.contains(&"e4"));
    assert!(sicilian.moves.contains(&"c5"));
  }

  #[test]
  fn test_opening_list() {
    let openings = opening_list!();
    assert!(!openings.is_empty());
    // Keys are now uppercase, but we can still find the opening by any case
    assert!(openings.contains(&"SICILIAN DEFENSE"));
  }

  #[test]
  fn test_opening_search() {
    let sicilian_openings = opening_search!("Sicilian");
    assert!(!sicilian_openings.is_empty());
    assert!(
      sicilian_openings
        .iter()
        .any(|(name, _)| name.contains("SICILIAN"))
    );
  }

  #[test]
  fn test_case_insensitive() {
    // Test different cases
    let sicilian1 = opening!("Sicilian Defense");
    let sicilian2 = opening!("sicilian defense");
    let sicilian3 = opening!("SICILIAN DEFENSE");

    assert_eq!(sicilian1.eco, sicilian2.eco);
    assert_eq!(sicilian2.eco, sicilian3.eco);
    assert_eq!(sicilian1.name, sicilian2.name);
    assert_eq!(sicilian1.moves, sicilian2.moves);
    assert_eq!(sicilian2.moves, sicilian3.moves);
  }

  #[test]
  fn test_moves_parsing() {
    let ruy_lopez = opening!("Ruy Lopez");
    assert_eq!(ruy_lopez.moves, &["e4", "e5", "Nf3", "Nc6", "Bb5"]);

    let sicilian = opening!("Sicilian Defense");
    assert!(sicilian.moves.len() >= 2);
    assert_eq!(sicilian.moves[0], "e4");
    assert_eq!(sicilian.moves[1], "c5");
  }

  #[test]
  #[should_panic(expected = "Opening 'Nonexistent Opening' not found")]
  fn test_opening_not_found() {
    let _ = opening!("Nonexistent Opening");
  }
}
