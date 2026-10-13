use crate::commands::ParseError;
use crate::store::Store;

pub struct Get {
    key: String,
}

impl Get {
    pub fn run(&self, _store: &mut Store) {
        println!("get {}", self.key);
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Get, ParseError> {
        let key = args.next().ok_or(ParseError::MissingArgument("key"))?;
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Get { key })
    }
}
