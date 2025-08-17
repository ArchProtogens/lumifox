# lumifox_chess_proc

A procedural macro crate that provides compile-time FEN string parsing for the `lumifox_chess` library.

## Overview

This crate provides the `fen!()` macro that parses FEN (Forsyth-Edwards Notation) strings at compile time, ensuring that only valid chess positions can be used in your code.

## Features

- **Compile-time FEN validation**: Invalid FEN strings cause compilation errors
- **Zero runtime overhead**: FEN parsing happens at compile time
- **Type safety**: Returns properly typed `GameData` instances
- **Comprehensive error reporting**: Clear error messages for invalid FEN strings

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lumifox_chess = { path = "../chess" }
lumifox_chess_proc = { path = "../proc" }
```

Then use the `fen!()` macro in your code:

```rust
use lumifox_chess::model::gamedata::GameData;
use lumifox_chess_proc::fen;

fn main() {
    // Parse the starting position at compile time
    let start_pos: GameData = fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // Parse a complex mid-game position
    let kiwipete: GameData = fen!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

    // Parse a position with en passant
    let en_passant_pos: GameData = fen!("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");

    // This would cause a compile-time error:
    // let invalid: GameData = fen!("invalid_fen_string");
}
```

## FEN String Format

The `fen!()` macro accepts standard FEN notation with all six components:

1. **Piece placement**: Board state from rank 8 to rank 1
2. **Active colour**: "w" for white, "b" for black
3. **Castling rights**: Combination of "K", "Q", "k", "q", or "-" for none
4. **En passant target**: Target square or "-" for none
5. **Half move clock**: Number of half moves since last capture or pawn move
6. **Full move number**: Number of full moves (incremented after black's turn)

## Error Handling

If an invalid FEN string is provided, the macro will generate a compile-time error with a descriptive message:

```rust
let invalid: GameData = fen!("invalid_fen_string");
// Compile error: Invalid FEN string: MalformedFen
```

## Examples

See the `examples/` directory for comprehensive usage examples:

- `fen_macro_test.rs`: Demonstrates successful parsing of various positions
- `fen_macro_error_test.rs`: Shows compile-time error handling

Run the examples with:

```bash
cargo run --example fen_macro_test
```

## Licence

This library is licensed under the LGPLv3. See the `LICENCE-LGPL-3.0.md` file for details.
