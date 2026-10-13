use crate::commands::ParseError;
use crate::store::Store;

pub struct Bench;

impl Bench {
    pub fn run(&self, _store: &mut Store) {
        println!("bench");
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Bench, ParseError> {
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Bench)
    }
}
