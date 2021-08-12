use std::fmt;

use crate::some_from::SomeFrom;

pub enum IRInstruction {
    NOP,
    Left(usize),
    Right(usize),
    Add(usize),
    Sub(usize),
    Input(usize),
    Output(usize),
    Open(usize),
    Close(usize),
}

impl IRInstruction {
    pub fn modify_argument<F>(&mut self, f: F)
    where
        F: FnOnce(usize) -> usize,
    {
        match self {
            Self::NOP => (),
            Self::Left(a)
            | Self::Right(a)
            | Self::Add(a)
            | Self::Sub(a)
            | Self::Input(a)
            | Self::Output(a)
            | Self::Open(a)
            | Self::Close(a) => *a = f(*a),
        }
    }

    fn is_degenerate(&self) -> bool {
        match self {
            Self::Left(a)
            | Self::Right(a)
            | Self::Add(a)
            | Self::Sub(a)
            | Self::Input(a)
            | Self::Output(a) => *a == 0,
            _ => false,
        }
    }

    pub fn combine(&self, other: &Self) -> Option<Self> {
        let new_instruction = match (self, other) {
            (Self::Left(a), Self::Left(b)) => Self::Left(a + b),
            (Self::Right(a), Self::Right(b)) => Self::Right(a + b),
            (Self::Left(l), Self::Right(r)) | (Self::Right(r), Self::Left(l)) => {
                if r > l {
                    Self::Right(r - l)
                } else {
                    Self::Left(l - r)
                }
            }

            (Self::Add(a), Self::Add(b)) => Self::Add(a + b),
            (Self::Sub(a), Self::Sub(b)) => Self::Sub(a + b),
            (Self::Add(a), Self::Sub(s)) | (Self::Sub(s), Self::Add(a)) => {
                if a > s {
                    Self::Add(a - s)
                } else {
                    Self::Sub(s - a)
                }
            }

            (Self::Input(a), Self::Input(b)) => Self::Input(a + b),
            (Self::Output(a), Self::Output(b)) => Self::Output(a + b),
            _ => return None,
        };

        Some(if new_instruction.is_degenerate() {
            Self::NOP
        } else {
            new_instruction
        })
    }

    #[allow(dead_code)]
    pub fn variant_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Left(_), Self::Left(_)) => true,
            (Self::Right(_), Self::Right(_)) => true,
            (Self::Add(_), Self::Add(_)) => true,
            (Self::Sub(_), Self::Sub(_)) => true,
            (Self::Input(_), Self::Input(_)) => true,
            (Self::Output(_), Self::Output(_)) => true,
            (Self::Open(_), Self::Open(_)) => true,
            (Self::Close(_), Self::Close(_)) => true,
            _ => false,
        }
    }

    pub fn argument(&self) -> usize {
        match self {
            Self::NOP => 0,
            Self::Left(a)
            | Self::Right(a)
            | Self::Add(a)
            | Self::Sub(a)
            | Self::Input(a)
            | Self::Output(a)
            | Self::Open(a)
            | Self::Close(a) => *a,
        }
    }

    pub fn is_nop(&self) -> bool {
        match self {
            Self::NOP => true,
            _ => false,
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Self::Right(_) => true,
            _ => false,
        }
    }

    pub fn is_add(&self) -> bool {
        match self {
            Self::Add(_) => true,
            _ => false,
        }
    }

    pub fn is_sub(&self) -> bool {
        match self {
            Self::Sub(_) => true,
            _ => false,
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::Open(_) => true,
            _ => false,
        }
    }

    pub fn is_close(&self) -> bool {
        match self {
            Self::Close(_) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for IRInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NOP => write!(f, "NOP"),
            Self::Left(a) => write!(f, "Left({})", a),
            Self::Right(a) => write!(f, "Right({})", a),
            Self::Add(a) => write!(f, "Add({})", a),
            Self::Sub(a) => write!(f, "Sub({})", a),
            Self::Input(a) => write!(f, "Input({})", a),
            Self::Output(a) => write!(f, "Output({})", a),
            Self::Open(a) => write!(f, "Open({})", a),
            Self::Close(a) => write!(f, "Close({})", a),
        }
    }
}

impl fmt::Display for IRInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NOP => write!(f, " "),
            Self::Left(a) => write!(f, "{}", "<".repeat(*a)),
            Self::Right(a) => write!(f, "{}", ">".repeat(*a)),
            Self::Add(a) => write!(f, "{}", "+".repeat(*a)),
            Self::Sub(a) => write!(f, "{}", "-".repeat(*a)),
            Self::Input(a) => write!(f, "{}", ",".repeat(*a)),
            Self::Output(a) => write!(f, "{}", ",".repeat(*a)),
            Self::Open(_) => write!(f, "["),
            Self::Close(_) => write!(f, "]"),
        }
    }
}

impl SomeFrom<char> for IRInstruction {
    fn some_from(c: char) -> Option<Self> {
        match c {
            '<' => Some(Self::Left(1)),
            '>' => Some(Self::Right(1)),
            '+' => Some(Self::Add(1)),
            '-' => Some(Self::Sub(1)),
            ',' => Some(Self::Input(1)),
            '.' => Some(Self::Output(1)),
            '[' => Some(Self::Open(1)),
            ']' => Some(Self::Close(1)),
            _ => None,
        }
    }
}

impl SomeFrom<&char> for IRInstruction {
    fn some_from(c: &char) -> Option<Self> {
        Self::some_from(*c)
    }
}

impl SomeFrom<u8> for IRInstruction {
    fn some_from(b: u8) -> Option<Self> {
        Self::some_from(b as char)
    }
}

impl SomeFrom<&u8> for IRInstruction {
    fn some_from(b: &u8) -> Option<Self> {
        Self::some_from(*b)
    }
}
