use crate::string_utils::LineChar;
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



#[derive(Error, Debug)]
enum PartFromIterError {
    #[error("not a brainfuck part")]
    NotAPart,
    #[error("consumed all of the iterator")]
    EndOfIterator,
}

impl Part {
    fn from_iter<I>(iter: &mut std::iter::Peekable<I>) -> Result<(usize, Self), PartFromIterError>
    where
        I: std::iter::Iterator<Item = (usize, char)>,
    {
        let (index, part_character) = iter.next().ok_or(PartFromIterError::EndOfIterator)?;
        let mut part = Part::try_from(part_character).map_err(|_| PartFromIterError::NotAPart)?;

        while let Some(character) = iter.peek().map(|(_, c)| c) {
            if let Ok(next_part) = Part::try_from(character) {
                part = match (&part, next_part) {
                    (Self::Shift(l), Self::Shift(r)) => Self::Shift(l + r),
                    (Self::Manipulate(l), Self::Manipulate(r)) => Self::Manipulate(l + r),
                    (Self::Input(l), Self::Input(r)) => Self::Input(l + r),
                    (Self::Output(l), Self::Output(r)) => Self::Output(l + r),
                    _ => return Ok((index, part)),
                }
            } else {
                return Ok((index, part));
            }

            iter.next();
        }

        return Ok((index, part));
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

    #[allow(dead_code)]
    pub fn is_shift(&self) -> bool {
        match self {
            Self::Shift(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_left(&self) -> bool {
        match self {
            Self::Shift(offset) => offset.is_negative(),
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_right(&self) -> bool {
        match self {
            Self::Shift(offset) => offset.is_positive(),
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_manipulate(&self) -> bool {
        match self {
            Self::Manipulate(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_less(&self) -> bool {
        match self {
            Self::Manipulate(ammount) => ammount.is_negative(),
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_more(&self) -> bool {
        match self {
            Self::Manipulate(ammount) => ammount.is_positive(),
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_in(&self) -> bool {
        match self {
            Self::Input(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

impl TryFrom<&char> for Part {
    type Error = PartTryFromError;

    fn try_from(character: &char) -> Result<Self, Self::Error> {
        Self::try_from(character.clone())
    }
}

impl TryFrom<u8> for Part {
    type Error = PartTryFromError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Self::try_from(byte as char)
    }
}

impl TryFrom<&u8> for Part {
    type Error = PartTryFromError;

    fn try_from(byte: &u8) -> Result<Self, Self::Error> {
        Self::try_from(byte.clone())
    }
}

pub struct Parts {
    parts_raw: Vec<Part>,
}

impl fmt::Display for Parts {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.parts_raw
                .iter()
                .fold(String::new(), |a, l| a + &l.to_string())
        )
    }
}

pub struct JumpAndLineChar {
    jump_position: usize,
    line_char: LineChar,
}

impl JumpAndLineChar {
    pub fn new(jump_position: usize, line_char: LineChar) -> Self {
        Self {
            jump_position: jump_position,
            line_char: line_char,
        }
    }
}

#[derive(Error, Debug)]
pub enum PartsParseError {
    #[error("unbalanced \"[\" at {}", .0)]
    MismatchedOpen(LineChar),
    #[error("unbalanced \"]\" at {}", .0)]
    MismatchedClose(LineChar),
}

impl str::FromStr for Parts {
    type Err = PartsParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut parts_raw = Vec::with_capacity(100);
        let mut brackets = Vec::with_capacity(20);

        for (line_i, line) in string.lines().enumerate() {
            let mut char_iter = line.char_indices().peekable();
            loop {
                match Part::from_iter(&mut char_iter) {
                    Ok((index, part)) => {
                        if part.is_open() {
                            brackets.push(JumpAndLineChar::new(
                                parts_raw.len(),
                                LineChar::new(line_i, index),
                            ));
                            parts_raw.push(part);
                        } else if part.is_close() {
                            let open = brackets
                                .pop()
                                .ok_or_else(|| {
                                    PartsParseError::MismatchedClose(LineChar::new(line_i, index))
                                })?
                                .jump_position;
                            parts_raw[open] = Part::Open(parts_raw.len() as i32);
                            parts_raw.push(Part::Close(open as i32));
                        } else if !part.is_degenerate() {
                            parts_raw.push(part);
                        }
                    }
                    Err(e) => match e {
                        PartFromIterError::NotAPart => (),
                        PartFromIterError::EndOfIterator => break,
                    },
                }
            }
        }

        if let Some(mismatched) = brackets.pop() {
            Err(PartsParseError::MismatchedOpen(mismatched.line_char))
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
