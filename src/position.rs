use std::fmt;

#[derive(Debug)]
pub struct Position {
    line: usize,
    character: usize,
}

impl Position {
    pub fn new(line: usize, character: usize) -> Self {
        Self {
            line: line + 1,
            character: character + 1,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "line: {}, character: {}",
            self.line, self.character
        )
    }
}
