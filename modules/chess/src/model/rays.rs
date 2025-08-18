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

pub const DIR_OFFSETS: [i8; 8] = [1, -1, -8, 8, -7, -9, 9, 7];

// Order: E, W, N, S, NE, NW, SE, SW (matches DIR_OFFSETS above)
pub const RAYS: [[u64; 8]; 64] = build_rays();

// Precomputed knight moves for every square, built at compile time.
#[cfg(feature = "precomputed_rays")]
pub const KNIGHT_MOVES: [u64; 64] = build_knight_moves();

// Precomputed king moves for every square (one-step adjacency)
#[cfg(feature = "precomputed_rays")]
pub const KING_MOVES: [u64; 64] = build_king_moves();

// Pawn attack and push masks. Separate tables per color to keep callers simple.
// White pawns move "up" (left shift) in existing movegen; black pawns move "down" (right shift).
#[cfg(feature = "precomputed_rays")]
pub const PAWN_ATTACK_WHITE: [u64; 64] = build_pawn_attack_white();
#[cfg(feature = "precomputed_rays")]
pub const PAWN_ATTACK_BLACK: [u64; 64] = build_pawn_attack_black();
#[cfg(feature = "precomputed_rays")]
pub const PAWN_PUSH_WHITE: [u64; 64] = build_pawn_push_white();
#[cfg(feature = "precomputed_rays")]
pub const PAWN_PUSH_BLACK: [u64; 64] = build_pawn_push_black();

// Between and line masks for sliding pieces
#[cfg(feature = "precomputed_rays")]
pub static BETWEEN: [[u64; 64]; 64] = build_between();
#[cfg(feature = "precomputed_rays")]
pub static LINE: [[u64; 64]; 64] = build_line();

// Helper const-fn to build the rays table at compile time.
const fn build_rays() -> [[u64; 8]; 64] {
  let mut table: [[u64; 8]; 64] = [[0u64; 8]; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    let mut d: usize = 0;
    while d < 8 {
      table[sq][d] = build_ray_for(sq as i8, DIR_OFFSETS[d]);
      d += 1;
    }
    sq += 1;
  }
  table
}

// Build a ray mask for a single square and direction. Returns raw u64.
// Use rank/file deltas to avoid wrapping across board edges when stepping.
const fn build_ray_for(square: i8, dir: i8) -> u64 {
  let mut mask: u64 = 0;
  let rank = square / 8;
  let file = square % 8;

  // direction deltas (dr, df)
  let (dr, df) = match dir {
    1 => (0, 1),    // E
    -1 => (0, -1),  // W
    -8 => (-1, 0),  // N
    8 => (1, 0),    // S
    -7 => (-1, 1),  // NE
    -9 => (-1, -1), // NW
    9 => (1, 1),    // SE
    7 => (1, -1),   // SW
    _ => (0, 0),
  };

  let mut r = rank + dr;
  let mut f = file + df;
  while r >= 0 && r < 8 && f >= 0 && f < 8 {
    let cur = r * 8 + f;
    mask |= 1u64 << (cur as u8);
    r += dr;
    f += df;
  }

  mask
}

// Build knight move mask for a single square.
#[cfg(feature = "precomputed_rays")]
const fn build_knight_for(square: i8) -> u64 {
  let mut mask: u64 = 0;
  let rank = square / 8;
  let file = square % 8;

  // All 8 knight jumps as (dr, df)
  let deltas: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
  ];
  let mut i: usize = 0;
  while i < 8 {
    let (dr, df) = deltas[i];
    let r = rank + dr;
    let f = file + df;
    if r >= 0 && r < 8 && f >= 0 && f < 8 {
      let cur = r * 8 + f;
      mask |= 1u64 << (cur as u8);
    }
    i += 1;
  }

  mask
}

#[cfg(feature = "precomputed_rays")]
const fn build_knight_moves() -> [u64; 64] {
  let mut table: [u64; 64] = [0u64; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    table[sq] = build_knight_for(sq as i8);
    sq += 1;
  }
  table
}

// Build king move mask for a single square.
#[cfg(feature = "precomputed_rays")]
const fn build_king_for(square: i8) -> u64 {
  let mut mask: u64 = 0;
  let rank = square / 8;
  let file = square % 8;

  let deltas: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
  ];
  let mut i: usize = 0;
  while i < 8 {
    let (dr, df) = deltas[i];
    let r = rank + dr;
    let f = file + df;
    if r >= 0 && r < 8 && f >= 0 && f < 8 {
      let cur = r * 8 + f;
      mask |= 1u64 << (cur as u8);
    }
    i += 1;
  }

  mask
}

#[cfg(feature = "precomputed_rays")]
const fn build_king_moves() -> [u64; 64] {
  let mut table: [u64; 64] = [0u64; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    table[sq] = build_king_for(sq as i8);
    sq += 1;
  }
  table
}

// Pawn attack masks for white (attacks to NW and NE from pawn's perspective)
#[cfg(feature = "precomputed_rays")]
const fn build_pawn_attack_white() -> [u64; 64] {
  let mut table: [u64; 64] = [0u64; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    let rank = (sq as i8) / 8;
    let file = (sq as i8) % 8;
    let mut mask: u64 = 0;

    // white pawn attacks are from pawn on (r-1,f-1) -> square and (r-1,f+1)
    // But to get attack destinations from a pawn on `sq`, we compute where a pawn at sq would attack.
    // White pawns move up (rank+1), but in movegen they use << so pawn on sq attacks sq+7 and sq+9.
    // So we compute (r+1, f-1) and (r+1, f+1)
    let (r1, f1) = (rank + 1, file - 1);
    if r1 >= 0 && r1 < 8 && f1 >= 0 && f1 < 8 {
      mask |= 1u64 << ((r1 * 8 + f1) as u8);
    }
    let (r2, f2) = (rank + 1, file + 1);
    if r2 >= 0 && r2 < 8 && f2 >= 0 && f2 < 8 {
      mask |= 1u64 << ((r2 * 8 + f2) as u8);
    }

    table[sq] = mask;
    sq += 1;
  }
  table
}

// Pawn attack masks for black (attacks to SW and SE from pawn's perspective)
#[cfg(feature = "precomputed_rays")]
const fn build_pawn_attack_black() -> [u64; 64] {
  let mut table: [u64; 64] = [0u64; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    let rank = (sq as i8) / 8;
    let file = (sq as i8) % 8;
    let mut mask: u64 = 0;

    // Black pawns move down (rank-1) in board coordinates used earlier; their attacks are (r-1,f-1) and (r-1,f+1)
    let (r1, f1) = (rank - 1, file - 1);
    if r1 >= 0 && r1 < 8 && f1 >= 0 && f1 < 8 {
      mask |= 1u64 << ((r1 * 8 + f1) as u8);
    }
    let (r2, f2) = (rank - 1, file + 1);
    if r2 >= 0 && r2 < 8 && f2 >= 0 && f2 < 8 {
      mask |= 1u64 << ((r2 * 8 + f2) as u8);
    }

    table[sq] = mask;
    sq += 1;
  }
  table
}

// Pawn single-push masks: destination squares a pawn from `sq` could move into (without occupancy checks)
#[cfg(feature = "precomputed_rays")]
const fn build_pawn_push_white() -> [u64; 64] {
  let mut table: [u64; 64] = [0u64; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    let rank = (sq as i8) / 8;
    let file = (sq as i8) % 8;
    let mut mask: u64 = 0;
    let r = rank + 1;
    if r >= 0 && r < 8 {
      let cur = r * 8 + file;
      mask |= 1u64 << (cur as u8);
    }
    table[sq] = mask;
    sq += 1;
  }
  table
}

#[cfg(feature = "precomputed_rays")]
const fn build_pawn_push_black() -> [u64; 64] {
  let mut table: [u64; 64] = [0u64; 64];
  let mut sq: usize = 0;
  while sq < 64 {
    let rank = (sq as i8) / 8;
    let file = (sq as i8) % 8;
    let mut mask: u64 = 0;
    let r = rank - 1;
    if r >= 0 && r < 8 {
      let cur = r * 8 + file;
      mask |= 1u64 << (cur as u8);
    }
    table[sq] = mask;
    sq += 1;
  }
  table
}

// Build BETWEEN and LINE masks. Use RAYS to detect collinearity; BETWEEN contains squares strictly between
// from and to (exclusive), LINE contains the full line including endpoints.
#[cfg(feature = "precomputed_rays")]
const fn build_between() -> [[u64; 64]; 64] {
  let mut table: [[u64; 64]; 64] = [[0u64; 64]; 64];
  let mut from: usize = 0;
  while from < 64 {
    let mut to: usize = 0;
    while to < 64 {
      if from == to {
        table[from][to] = 0;
      } else {
        // For each direction, check if `to` is in the ray from `from` in that direction.
        let mut mask: u64 = 0;
        let mut d: usize = 0;
        while d < 8 {
          let ray = RAYS[from][d];
          if (ray & (1u64 << (to as u8))) != 0 {
            // Squares between are ray & ~((1<<from) | (1<<to)) trimmed to up-to target
            // Walk from 'from' towards 'to' accumulating squares until we reach 'to'.
            let mut cur_mask: u64 = 0;
            let mut sqi = from as i8;
            let dir = DIR_OFFSETS[d];
            sqi = sqi + dir;
            while sqi >= 0 && sqi < 64 {
              let idx = sqi as usize;
              if idx == to {
                break;
              }
              cur_mask |= 1u64 << (idx as u8);
              sqi = sqi + dir;
            }
            mask = cur_mask;
            break;
          }
          d += 1;
        }
        table[from][to] = mask;
      }
      to += 1;
    }
    from += 1;
  }
  table
}

#[cfg(feature = "precomputed_rays")]
const fn build_line() -> [[u64; 64]; 64] {
  let mut table: [[u64; 64]; 64] = [[0u64; 64]; 64];
  let mut from: usize = 0;
  while from < 64 {
    let mut to: usize = 0;
    while to < 64 {
      if from == to {
        table[from][to] = 1u64 << (from as u8);
      } else {
        // if `to` is in some ray from `from`, then line is between[from][to] | endpoints
        let between_mask = BETWEEN[from][to];
        if between_mask != 0 {
          table[from][to] = between_mask | (1u64 << (from as u8)) | (1u64 << (to as u8));
        } else {
          table[from][to] = 0;
        }
      }
      to += 1;
    }
    from += 1;
  }
  table
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::constants::*;

  #[test]
  fn rays_table_nonzero() {
    let mut found = 0;
    for row in RAYS.iter() {
      for &val in row.iter() {
        found += (val != 0) as usize;
      }
    }
    assert!(found > 0);
  }

  // Helper to build mask from list of square indices
  fn mask_from(indices: &[u8]) -> u64 {
    let mut m: u64 = 0;
    for &i in indices {
      m |= 1u64 << i;
    }
    m
  }

  #[test]
  #[cfg(feature = "precomputed_rays")]
  fn knight_moves_corners_and_center() {
    // A1 -> squares: B3, C2
    assert_eq!(KNIGHT_MOVES[A1 as usize], mask_from(&[B3, C2]));

    // D4 -> expected 8 knight destinations
    let expected = mask_from(&[E6, C6, E2, C2, F5, B5, F3, B3]);
    assert_eq!(KNIGHT_MOVES[D4 as usize], expected);
  }

  #[test]
  #[cfg(feature = "precomputed_rays")]
  fn king_moves_corner_and_center() {
    // A1 -> B1, A2, B2
    assert_eq!(KING_MOVES[A1 as usize], mask_from(&[B1, A2, B2]));

    // D4 neighbors
    let expected = mask_from(&[C3, D3, E3, C4, E4, C5, D5, E5]);
    assert_eq!(KING_MOVES[D4 as usize], expected);
  }

  #[test]
  #[cfg(feature = "precomputed_rays")]
  fn pawn_attack_and_push_masks() {
    // White pawn on C2 attacks B3 and D3
    assert_eq!(PAWN_ATTACK_WHITE[C2 as usize], mask_from(&[B3, D3]));

    // Black pawn on C7 attacks B6 and D6
    assert_eq!(PAWN_ATTACK_BLACK[C7 as usize], mask_from(&[B6, D6]));

    // Single pushes
    assert_eq!(PAWN_PUSH_WHITE[A2 as usize], mask_from(&[A3])); // A2 -> A3
    assert_eq!(PAWN_PUSH_BLACK[A7 as usize], mask_from(&[A6])); // A7 -> A6
  }

  #[test]
  #[cfg(feature = "precomputed_rays")]
  fn between_and_line_masks_diagonals_and_files() {
    // Between A1 and H8: B2..G7
    let between_diag = mask_from(&[B2, C3, D4, E5, F6, G7]);
    assert_eq!(BETWEEN[A1 as usize][H8 as usize], between_diag);

    // Line should include endpoints
    let line_diag = between_diag | mask_from(&[A1, H8]);
    assert_eq!(LINE[A1 as usize][H8 as usize], line_diag);

    // File A: between A1 and A8: A2..A7
    let between_file = mask_from(&[A2, A3, A4, A5, A6, A7]);
    assert_eq!(BETWEEN[A1 as usize][A8 as usize], between_file);
    assert_eq!(
      LINE[A1 as usize][A8 as usize],
      between_file | mask_from(&[A1, A8])
    );
  }
}
