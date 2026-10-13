use crate::commands::ParseError;
use crate::store::Store;

pub struct Compact;

impl Compact {
    pub fn run(&self, _store: &mut Store) {
        println!("compact");
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Compact, ParseError> {
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Compact)
    }
}
