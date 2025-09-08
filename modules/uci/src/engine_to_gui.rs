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

use std::fmt::Display;

use lumifox_chess::model::piecemove::PieceMove;

/// Commands sent from the engine to the GUI
#[derive(Debug, Clone, PartialEq)]
pub enum EngineToGuiCommand {
  /// Identify the engine
  Id {
    name: Option<String>,
    author: Option<String>,
  },

  /// Acknowledge UCI mode
  UciOk,

  /// Engine is ready to accept new commands
  ReadyOk,

  /// Best move found
  BestMove {
    bestmove: PieceMove,
    ponder: Option<PieceMove>,
  },

  /// Copy protection status
  CopyProtection { status: ProtectionStatus },

  /// Registration status
  Registration { status: RegistrationStatus },

  /// Information about the search
  Info { info: Vec<InfoType> },

  /// Engine options that can be changed
  Option { option: OptionType },
}

/// Copy protection status
#[derive(Debug, Clone, PartialEq)]
pub enum ProtectionStatus {
  Checking,
  Ok,
  Error,
}

/// Registration status
#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationStatus {
  Checking,
  Ok,
  Error,
}

/// Information types for the info command
#[derive(Debug, Clone, PartialEq)]
pub enum InfoType {
  /// Search depth in plies
  Depth(u32),
  /// Selective search depth in plies
  SelDepth(u32),
  /// Time searched in ms
  Time(u64),
  /// Nodes searched
  Nodes(u64),
  /// Best line found
  Pv(Vec<PieceMove>),
  /// Multi PV mode number
  MultiPv(u32),
  /// Score information
  Score(ScoreType),
  /// Currently searching this move
  CurrMove(PieceMove),
  /// Currently searching move number
  CurrMoveNumber(u32),
  /// Hash table fill level (per mill)
  HashFull(u32),
  /// Nodes per second
  Nps(u64),
  /// Tablebase hits
  TbHits(u64),
  /// Shredder database hits
  SbHits(u64),
  /// CPU usage (per mill)
  CpuLoad(u32),
  /// String information
  String(String),
  /// Move refutation
  Refutation {
    refuted_move: PieceMove,
    refutation_line: Vec<PieceMove>,
  },
  /// Current line being calculated
  CurrLine {
    cpu_nr: Option<u32>,
    line: Vec<PieceMove>,
  },
}

/// Score types
#[derive(Debug, Clone, PartialEq)]
pub enum ScoreType {
  /// Score in centipawns
  Cp {
    value: i32,
    bound: Option<ScoreBound>,
  },
  /// Mate in y moves
  Mate {
    moves: i32,
    bound: Option<ScoreBound>,
  },
}

/// Score bounds
#[derive(Debug, Clone, PartialEq)]
pub enum ScoreBound {
  /// Score is a lower bound
  LowerBound,
  /// Score is an upper bound
  UpperBound,
}

/// Engine option types
#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
  /// Checkbox option
  Check { name: String, default: bool },
  /// Spin wheel option (integer in range)
  Spin {
    name: String,
    default: i32,
    min: i32,
    max: i32,
  },
  /// Combo box option
  Combo {
    name: String,
    default: String,
    vars: Vec<String>,
  },
  /// Button option
  Button { name: String },
  /// String option
  String { name: String, default: String },
}

impl Display for EngineToGuiCommand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      EngineToGuiCommand::Id { name, author } => fmt_id(name, author),
      EngineToGuiCommand::UciOk => "uciok\n".to_string(),
      EngineToGuiCommand::ReadyOk => "readyok\n".to_string(),
      EngineToGuiCommand::BestMove { bestmove, ponder } => fmt_bestmove(bestmove, ponder),
      EngineToGuiCommand::CopyProtection { status } => fmt_copyprotection(status),
      EngineToGuiCommand::Registration { status } => fmt_registration(status),
      EngineToGuiCommand::Info { info } => fmt_info(info),
      EngineToGuiCommand::Option { option } => fmt_option(option),
    };

    write!(f, "{s}")
  }
}

// Helper functions for formatting

fn fmt_id(name: &Option<String>, author: &Option<String>) -> String {
  let mut string = String::new();
  if let Some(name) = name {
    string.push_str(&format!("id name {name}\n"));
  }
  if let Some(author) = author {
    string.push_str(&format!("id author {author}\n"));
  }
  string
}

fn fmt_bestmove(bestmove: &PieceMove, ponder: &Option<PieceMove>) -> String {
  let mut string = format!("bestmove {bestmove}");
  if let Some(ponder) = ponder {
    string.push_str(&format!(" ponder {ponder}"));
  }
  string.push('\n');
  string
}

fn fmt_copyprotection(status: &ProtectionStatus) -> String {
  let status_str = match status {
    ProtectionStatus::Checking => "checking",
    ProtectionStatus::Ok => "ok",
    ProtectionStatus::Error => "error",
  };
  format!("copyprotection {status_str}\n")
}

fn fmt_registration(status: &RegistrationStatus) -> String {
  let status_str = match status {
    RegistrationStatus::Checking => "checking",
    RegistrationStatus::Ok => "ok",
    RegistrationStatus::Error => "error",
  };
  format!("registration {status_str}\n")
}

fn fmt_info(info: &[InfoType]) -> String {
  // Helper to format ScoreType
  #[inline(always)]
  fn fmt_score(score: &ScoreType) -> String {
    match score {
      ScoreType::Cp { value, bound } => {
        let mut s = format!("score cp {value}");
        if let Some(b) = bound {
          let bstr = match b {
            ScoreBound::LowerBound => " lowerbound",
            ScoreBound::UpperBound => " upperbound",
          };
          s.push_str(bstr);
        }
        s
      }
      ScoreType::Mate { moves, bound } => {
        let mut s = format!("score mate {moves}");
        if let Some(b) = bound {
          let bstr = match b {
            ScoreBound::LowerBound => " lowerbound",
            ScoreBound::UpperBound => " upperbound",
          };
          s.push_str(bstr);
        }
        s
      }
    }
  }

  let mut line = "info".to_string();
  for it in info {
    match it {
      InfoType::Depth(d) => line.push_str(&format!(" depth {d}")),
      InfoType::SelDepth(d) => line.push_str(&format!(" seldepth {d}")),
      InfoType::Time(ms) => line.push_str(&format!(" time {ms}")),
      InfoType::Nodes(n) => line.push_str(&format!(" nodes {n}")),
      InfoType::Pv(moves) => {
        if !moves.is_empty() {
          line.push_str(" pv");
          for mv in moves {
            line.push_str(&format!(" {mv}"));
          }
        }
      }
      InfoType::MultiPv(n) => line.push_str(&format!(" multipv {n}")),
      InfoType::Score(s) => line.push_str(&format!(" {}", fmt_score(s))),
      InfoType::CurrMove(m) => line.push_str(&format!(" currmove {m}")),
      InfoType::CurrMoveNumber(n) => line.push_str(&format!(" currmovenumber {n}")),
      InfoType::HashFull(p) => line.push_str(&format!(" hashfull {p}")),
      InfoType::Nps(n) => line.push_str(&format!(" nps {n}")),
      InfoType::TbHits(n) => line.push_str(&format!(" tbhits {n}")),
      InfoType::SbHits(n) => line.push_str(&format!(" sbhits {n}")),
      InfoType::CpuLoad(p) => line.push_str(&format!(" cpuload {p}")),
      InfoType::String(s) => line.push_str(&format!(" string {s}")),
      InfoType::Refutation {
        refuted_move,
        refutation_line,
      } => {
        line.push_str(&format!(" refutation {refuted_move}"));
        for mv in refutation_line {
          line.push_str(&format!(" {mv}"));
        }
      }
      InfoType::CurrLine {
        cpu_nr,
        line: moves,
      } => {
        line.push_str(" currline");
        if let Some(cpu) = cpu_nr {
          line.push_str(&format!(" {cpu}"));
        }
        for mv in moves {
          line.push_str(&format!(" {mv}"));
        }
      }
    }
  }
  line.push('\n');
  line
}

fn fmt_option(option: &OptionType) -> String {
  let mut out = String::from("option");
  match option {
    OptionType::Check { name, default } => {
      out.push_str(&format!(" name {name} type check default {default}"));
    }
    OptionType::Spin {
      name,
      default,
      min,
      max,
    } => {
      out.push_str(&format!(
        " name {name} type spin default {default} min {min} max {max}"
      ));
    }
    OptionType::Combo {
      name,
      default,
      vars,
    } => {
      out.push_str(&format!(" name {name} type combo default {default}"));
      for v in vars {
        out.push_str(&format!(" var {v}"));
      }
    }
    OptionType::Button { name } => {
      out.push_str(&format!(" name {name} type button"));
    }
    OptionType::String { name, default } => {
      out.push_str(&format!(" name {name} type string default {default}"));
    }
  }
  out.push('\n');
  out
}
