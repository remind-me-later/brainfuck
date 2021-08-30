use std::fmt;

pub enum Warning {
    NOP(usize, usize, String),
}

impl Warning {
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

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NOP(_, _, ir_instruction) => write!(f, "no operation: \"{}\"", ir_instruction),
        }
    }
}
