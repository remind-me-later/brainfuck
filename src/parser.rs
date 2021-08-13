use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::str;

use itertools::Itertools;

use crate::ir_instruction::IRInstruction;
use crate::parser_error::ParserError;
use crate::parser_warning::ParserWarning;

struct IRInstructionWithIndex {
    instruction: IRInstruction,
    beginning: usize,
    end: usize,
}

impl IRInstructionWithIndex {
    fn new(instruction: IRInstruction, beginning: usize, end: usize) -> Self {
        Self {
            instruction: instruction,
            beginning: beginning,
            end: end,
        }
    }
}

impl<I> TryFrom<&mut iter::Peekable<I>> for IRInstructionWithIndex
where
    I: iter::Iterator<Item = (usize, char)>,
{
    type Error = ();

    fn try_from(iter: &mut iter::Peekable<I>) -> Result<Self, Self::Error> {
        while let Some((beginning, c)) = iter.next() {
            if let Ok(mut instruction) = IRInstruction::try_from(c) {
                let mut end = beginning.clone();

                while let Some(next_instruction) = iter
                    .peek()
                    .map(|(_, c)| IRInstruction::try_from(c).ok())
                    .flatten()
                    .map(|e| instruction.combine(&e))
                    .flatten()
                {
                    instruction = next_instruction;
                    end = iter.next().unwrap().0;
                }

                return Ok(Self::new(instruction, beginning, end));
            }
        }

        return Err(());
    }
}

pub struct Parser {
    program: Vec<IRInstruction>,
    warnings: Vec<ParserWarning>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            program: Vec::with_capacity(100),
            warnings: Vec::with_capacity(10),
        }
    }
}

impl Parser {
    pub fn parse(&mut self, string: &str) -> Result<(), ParserError> {
        let mut brackets = Vec::with_capacity(20);
        let mut indices = string.char_indices().peekable();

        while let Ok(instruction_with_context) = IRInstructionWithIndex::try_from(&mut indices) {
            let mut instruction = instruction_with_context.instruction;
            let beginning = instruction_with_context.beginning;
            let end = instruction_with_context.end;

            if instruction.is_open() {
                brackets.push(JumpIndex::new(
                    instruction.clone(),
                    self.program.len(),
                    beginning,
                ));
            } else if instruction.is_close() {
                let open = brackets
                    .pop()
                    .ok_or_else(|| {
                        ParserError::MismatchedBracket(beginning, instruction.to_string())
                    })?
                    .jump_index;

                let close = self.program.len();

                self.program[open].modify_argument(|_| close);
                instruction.modify_argument(|_| open);
            } else if instruction.is_left() || instruction.is_right() {
                instruction.modify_argument(|a| a % 30_000);
            } else if instruction.is_add() || instruction.is_sub() {
                instruction.modify_argument(|a| a % 256);
            } else if instruction.is_nop() {
                self.warnings.push(ParserWarning::NOP(
                    beginning,
                    end,
                    String::from(&string[beginning..=end]),
                ));
            }

            self.program.push(instruction);
        }

        if let Some(mismatched) = brackets.pop() {
            Err(ParserError::MismatchedBracket(
                mismatched.index,
                mismatched.instruction.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn warnings(&self) -> &Vec<ParserWarning> {
        &self.warnings
    }

    pub fn ir_program(&self) -> &Vec<IRInstruction> {
        &self.program
    }
}

impl fmt::Debug for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.program.iter().map(|i| format!("{:?}", i)).join("\n")
        )
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.program.iter().join(""))
    }
}

struct JumpIndex {
    instruction: IRInstruction,
    jump_index: usize,
    index: usize,
}

impl JumpIndex {
    pub fn new(instruction: IRInstruction, jump_index: usize, index: usize) -> Self {
        Self {
            instruction: instruction,
            jump_index: jump_index,
            index: index,
        }
    }
}
