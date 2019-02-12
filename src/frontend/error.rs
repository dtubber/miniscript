pub enum ErrorKind {
    Unknown
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String
}