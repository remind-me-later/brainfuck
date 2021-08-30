use thiserror::Error;

#[derive(Error, Debug)]
pub enum Fatal {
    #[error("unbalanced bracket: \"{}\"", .1)]
    MismatchedBracket(usize, String),
}

impl Fatal {
    pub fn beginning(&self) -> usize {
        match self {
            Self::MismatchedBracket(position, _) => *position,
        }
    }
}

