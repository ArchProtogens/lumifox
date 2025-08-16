# lumifox_chess

A high-performance, no_std-capable chess engine library providing bitboard representations
and move generation used by the Lumifox project.

## Quick example

```rust
use lumifox_chess::model::GameData;

let game = GameData::from_fen("startpos");
println!("Game: {:?}", game);
```

See the crate docs for more details.
