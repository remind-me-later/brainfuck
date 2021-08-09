use std::fmt;

#[derive(Debug)]
pub struct LineChar {
    line: usize,
    character: usize,
}

impl Default for LineChar {
    fn default() -> Self {
        Self {
            line: 1,
            character: 1,
        }
    }
}

impl LineChar {
    pub fn new(line: usize, character: usize) -> Self {
        Self {
            line: line + 1,
            character: character + 1,
        }
    }
}

impl fmt::Display for LineChar {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "line: {}, character: {}",
            self.line, self.character
        )
    }
}
