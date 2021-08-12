use thiserror::Error;

use crate::string_utils::CharPosition;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unbalanced \"[\" at {}", .0)]
    MismatchedOpen(CharPosition),
    #[error("unbalanced \"]\" at {}", .0)]
    MismatchedClose(CharPosition),
}
