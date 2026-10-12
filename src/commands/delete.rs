use crate::commands::ParseError;

pub struct Delete {
    key: String,
}

impl Delete {
    pub fn run(&self) {
        println!("delete {}", self.key);
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Delete, ParseError> {
        let key = args.next().ok_or(ParseError::MissingArgument("key"))?;
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Delete { key })
    }
}
