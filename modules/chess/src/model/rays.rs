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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rays_table_nonzero() {
    let mut found = 0;
    for sq in 0..64 {
      for d in 0..8 {
        found += (RAYS[sq][d] != 0) as usize;
      }
    }
    assert!(found > 0);
  }
}
