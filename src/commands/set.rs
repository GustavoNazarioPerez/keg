use crate::commands::ParseError;
use crate::store::Store;

pub struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn run(&self, _store: &mut Store) {
        println!("set {} = {}", self.key, self.value);
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Set, ParseError> {
        let key = args.next().ok_or(ParseError::MissingArgument("key"))?;
        let value: Vec<String> = args.collect();
        if value.is_empty() {
            return Err(ParseError::MissingArgument("value"));
        }
        Ok(Set {
            key,
            value: value.join(" "),
        })
    }
}
