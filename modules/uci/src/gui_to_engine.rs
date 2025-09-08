/*
 * This file is dual-licensed under the terms of the GNU Lesser General Public License,
 * Version 3 or later, and the BSD 3-Clause License.
 *
 * You are free to use this software under the terms of either licence.
 * See the `LICENCE-LGPL-3.0-or-later.md` and `LICENCE-BSD-3-Clause.md`
 * files in this repository for the full text of each licence.
 *
 * If the files have not been provided, you can find the full text of the licences at:
 * LGPL-3.0-or-later: https://opensource.org/license/lgpl-3-0
 * BSD-3-Clause: https://opensource.org/license/bsd-3-clause
 *
 * Copyright (C) 2025 Clifton Toaster Reid
 */

use lumifox_chess::model::{gamedata::GameData, piecemove::PieceMove};

use crate::error::UciError;
use std::str::FromStr;

/// Commands sent from the GUI to the engine
#[derive(Debug, Clone)]
pub enum GuiToEngineCommand {
  /// Tell engine to use the UCI (Universal Chess Interface)
  Uci,

  /// Switch the debug mode of the engine on and off
  Debug { on: bool },

  /// Used to synchronize the engine with the GUI
  IsReady,

  /// Set internal engine parameters
  SetOption { name: String, value: Option<String> },

  /// Register the engine with a name and/or code
  Register {
    later: bool,
    name: Option<String>,
    code: Option<String>,
  },

  /// Indicates the next search will be from a different game
  UciNewGame,

  /// Set up a position on the internal board
  Position {
    /// Either a FEN string or indicates starting position
    position: Box<PositionType>,
    /// Moves to play from the position
    moves: Vec<PieceMove>,
  },

  /// Start calculating on the current position
  Go {
    /// Restrict search to these moves only
    searchmoves: Option<Vec<PieceMove>>,
    /// Start searching in pondering mode
    ponder: bool,
    /// White has x milliseconds left on the clock
    wtime: Option<u64>,
    /// Black has x milliseconds left on the clock
    btime: Option<u64>,
    /// White increment per move in milliseconds
    winc: Option<u64>,
    /// Black increment per move in milliseconds
    binc: Option<u64>,
    /// Moves to the next time control
    movestogo: Option<u32>,
    /// Search x plies only
    depth: Option<u32>,
    /// Search x nodes only
    nodes: Option<u64>,
    /// Search for a mate in x moves
    mate: Option<u32>,
    /// Search exactly x milliseconds
    movetime: Option<u64>,
    /// Search until the "stop" command
    infinite: bool,
  },

  /// Stop calculating as soon as possible
  Stop,

  /// The user has played the expected move (during pondering)
  PonderHit,

  /// Quit the program as soon as possible
  Quit,
}

/// Position type for the position command
#[derive(Debug, Clone)]
pub enum PositionType {
  /// Starting position
  StartPos { moves: Vec<PieceMove> },
  /// Position from FEN string (parsed)
  Fen {
    gamedata: Box<GameData>,
    moves: Vec<PieceMove>,
  },
}

impl FromStr for GuiToEngineCommand {
  type Err = UciError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let args = s.split_whitespace().collect::<Vec<_>>();

    if args.is_empty() {
      return Err(UciError::Parser("Empty command".to_string()));
    }

    match *args.first().unwrap() {
      "uci" => Ok(GuiToEngineCommand::Uci),
      "debug" => {
        if args.len() != 2 {
          return Err(UciError::Parser(
            "Invalid number of arguments for debug command".to_string(),
          ));
        }
        match args[1] {
          "on" => Ok(GuiToEngineCommand::Debug { on: true }),
          "off" => Ok(GuiToEngineCommand::Debug { on: false }),
          _ => Err(UciError::Parser(
            "Invalid argument for debug command".to_string(),
          )),
        }
      }
      "isready" => Ok(GuiToEngineCommand::IsReady),
      "setoption" => {
        if args.len() < 3 || args[1] != "name" {
          return Err(UciError::Parser(
            "Invalid setoption command format".to_string(),
          ));
        }
        let name_start = 2;
        let value_pos = args.iter().position(|&s| s == "value");
        let (name, value) = if let Some(vp) = value_pos {
          if vp <= name_start {
            return Err(UciError::Parser(
              "Invalid setoption command format".to_string(),
            ));
          }
          let name = args[name_start..vp].join(" ");
          let value = if vp + 1 < args.len() {
            Some(args[vp + 1..].join(" "))
          } else {
            None
          };
          (name, value)
        } else {
          (args[name_start..].join(" "), None)
        };
        Ok(GuiToEngineCommand::SetOption { name, value })
      }
      "register" => {
        if args.len() < 2 {
          return Err(UciError::Parser(
            "Invalid number of arguments for register command".to_string(),
          ));
        }
        let word = args[1];
        let rest = args[2..].join(" ");

        // Expect either "startpos" or "fen"
        match word {
          "later" => Ok(GuiToEngineCommand::Register {
            later: true,
            name: None,
            code: None,
          }),
          "name" => Ok(GuiToEngineCommand::Register {
            later: false,
            name: Some(rest),
            code: None,
          }),
          "code" => Ok(GuiToEngineCommand::Register {
            later: false,
            name: None,
            code: Some(rest),
          }),
          _ => Err(UciError::Parser(
            "Invalid argument for register command".to_string(),
          )),
        }
      }
      "ucinewgame" => Ok(GuiToEngineCommand::UciNewGame),
      "position" => {
        if args.len() < 2 {
          return Err(UciError::Parser(
            "Invalid number of arguments for position command".to_string(),
          ));
        }

        let mut idx = 1;
        let mut moves: Vec<PieceMove> = Vec::new();

        // Expect either "startpos" or "fen"
        if args[idx] == "startpos" {
          // startpos may be followed by "moves ..."
          idx += 1;
          if idx < args.len() && args[idx] == "moves" {
            idx += 1;
            while idx < args.len() {
              moves.push(PieceMove::from_str(args[idx])?);
              idx += 1;
            }
          }

          let pos_type = PositionType::StartPos {
            moves: moves.clone(),
          };
          return Ok(GuiToEngineCommand::Position {
            position: Box::new(pos_type),
            moves,
          });
        }

        // Otherwise expect a fen
        if args[idx] == "fen" {
          // FEN has six space-separated fields. Collect next six tokens (or until "moves").
          idx += 1;
          let mut fen_parts: Vec<&str> = Vec::new();
          while idx < args.len() && args[idx] != "moves" && fen_parts.len() < 6 {
            fen_parts.push(args[idx]);
            idx += 1;
          }

          if fen_parts.len() < 6 {
            return Err(UciError::Parser(
              "Incomplete FEN in position command".to_string(),
            ));
          }

          let fen = fen_parts.join(" ");

          // If there's a "moves" token, collect following tokens as moves
          if idx < args.len() && args[idx] == "moves" {
            idx += 1;
            while idx < args.len() {
              moves.push(PieceMove::from_str(args[idx])?);
              idx += 1;
            }
          }

          // Parse FEN into GameData. Logic for applying moves to the GameData
          // (e.g., mutating gamedata.moves / plies) is left to the engine logic.
          let gamedata = GameData::from_fen(&fen)
            .map_err(|e| UciError::Parser(format!("Invalid FEN: {e:?}")))?;

          let pos_type = PositionType::Fen {
            gamedata: Box::new(gamedata),
            moves: moves.clone(),
          };
          return Ok(GuiToEngineCommand::Position {
            position: Box::new(pos_type),
            moves,
          });
        }

        Err(UciError::Parser(
          "Invalid position command, expected 'startpos' or 'fen'".to_string(),
        ))
      }
      "go" => {
        // defaults
        let mut idx = 1;
        let mut searchmoves: Option<Vec<PieceMove>> = None;
        let mut ponder = false;
        let mut wtime: Option<u64> = None;
        let mut btime: Option<u64> = None;
        let mut winc: Option<u64> = None;
        let mut binc: Option<u64> = None;
        let mut movestogo: Option<u32> = None;
        let mut depth: Option<u32> = None;
        let mut nodes: Option<u64> = None;
        let mut mate: Option<u32> = None;
        let mut movetime: Option<u64> = None;
        let mut infinite = false;

        while idx < args.len() {
          match args[idx] {
            "searchmoves" => {
              idx += 1;
              let mut moves = Vec::new();
              while idx < args.len() {
                // next token is either another keyword or a move; in UCI searchmoves should be
                // followed by only move tokens until end of string. We'll accept tokens until we
                // hit a known keyword to be safe.
                let kw = args[idx];
                if kw == "ponder"
                  || kw == "wtime"
                  || kw == "btime"
                  || kw == "winc"
                  || kw == "binc"
                  || kw == "movestogo"
                  || kw == "depth"
                  || kw == "nodes"
                  || kw == "mate"
                  || kw == "movetime"
                  || kw == "infinite"
                {
                  break;
                }
                moves.push(PieceMove::from_str(kw)?);
                idx += 1;
              }
              searchmoves = Some(moves);
              continue;
            }
            "ponder" => {
              ponder = true;
              idx += 1;
            }
            "wtime" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for wtime".to_string()));
              }
              wtime = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid wtime value".to_string()))?,
              );
              idx += 1;
            }
            "btime" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for btime".to_string()));
              }
              btime = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid btime value".to_string()))?,
              );
              idx += 1;
            }
            "winc" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for winc".to_string()));
              }
              winc = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid winc value".to_string()))?,
              );
              idx += 1;
            }
            "binc" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for binc".to_string()));
              }
              binc = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid binc value".to_string()))?,
              );
              idx += 1;
            }
            "movestogo" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for movestogo".to_string()));
              }
              movestogo = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid movestogo value".to_string()))?,
              );
              idx += 1;
            }
            "depth" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for depth".to_string()));
              }
              depth = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid depth value".to_string()))?,
              );
              idx += 1;
            }
            "nodes" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for nodes".to_string()));
              }
              nodes = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid nodes value".to_string()))?,
              );
              idx += 1;
            }
            "mate" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for mate".to_string()));
              }
              mate = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid mate value".to_string()))?,
              );
              idx += 1;
            }
            "movetime" => {
              idx += 1;
              if idx >= args.len() {
                return Err(UciError::Parser("Missing value for movetime".to_string()));
              }
              movetime = Some(
                args[idx]
                  .parse()
                  .map_err(|_| UciError::Parser("Invalid movetime value".to_string()))?,
              );
              idx += 1;
            }
            "infinite" => {
              infinite = true;
              idx += 1;
            }
            _ => {
              return Err(UciError::Parser(format!(
                "Unrecognized token in go command: {}",
                args[idx]
              )));
            }
          }
        }

        Ok(GuiToEngineCommand::Go {
          searchmoves,
          ponder,
          wtime,
          btime,
          winc,
          binc,
          movestogo,
          depth,
          nodes,
          mate,
          movetime,
          infinite,
        })
      }
      "stop" => Ok(GuiToEngineCommand::Stop),
      "ponderhit" => Ok(GuiToEngineCommand::PonderHit),
      "quit" => Ok(GuiToEngineCommand::Quit),
      _ => Err(UciError::Parser("Unrecognized command".to_string())),
    }
  }
}
