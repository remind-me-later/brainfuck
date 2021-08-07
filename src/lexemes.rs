use crate::executable::Executable;
use crate::position::Position;
use std::convert::TryFrom;
use std::fmt;
use std::str;
use thiserror::Error;

pub enum Lexeme {
    Shift(i32),
    Manipulate(i32),
    Input(i32),
    Output(i32),
    Open(i32),
    Close(i32),
}

impl Lexeme {
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

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Lexeme::Shift(offset) => {
                if offset.is_positive() {
                    ">"
                } else {
                    "<"
                }
            }
            Lexeme::Manipulate(ammount) => {
                if ammount.is_positive() {
                    "+"
                } else {
                    "-"
                }
            }
            Lexeme::Input(_) => ",",
            Lexeme::Output(_) => ".",
            Lexeme::Open(_) => "[",
            Lexeme::Close(_) => "]",
        };

        let res = match self {
            Lexeme::Shift(argument)
            | Lexeme::Manipulate(argument)
            | Lexeme::Input(argument)
            | Lexeme::Output(argument) => {
                if argument.abs() == 1 {
                    symbol.to_string()
                } else {
                    format!("{}{}", symbol, argument)
                }
            }
            Lexeme::Open(_) | Lexeme::Close(_) => symbol.to_string(),
        };

        write!(f, "{}", res)
    }
}

#[derive(Error, Debug)]
#[error("not a brainfuck lexeme")]
pub struct LexemeTryFromError;

impl TryFrom<char> for Lexeme {
    type Error = LexemeTryFromError;

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
            _ => Err(LexemeTryFromError),
        }
    }
}

impl TryFrom<u8> for Lexeme {
    type Error = LexemeTryFromError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Self::try_from(byte as char)
    }
}

pub struct Lexemes {
    lexemes_raw: Vec<Lexeme>,
}

impl fmt::Display for Lexemes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.lexemes_raw
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
pub enum LexemesParseError {
    #[error("unbalanced \"[\" at {}", .0)]
    MismatchedOpen(Position),
    #[error("unbalanced \"]\" at {}", .0)]
    MismatchedClose(Position),
}

impl str::FromStr for Lexemes {
    type Err = LexemesParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut lexemes_raw = Vec::with_capacity(100);
        let mut brackets = Vec::with_capacity(20);

        for (line_number, line) in string.lines().enumerate() {
            for (character_number, character) in line.chars().enumerate() {
                if let Ok(mut lexeme) = Lexeme::try_from(character) {
                    if let Some(last) = lexemes_raw.last() {
                        if let Some(collapsed) = lexeme.collapse(last) {
                            lexemes_raw.pop();
                            if !collapsed.is_degenerate() {
                                lexemes_raw.push(collapsed);
                            }
                        } else {
                            if lexeme.is_open() {
                                brackets.push(JumpAndPosition::new(
                                    lexemes_raw.len(),
                                    Position::new(line_number, character_number),
                                ));
                            } else if lexeme.is_close() {
                                let open = brackets
                                    .pop()
                                    .ok_or_else(|| {
                                        LexemesParseError::MismatchedClose(Position::new(
                                            line_number,
                                            character_number,
                                        ))
                                    })?
                                    .jump_position;
                                lexemes_raw[open] = Lexeme::Open(lexemes_raw.len() as i32);
                                lexeme = Lexeme::Close(open as i32);
                            }

                            lexemes_raw.push(lexeme);
                        }
                    } else {
                        if lexeme.is_open() {
                            brackets.push(JumpAndPosition::new(
                                0,
                                Position::new(line_number, character_number),
                            ));
                        } else if lexeme.is_close() {
                            return Err(LexemesParseError::MismatchedClose(Position::new(
                                line_number,
                                character_number,
                            )));
                        }

                        lexemes_raw.push(lexeme);
                    }
                }
            }
        }
        if let Some(mismatched) = brackets.pop() {
            Err(LexemesParseError::MismatchedOpen(
                mismatched.source_position,
            ))
        } else {
            Ok(Lexemes {
                lexemes_raw: lexemes_raw,
            })
        }
    }
}

impl Executable for Lexemes {
    fn execute<R, W>(&self, write: &mut W, read: &mut R)
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        // program counter
        let mut pc = 0;
        let mut head: usize = 0;
        let mut tape: [u8; 30_000] = [0; 30_000];
        let len = self.lexemes_raw.len();

        while pc < len {
            match self.lexemes_raw[pc] {
                Lexeme::Shift(offset) => head = (head as i32 + offset).rem_euclid(30_000) as usize,
                Lexeme::Manipulate(ammount) => {
                    tape[head] = tape[head].wrapping_add(ammount.wrapping_rem(256) as u8)
                }
                Lexeme::Input(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        read.read_exact(&mut buffer).unwrap();
                        tape[head] = buffer[0];
                    }
                }
                Lexeme::Output(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        buffer[0] = tape[head];
                        write.write(&mut buffer).unwrap();
                    }
                }
                Lexeme::Open(close) => {
                    if tape[head] == 0 {
                        pc = close as usize - 1;
                    }
                }
                Lexeme::Close(open) => {
                    if tape[head] != 0 {
                        pc = open as usize;
                    }
                }
            }

            pc += 1;
        }
    }
}
