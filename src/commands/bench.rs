use crate::commands::ParseError;

pub struct Bench;

impl Bench {
    pub fn run(&self) {
        println!("bench");
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Bench, ParseError> {
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Bench)
    }
}
