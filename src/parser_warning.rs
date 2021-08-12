use std::fmt;

pub enum ParserWarning {
    NOP(usize, usize, String),
}

impl ParserWarning {
    pub fn beginning(&self) -> usize {
        match self {
            Self::NOP(beginning, _, _) => *beginning,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            Self::NOP(_, end, _) => *end,
        }
    }

    pub fn line(&self) -> String {
        match self {
            Self::NOP(_, _, line) => line.clone(),
        }
    }
}

impl fmt::Display for ParserWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NOP(_, _, _) => write!(f, "no operation",),
        }
    }
}
