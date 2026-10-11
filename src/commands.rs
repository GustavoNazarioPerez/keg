use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidCommandError(pub String);

impl std::error::Error for InvalidCommandError {}

impl fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = &self.0;
        write!(f, "error: invalid command {command}")
    }
}

pub enum Command {
    Set,
    Get,
    Delete,
    Scan,
    Count,
    Stats,
    Compact,
    Bench,
}

impl FromStr for Command {
    type Err = InvalidCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" => Ok(Command::Set),
            "get" => Ok(Command::Get),
            "delete" => Ok(Command::Delete),
            "scan" => Ok(Command::Scan),
            "count" => Ok(Command::Count),
            "stats" => Ok(Command::Stats),
            "compact" => Ok(Command::Compact),
            "bench" => Ok(Command::Bench),
            _ => Err(InvalidCommandError(String::from(s))),
        }
    }
}
