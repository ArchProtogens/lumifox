# UCI module - Proof of Concept TODO

This module is an early proof-of-concept implementation of a UCI (Universal Chess Interface) protocol
handler for Lumifox. The current implementation provides typed enums for commands going to/from the
GUI and a parser for GUI->Engine commands. Next steps:

- [ ] Hook this module into the main Lumifox engine so commands from GUIs actually drive the engine.
  - Define a small runtime adapter that translates `GuiToEngineCommand` into engine calls.
  - Provide an API for engine to send `EngineToGuiCommand` messages to the front-end.

- [ ] Improve error handling
  - Replace simple string-based parsing errors with structured error types and mapping to proper
    I/O / parse errors.
  - Add robust handling for malformed input (extra tokens, quoted strings, etc.)

- [ ] Provide a developer-friendly API
  - Add builder helpers, typed setters and getters for options.
  - Add ergonomic utilities to construct `Info` messages and `Option` declarations.

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
