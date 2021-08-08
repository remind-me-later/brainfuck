use crate::executable::Executable;
use crate::position::Position;
use std::convert::TryFrom;
use std::fmt;
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

impl Instruction {
    pub fn is_open(self) -> bool {
        self == Instruction::Open
    }

    pub fn is_close(self) -> bool {
        self == Instruction::Close
    }
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
#[error("not a brainfuck instruction")]
pub struct InstructionTryFromError;

impl TryFrom<char> for Instruction {
    type Error = InstructionTryFromError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '-' => Ok(Self::Less),
            '+' => Ok(Self::More),
            ',' => Ok(Self::Input),
            '.' => Ok(Self::Output),
            '[' => Ok(Self::Open),
            ']' => Ok(Self::Close),
            _ => Err(InstructionTryFromError),
        }
    }
}

impl TryFrom<u8> for Instruction {
    type Error = InstructionTryFromError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Self::try_from(byte as char)
    }
}

pub struct Instructions {
    instructions_raw: Vec<Instruction>,
}

impl Instructions {
    fn new(instructions_raw: Vec<Instruction>) -> Self {
        Self {
            instructions_raw: instructions_raw,
        }
    }
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
    #[error("unbalanced \"[\" at {}", .0)]
    MismatchedOpen(Position),
    #[error("unbalanced \"]\" at {}", .0)]
    MismatchedClose(Position),
}

impl str::FromStr for Instructions {
    type Err = InstructionsParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut instructions_raw = Vec::with_capacity(100);
        let mut brackets = Vec::with_capacity(20);

        for (line_number, line) in string.lines().enumerate() {
            for (character_number, character) in line.chars().enumerate() {
                if let Ok(instruction) = Instruction::try_from(character) {
                    if instruction.is_open() {
                        brackets.push(Position::new(line_number, character_number));
                    } else if instruction.is_close() {
                        brackets.pop().ok_or_else(|| {
                            InstructionsParseError::MismatchedClose(Position::new(
                                line_number,
                                character_number,
                            ))
                        })?;
                    }

                    instructions_raw.push(instruction);
                }
            }
        }

        if let Some(mismatched_left) = brackets.pop() {
            Err(InstructionsParseError::MismatchedOpen(mismatched_left))
        } else {
            Ok(Instructions::new(instructions_raw))
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
                Instruction::Right => head = if head == 29_999 { 0 } else { head + 1 },
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

                        while depth != 0 || !self.instructions_raw[pc].is_close() {
                            if self.instructions_raw[pc].is_open() {
                                depth += 1;
                            } else if self.instructions_raw[pc].is_close() {
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

                        while depth != 0 || !self.instructions_raw[pc].is_open() {
                            if self.instructions_raw[pc].is_open() {
                                depth += 1;
                            } else if self.instructions_raw[pc].is_close() {
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
    use std::io::Read;

    #[test]
    fn simple_hello() {
        // Super simple hello world program should show any glaring errors
        let program = "
[
  A simple \"Hello, World\" program that prints a newline at the end,
  only the first cell is manipulated to obtain the desired ASCII values.

  A loop at the beginning of a program will never be executed as the value
  of the first cell is 0, so you can write a comment using any character you
  like as long as the '[' and ']' are balanced.
]

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.   Add 72 which is ASCII for 'H' to Cell #0 and print it
+++++++++++++++++++++++++++++.                                              Add 30 to get to the value 101 for 'e'
+++++++.                                                                    Add 7 for 'l'
.                                                                           Print for another 'l'
+++.                                                                        Add 3 for 'o'
-------------------------------------------------------------------.        Subtract until 44 for comma
------------.                                                               The same to get to 32 for Space
+++++++++++++++++++++++++++++++++++++++++++++++++++++++.                    Get to 87 for 'W'
++++++++++++++++++++++++.                                                   111 for 'o'
+++.                                                                        114 for 'r'
------.                                                                     108 for 'l'
--------.                                                                   100 for 'd'
-------------------------------------------------------------------.        10
for '!'";

        let mut output_file = Vec::with_capacity(13);

        program
            .parse::<Instructions>()
            .unwrap()
            .execute(&mut output_file, &mut std::io::empty());

        let mut out = Vec::with_capacity(13);
        output_file.as_slice().read_to_end(&mut out).unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "Hello, World!");
    }

    #[test]
    fn number() {
        // This program uses a lot of nested loops so it should be a good test
        let program = "
    >>>>+>+++>+++>>>>>+++[
      >,+>++++[>++++<-]>[<<[-[->]]>[<]>-]<<[
        >+>+>>+>+[<<<<]<+>>[+<]<[>]>+[[>>>]>>+[<<<<]>-]+<+>>>-[
          <<+[>]>>+<<<+<+<--------[
            <<-<<+[>]>+<<-<<-[
              <<<+<-[>>]<-<-<<<-<----[
                <<<->>>>+<-[
                  <<<+[>]>+<<+<-<-[
                    <<+<-<+[>>]<+<<<<+<-[
                      <<-[>]>>-<<<-<-<-[
                        <<<+<-[>>]<+<<<+<+<-[
                          <<<<+[>]<-<<-[
                            <<+[>]>>-<<<<-<-[
                              >>>>>+<-<<<+<-[
                                >>+<<-[
                                  <<-<-[>]>+<<-<-<-[
                                    <<+<+[>]<+<+<-[
                                      >>-<-<-[
                                        <<-[>]<+<++++[<-------->-]++<[
                                          <<+[>]>>-<-<<<<-[
                                            <<-<<->>>>-[
                                              <<<<+[>]>+<<<<-[
                                                <<+<<-[>>]<+<<<<<-[
                                                  >>>>-<<<-<-
      ]]]]]]]]]]]]]]]]]]]]]]>[>[[[<<<<]>+>>[>>>>>]<-]<]>>>+>>>>>>>+>]<
    ]<[-]<<<<<<<++<+++<+++[
      [>]>>>>>>++++++++[<<++++>++++++>-]<-<<[-[<+>>.<-]]<<<<[
        -[-[>+<-]>]>>>>>[.[>]]<<[<+>-]>>>[<<++[<+>--]>>-]
        <<[->+<[<++>-]]<<<[<+>-]<<<<
      ]>>+>>>--[<+>---]<.>>[[-]<<]<
    ]
    [Enter a number using ()-./0123456789abcdef and space, and hit return.
    Daniel B Cristofani (cristofdathevanetdotcom)
    http://www.hevanet.com/cristofd/brainfuck/]
    ";

        let mut output_file = Vec::with_capacity(300);
        let input_file = [
            '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8,
            '9' as u8, '\n' as u8,
        ];

        program
            .parse::<Instructions>()
            .unwrap()
            .execute(&mut output_file, &mut &input_file[..]);

        let mut out = Vec::with_capacity(300);
        output_file.as_slice().read_to_end(&mut out).unwrap();

        assert_eq!(
            String::from_utf8(out).unwrap(),
            "                /\\
                \\/\\
              /\\   
              \\/\\
            /\\ \\/
              \\
          /    
          \\/\\
        /  \\/
        \\/\\
       \\  /
      \\/\\
    /\\   
     /\\
  /\\  /
   / 
 \\ \\/
  \\
   
"
        );
    }
}
