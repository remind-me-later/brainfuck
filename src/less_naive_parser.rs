use crate::position::Position;
use crate::virtual_machine::{VMRegisters, VMRunnable};
use std::convert::TryFrom;
use std::fmt;
use std::str;
use thiserror::Error;

pub enum Part {
    Shift(i32),
    Manipulate(i32),
    Input(i32),
    Output(i32),
    Open(i32),
    Close(i32),
}

impl Part {
    pub fn collapse(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Shift(mine), Self::Shift(theirs)) => Some(Self::Shift(mine + theirs)),
            (Self::Manipulate(mine), Self::Manipulate(theirs)) => {
                Some(Self::Manipulate(mine + theirs))
            }
            (Self::Input(mine), Self::Input(theirs)) => Some(Self::Input(mine + theirs)),
            (Self::Output(mine), Self::Output(theirs)) => Some(Self::Output(mine + theirs)),
            _ => None,
        }
    }

    pub fn is_degenerate(&self) -> bool {
        let value = match self {
            Self::Shift(offset) => offset,
            Self::Manipulate(ammount) => ammount,
            Self::Input(times) => times,
            Self::Output(times) => times,
            Self::Open(offset) => offset,
            Self::Close(offset) => offset,
        };

        *value == 0
    }

    pub fn is_shift(&self) -> bool {
        match self {
            Self::Shift(_) => true,
            _ => false,
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Self::Shift(offset) => offset.is_negative(),
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Self::Shift(offset) => offset.is_positive(),
            _ => false,
        }
    }

    pub fn is_manipulate(&self) -> bool {
        match self {
            Self::Manipulate(_) => true,
            _ => false,
        }
    }

    pub fn is_less(&self) -> bool {
        match self {
            Self::Manipulate(ammount) => ammount.is_negative(),
            _ => false,
        }
    }

    pub fn is_more(&self) -> bool {
        match self {
            Self::Manipulate(ammount) => ammount.is_positive(),
            _ => false,
        }
    }

    pub fn is_in(&self) -> bool {
        match self {
            Self::Input(_) => true,
            _ => false,
        }
    }

    pub fn is_out(&self) -> bool {
        match self {
            Self::Output(_) => true,
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

    pub fn argument(&self) -> i32 {
        *match self {
            Self::Shift(offset) => offset,
            Self::Manipulate(ammount) => ammount,
            Self::Input(times) => times,
            Self::Output(times) => times,
            Self::Open(offset) => offset,
            Self::Close(offset) => offset,
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Self::Shift(offset) => {
                if offset.is_positive() {
                    ">"
                } else {
                    "<"
                }
            }
            Self::Manipulate(ammount) => {
                if ammount.is_positive() {
                    "+"
                } else {
                    "-"
                }
            }
            Self::Input(_) => ",",
            Self::Output(_) => ".",
            Self::Open(_) => "[",
            Self::Close(_) => "]",
        };

        let res = match self {
            Self::Shift(argument)
            | Self::Manipulate(argument)
            | Self::Input(argument)
            | Self::Output(argument) => {
                if argument.abs() == 1 {
                    symbol.to_string()
                } else {
                    format!("{}{}", symbol, argument)
                }
            }
            Self::Open(_) | Self::Close(_) => symbol.to_string(),
        };

        write!(f, "{}", res)
    }
}

#[derive(Error, Debug)]
#[error("not a brainfuck part")]
pub struct PartTryFromError;

impl TryFrom<char> for Part {
    type Error = PartTryFromError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '<' => Ok(Self::Shift(-1)),
            '>' => Ok(Self::Shift(1)),
            '-' => Ok(Self::Manipulate(-1)),
            '+' => Ok(Self::Manipulate(1)),
            ',' => Ok(Self::Input(1)),
            '.' => Ok(Self::Output(1)),
            '[' => Ok(Self::Open(1)),
            ']' => Ok(Self::Close(1)),
            _ => Err(PartTryFromError),
        }
    }
}

impl TryFrom<u8> for Part {
    type Error = PartTryFromError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Self::try_from(byte as char)
    }
}

pub struct Parts {
    parts_raw: Vec<Part>,
}

impl fmt::Display for Parts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.parts_raw
                .iter()
                .fold(String::new(), |a, l| a + &l.to_string())
        )
    }
}

pub struct JumpAndPosition {
    jump_position: usize,
    source_position: Position,
}

impl JumpAndPosition {
    pub fn new(jump_position: usize, source_position: Position) -> Self {
        Self {
            jump_position: jump_position,
            source_position: source_position,
        }
    }
}

#[derive(Error, Debug)]
pub enum PartsParseError {
    #[error("unbalanced \"[\" at {}", .0)]
    MismatchedOpen(Position),
    #[error("unbalanced \"]\" at {}", .0)]
    MismatchedClose(Position),
}

impl str::FromStr for Parts {
    type Err = PartsParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut parts_raw = Vec::with_capacity(100);
        let mut brackets = Vec::with_capacity(20);

        for (line_number, line) in string.lines().enumerate() {
            for (character_number, character) in line.chars().enumerate() {
                if let Ok(mut lexeme) = Part::try_from(character) {
                    if let Some(last) = parts_raw.last() {
                        if let Some(collapsed) = lexeme.collapse(last) {
                            parts_raw.pop();
                            if !collapsed.is_degenerate() {
                                parts_raw.push(collapsed);
                            }
                        } else {
                            if lexeme.is_open() {
                                brackets.push(JumpAndPosition::new(
                                    parts_raw.len(),
                                    Position::new(line_number, character_number),
                                ));
                            } else if lexeme.is_close() {
                                let open = brackets
                                    .pop()
                                    .ok_or_else(|| {
                                        PartsParseError::MismatchedClose(Position::new(
                                            line_number,
                                            character_number,
                                        ))
                                    })?
                                    .jump_position;
                                parts_raw[open] = Part::Open(parts_raw.len() as i32);
                                lexeme = Part::Close(open as i32);
                            }

                            parts_raw.push(lexeme);
                        }
                    } else {
                        if lexeme.is_open() {
                            brackets.push(JumpAndPosition::new(
                                0,
                                Position::new(line_number, character_number),
                            ));
                        } else if lexeme.is_close() {
                            return Err(PartsParseError::MismatchedClose(Position::new(
                                line_number,
                                character_number,
                            )));
                        }

                        parts_raw.push(lexeme);
                    }
                }
            }
        }
        if let Some(mismatched) = brackets.pop() {
            Err(PartsParseError::MismatchedOpen(mismatched.source_position))
        } else {
            Ok(Parts {
                parts_raw: parts_raw,
            })
        }
    }
}

impl VMRunnable for Parts {
    fn run<R, W>(&self, registers: &mut VMRegisters, writer: &mut W, reader: &mut R)
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        let len = self.parts_raw.len();

        while registers.pc() < len {
            match self.parts_raw[registers.pc()] {
                Part::Shift(offset) => registers
                    .head_to((registers.head() as i32 + offset).rem_euclid(30_000) as usize),
                Part::Manipulate(ammount) => {
                    *registers.cell_mut() = registers
                        .cell()
                        .wrapping_add(ammount.wrapping_rem(256) as u8)
                }
                Part::Input(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        reader.read_exact(&mut buffer).unwrap();
                        *registers.cell_mut() = buffer[0];
                    }
                }
                Part::Output(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        buffer[0] = registers.cell();
                        writer.write(&mut buffer).unwrap();
                    }
                }
                Part::Open(close) => {
                    if registers.cell() == 0 {
                        registers.jump_to(close as usize - 1);
                    }
                }
                Part::Close(open) => {
                    if registers.cell() != 0 {
                        registers.jump_to(open as usize);
                    }
                }
            }

            registers.increase_pc();
        }
    }
}
