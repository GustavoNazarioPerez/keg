use crate::commands::ParseError;
use crate::store::Store;

pub struct Count;

impl Count {
    pub fn run(&self, _store: &mut Store) {
        println!("count");
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Count, ParseError> {
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Count)
    }
}
