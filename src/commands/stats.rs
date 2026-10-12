use crate::commands::ParseError;

pub struct Stats;

impl Stats {
    pub fn run(&self) {
        println!("stats");
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Stats, ParseError> {
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Stats)
    }
}
