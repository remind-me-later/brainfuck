use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unbalanced \"[\"")]
    MismatchedOpen(usize),
    #[error("unbalanced \"]\"")]
    MismatchedClose(usize),
}
