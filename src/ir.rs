use std::convert;
use std::fmt;
use std::ops;

use itertools::Itertools;

#[derive(Clone)]
pub enum Instruction {
    NOP,
    Left(usize),
    Right(usize),
    Add(usize),
    Sub(usize),
    Input(usize),
    Output(usize),
    Open(usize),
    Close(usize),
    Zero,
}

impl Instruction {
    pub fn modify_argument<F>(&mut self, f: F)
    where
        F: FnOnce(usize) -> usize,
    {
        match self {
            Self::NOP | Self::Zero => (),
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
            (Self::Zero, Self::Zero) => Self::Zero,
            _ => return None,
        };

        Some(if new_instruction.is_degenerate() {
            Self::NOP
        } else {
            new_instruction
        })
    }

    pub fn argument(&self) -> usize {
        match self {
            Self::NOP | Self::Zero => 0,
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

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Zero => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Instruction {
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
            Self::Zero => write!(f, "Zero"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NOP => write!(f, " "),
            Self::Left(a) => write!(f, "{}{}", "<", a),
            Self::Right(a) => write!(f, "{}{}", ">", a),
            Self::Add(a) => write!(f, "{}{}", "+", a),
            Self::Sub(a) => write!(f, "{}{}", "-", a),
            Self::Input(a) => write!(f, "{}{}", ",", a),
            Self::Output(a) => write!(f, "{}{}", ",", a),
            Self::Open(_) => write!(f, "["),
            Self::Close(_) => write!(f, "]"),
            Self::Zero => write!(f, "!"),
        }
    }
}

impl convert::TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Left(1)),
            '>' => Ok(Self::Right(1)),
            '+' => Ok(Self::Add(1)),
            '-' => Ok(Self::Sub(1)),
            ',' => Ok(Self::Input(1)),
            '.' => Ok(Self::Output(1)),
            '[' => Ok(Self::Open(1)),
            ']' => Ok(Self::Close(1)),
            _ => Err(()),
        }
    }
}

impl convert::TryFrom<&char> for Instruction {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Self::try_from(*c)
    }
}

impl convert::TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        Self::try_from(b as char)
    }
}

impl convert::TryFrom<&u8> for Instruction {
    type Error = ();

    fn try_from(b: &u8) -> Result<Self, Self::Error> {
        Self::try_from(*b)
    }
}

impl convert::TryFrom<&[Instruction]> for Instruction {
    type Error = ();

    fn try_from(s: &[Instruction]) -> Result<Self, Self::Error> {
        if s.len() == 1 {
            // zero
            if s[0].is_sub() && s[0].argument() == 1 {
                return Ok(Instruction::Zero);
            }
        }

        Err(())
    }
}

pub struct IR {
    ir: Vec<Instruction>,
}

impl Default for IR {
    fn default() -> Self {
        Self {
            ir: Vec::with_capacity(100),
        }
    }
}

impl ops::Index<usize> for IR {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ir[index]
    }
}

impl ops::IndexMut<usize> for IR {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ir[index]
    }
}

impl fmt::Debug for IR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.ir.iter().map(|i| format!("{:?}", i)).join("\n")
        )
    }
}

impl fmt::Display for IR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ir.iter().join(""))
    }
}

impl IR {
    pub fn len(&self) -> usize {
        self.ir.len()
    }

    pub fn push(&mut self, value: Instruction) {
        self.ir.push(value)
    }

    pub fn vec(&self) -> &Vec<Instruction> {
        &self.ir
    }

    pub fn mut_vec(&mut self) -> &mut Vec<Instruction> {
        &mut self.ir
    }
}
