# UCI module - Proof of Concept TODO

This module is an early proof-of-concept implementation of a UCI (Universal Chess Interface) protocol
handler for Lumifox. The current implementation provides typed enums for commands going to/from the
GUI and a parser for GUI->Engine commands. Next steps:

- [x] **COMPLETED**: Basic UCI protocol implementation with typed enums and parsing
  - Added typed enums for Engine→GUI messages (`engine_to_gui.rs`)
  - Added parser for GUI→Engine commands with position/go handling (`gui_to_engine.rs`)
  - Added UCI error types with `thiserror` integration
  - Implemented Display trait for modern command formatting
  - Integrated `PieceMove` type for type-safe move representations
  - Enhanced position handling with typed `GameData` parsing
  - Extracted helper functions for better code organization

- [ ] Hook this module into the main Lumifox engine so commands from GUIs actually drive the engine.
  - Define a small runtime adapter that translates `GuiToEngineCommand` into engine calls.
  - Provide an API for engine to send `EngineToGuiCommand` messages to the front-end.
  - Implement a trait-based handler system that calls appropriate functions when UCI commands are received.

- [x] **PARTIALLY COMPLETED**: Improve error handling
  - Added `MoveParseError` enum with detailed error variants for move parsing
  - Added `UciError` with `InvalidPieceMove` variant for robust error propagation
  - Integrated FEN parsing error handling with `GameData::from_fen()`
  - Replaced simple string-based parsing errors with structured error types
  - **STILL NEEDED**: Robust handling for malformed input (extra tokens, quoted strings, etc.)

- [x] **PARTIALLY COMPLETED**: Provide a developer-friendly API
  - Added typed `PieceMove` integration replacing string-based moves
  - Enhanced position handling with `GameData` struct instead of raw strings
  - Extracted formatting and parsing helper functions
  - **STILL NEEDED**: Builder helpers, typed setters/getters for options, ergonomic utilities for `Info` messages

- [ ] Tests and examples
  - Add unit tests for parsing (`GuiToEngineCommand::from_str`) covering edge cases.
  - Add serialization tests for `EngineToGuiCommand::to_string` outputs.
  - Provide small example program showing a loop reading UCI lines and interacting with the engine.

- [ ] Documentation and README
  - Document the module's public API and integration instructions.

- [ ] Performance & concurrency considerations
  - Evaluate bounded input parsing and I/O handling for responsiveness.

Notes:

- This is intentionally minimal and follows the UCI spec only closely enough for a POC.
- The implementation currently uses `thiserror` for error ergonomics. Consider whether to expose
  those types across module boundaries or keep them internal.

Suggested followups (separate PRs):

- Wire up UCI to an async or threaded command loop in `modules/uci`.
- Add an adapter in `engine/src` to accept `GuiToEngineCommand` and emit `EngineToGuiCommand`.
- Add integration test that runs the engine binary and exercises a short UCI handshake.
