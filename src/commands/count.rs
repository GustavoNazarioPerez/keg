use crate::commands::ParseError;

pub struct Count;

impl Count {
    pub fn run(&self) {
        println!("count");
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Count, ParseError> {
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Count)
    }
}
