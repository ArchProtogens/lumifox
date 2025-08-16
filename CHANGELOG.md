# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-08-16

### Added

- Initial release candidate for the Lumifox chess engine workspace.
- Precomputed rays table and related sliding-attack performance improvements in the chess module.
- `engine` and `uci` packages: added `description` and `repository` metadata for packaging and publishing.

### Changed

- Refactored `GameBoard` internals to improve safety and performance (see BREAKING CHANGE).
- Examples and move-generation internals refactored to use iterator-based loops and updated bitboard accessors.

### Removed

- Deleted legacy benches file used for ad-hoc benchmarking. Benchmarks can be migrated to `criterion` later.

### BREAKING CHANGES

- `modules/chess::model::GameBoard` API changes:
  - `clear_square` no longer returns the cleared `PieceType`. It now returns `Option<()>` and always clears all piece bitboards and the color bit for the square.
  - `set_square` and `move_piece` now return `Option<()>` to surface failure modes to callers.
  - Callers which relied on the previous `clear_square` return value must now call `get_piece` before clearing the square.

### Notes

- All unit tests for `lumifox_chess` passed locally (163 tests) and release build succeeded.
- No open issues or pull requests were present when this release was prepared.
