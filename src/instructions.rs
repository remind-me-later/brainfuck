use crate::executable::Executable;
use std::convert::TryFrom;
use std::fmt;
use std::io::Read;
use std::str;
use thiserror::Error;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Instruction {
    Left,
    Right,
    Less,
    More,
    Input,
    Output,
    Open,
    Close,
}

impl fmt::Display for Instruction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", char::from(*self))
    }
}

impl From<Instruction> for char {
    fn from(instruction: Instruction) -> Self {
        match instruction {
            Instruction::Left => '<',
            Instruction::Right => '>',
            Instruction::Less => '-',
            Instruction::More => '+',
            Instruction::Input => ',',
            Instruction::Output => '.',
            Instruction::Open => '[',
            Instruction::Close => ']',
        }
    }
}

impl From<Instruction> for u8 {
    fn from(instruction: Instruction) -> Self {
        char::from(instruction) as u8
    }
}

#[derive(Error, Debug)]
#[error("not a brainfuck instruction {character:?}")]
pub struct InstructionTryFromError {
    character: char,
}

impl TryFrom<char> for Instruction {
    type Error = InstructionTryFromError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '<' => Ok(Instruction::Left),
            '>' => Ok(Instruction::Right),
            '-' => Ok(Instruction::Less),
            '+' => Ok(Instruction::More),
            ',' => Ok(Instruction::Input),
            '.' => Ok(Instruction::Output),
            '[' => Ok(Instruction::Open),
            ']' => Ok(Instruction::Close),
            _ => Err(InstructionTryFromError {
                character: character,
            }),
        }
    }
}

impl TryFrom<u8> for Instruction {
    type Error = InstructionTryFromError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Instruction::try_from(byte as char)
    }
}

pub struct Instructions {
    instructions_raw: Vec<Instruction>,
}

impl fmt::Display for Instructions {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.instructions_raw
                .iter()
                .fold(String::new(), |s, i| s + &i.to_string())
        )
    }
}

#[derive(Error, Debug)]
pub enum InstructionsParseError {
    #[error("unbalanced \"[\" at line: {line_number:?}, character: {character_number:?}")]
    MismatchedOpen {
        line_number: usize,
        character_number: usize,
    },
    #[error("unbalanced \"]\" at line: {line_number:?}, character: {character_number:?}")]
    MismatchedClose {
        line_number: usize,
        character_number: usize,
    },
}

impl str::FromStr for Instructions {
    type Err = InstructionsParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut instructions_raw = Vec::with_capacity(100);
        let mut brackets = Vec::with_capacity(20);

        for (line_number, line) in string.lines().enumerate() {
            for (character_number, character) in line.chars().enumerate() {
                if let Ok(instruction) = Instruction::try_from(character) {
                    if instruction == Instruction::Open {
                        brackets.push((line_number + 1, character_number + 1));
                    } else if instruction == Instruction::Close {
                        brackets
                            .pop()
                            .ok_or_else(|| InstructionsParseError::MismatchedClose {
                                line_number: line_number + 1,
                                character_number: character_number + 1,
                            })?;
                    }

                    instructions_raw.push(instruction);
                }
            }
        }

        if let Some(mismatched_left) = brackets.pop() {
            Err(InstructionsParseError::MismatchedOpen {
                line_number: mismatched_left.0,
                character_number: mismatched_left.1,
            })
        } else {
            Ok(Instructions {
                instructions_raw: instructions_raw,
            })
        }
    }
}

impl Executable for Instructions {
    fn execute<R, W>(&self, write: &mut W, read: &mut R)
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        // program counter
        let mut pc = 0;
        let mut head: usize = 0;
        let mut tape: [u8; 30_000] = [0; 30_000];
        let len = self.instructions_raw.len();

        while pc < len {
            match self.instructions_raw[pc] {
                Instruction::Left => head = if head == 0 { 29_999 } else { head - 1 },
                Instruction::Right => head = (head + 1).wrapping_rem(30_000),
                Instruction::Less => tape[head] = tape[head].wrapping_sub(1),
                Instruction::More => tape[head] = tape[head].wrapping_add(1),
                Instruction::Input => {
                    let mut buffer = [0; 1];
                    read.read_exact(&mut buffer).unwrap();
                    tape[head] = buffer[0];
                }
                Instruction::Output => {
                    let mut buffer = [0; 1];
                    buffer[0] = tape[head];
                    write.write(&mut buffer).unwrap();
                }
                Instruction::Open => {
                    if tape[head] == 0 {
                        let mut depth = 0;
                        pc += 1;

                        while depth != 0 || self.instructions_raw[pc] != Instruction::Close {
                            if self.instructions_raw[pc] == Instruction::Open {
                                depth += 1;
                            } else if self.instructions_raw[pc] == Instruction::Close {
                                depth -= 1;
                            }
                            pc += 1;
                        }
                        pc -= 1;
                    }
                }
                Instruction::Close => {
                    if tape[head] != 0 {
                        let mut depth = 0;
                        pc -= 1;

                        while depth != 0 || self.instructions_raw[pc] != Instruction::Open {
                            if self.instructions_raw[pc] == Instruction::Open {
                                depth += 1;
                            } else if self.instructions_raw[pc] == Instruction::Close {
                                depth -= 1;
                            }
                            pc -= 1;
                        }
                    }
                }
            }

            pc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_hello() {
        let program = "[ A simple \"Hello, World\" program that prints a newline at the end, only the first cell is manipulated to obtain the desired ASCII values. A loop at the beginning of a program will never be executed as the value of the first cell is 0, so you can write a comment using any character you like as long as the '[' and ']' are balanced.]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.-------------------------------------------------------------------.------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.------.--------.-------------------------------------------------------------------.";

        let mut file = Vec::new();

        program
            .parse::<Instructions>()
            .unwrap()
            .execute(&mut file, &mut std::io::empty());

        let mut out = Vec::new();
        file.as_slice().read_to_end(&mut out).unwrap();

        println!("{}", String::from_utf8(out).unwrap());
    }
}
