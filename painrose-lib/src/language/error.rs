#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub kind: ParseErrorKind
}

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidPrefixError,
    BadCoordinateError
}