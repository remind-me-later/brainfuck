use thiserror::Error;

pub type ParserResult = Result<(), ParserError>;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unbalanced bracket: \"{}\"", .1)]
    MismatchedBracket(usize, String),
}

impl ParserError {
    pub fn beginning(&self) -> usize {
        match self {
            Self::MismatchedBracket(position, _) => *position,
        }
    }
}
