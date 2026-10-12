mod bench;
mod compact;
mod count;
mod delete;
mod get;
mod scan;
mod set;
mod stats;

use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidCommand(String),
    MissingArgument(&'static str),
    UnexpectedArgument(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidCommand(c) => write!(f, "error: invalid command {c}"),
            ParseError::MissingArgument(a) => write!(f, "error: missing argument <{a}>"),
            ParseError::UnexpectedArgument(a) => write!(f, "error: unexpected argument {a}"),
        }
    }
}

impl std::error::Error for ParseError {}

pub enum Command {
    Set(set::Set),
    Get(get::Get),
    Delete(delete::Delete),
    Scan(scan::Scan),
    Count(count::Count),
    Stats(stats::Stats),
    Compact(compact::Compact),
    Bench(bench::Bench),
}

impl Command {
    pub fn from_args(
        command: &str,
        mut args: impl Iterator<Item = String>,
    ) -> Result<Command, ParseError> {
        match command {
            "set" => Ok(Command::Set(set::Set::from_args(&mut args)?)),
            "get" => Ok(Command::Get(get::Get::from_args(&mut args)?)),
            "delete" => Ok(Command::Delete(delete::Delete::from_args(&mut args)?)),
            "scan" => Ok(Command::Scan(scan::Scan::from_args(&mut args)?)),
            "count" => Ok(Command::Count(count::Count::from_args(&mut args)?)),
            "stats" => Ok(Command::Stats(stats::Stats::from_args(&mut args)?)),
            "compact" => Ok(Command::Compact(compact::Compact::from_args(&mut args)?)),
            "bench" => Ok(Command::Bench(bench::Bench::from_args(&mut args)?)),
            _ => Err(ParseError::InvalidCommand(String::from(command))),
        }
    }

    pub fn run(&self) {
        match self {
            Command::Set(cmd) => cmd.run(),
            Command::Get(cmd) => cmd.run(),
            Command::Delete(cmd) => cmd.run(),
            Command::Scan(cmd) => cmd.run(),
            Command::Count(cmd) => cmd.run(),
            Command::Stats(cmd) => cmd.run(),
            Command::Compact(cmd) => cmd.run(),
            Command::Bench(cmd) => cmd.run(),
        }
    }
}
