use crate::commands::ParseError;

pub struct Scan {
    prefix: Option<String>,
}

impl Scan {
    pub fn run(&self) {
        match &self.prefix {
            Some(prefix) => println!("scan {prefix}"),
            None => println!("scan"),
        }
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Scan, ParseError> {
        let prefix = args.next();
        if let Some(extra) = args.next() {
            return Err(ParseError::UnexpectedArgument(extra));
        }
        Ok(Scan { prefix })
    }
}
