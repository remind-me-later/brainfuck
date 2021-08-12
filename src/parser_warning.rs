use std::fmt;

use crate::string_utils::CharPosition;

pub enum ParserWarning {
    UselessExpression(CharPosition, CharPosition),
}

impl fmt::Display for ParserWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UselessExpression(beginning, end) => write!(
                f,
                "useless expression beginning at {} and ending at {}",
                beginning, end
            ),
        }
    }
}

