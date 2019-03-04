use std::error::Error as BaseError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Unknown,
    Syntax
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl BaseError for Error {
    fn description(&self) -> &str {
        let kind = match &self.kind {
            ErrorKind::Unknown => {
                String::from("Unknown error!")
            },
            ErrorKind::Syntax => {
                String::from("Syntax error!")
            }
        };
        let description = format!("{} {}", kind, self.message);
        
        let ptr = description.as_str().as_ptr();
        let len = description.as_str().len();

        let st = unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8(slice).unwrap()
        };
        st
    }

    fn cause(&self) -> Option<&BaseError> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
