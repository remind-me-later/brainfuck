mod fatal;
mod warning;

use crate::ir::{Instruction, IR};
use fatal::Fatal;
use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::str;

struct InstructionWithIndex {
    instruction: Instruction,
    beginning: usize,
    end: usize,
}

impl InstructionWithIndex {
    fn new(instruction: Instruction, beginning: usize, end: usize) -> Self {
        Self {
            instruction: instruction,
            beginning: beginning,
            end: end,
        }
    }
}

impl<I> TryFrom<&mut iter::Peekable<I>> for InstructionWithIndex
where
    I: iter::Iterator<Item = (usize, char)>,
{
    type Error = ();

    fn try_from(iter: &mut iter::Peekable<I>) -> Result<Self, Self::Error> {
        while let Some((beginning, c)) = iter.next() {
            if let Ok(mut instruction) = Instruction::try_from(c) {
                let mut end = beginning.clone();

                while let Some(next_instruction) = iter
                    .peek()
                    .map(|(_, c)| Instruction::try_from(c).ok())
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
    ir: IR,
    warnings: Vec<warning::Warning>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            ir: IR::default(),
            warnings: Vec::with_capacity(10),
        }
    }
}

pub type ParseResult = Result<(), Fatal>;

impl Parser {
    pub fn parse(&mut self, string: &str) -> ParseResult {
        let mut brackets = Vec::with_capacity(20);
        let mut indices = string.char_indices().peekable();

        while let Ok(instruction_with_context) = InstructionWithIndex::try_from(&mut indices) {
            let mut instruction = instruction_with_context.instruction;
            let beginning = instruction_with_context.beginning;
            let end = instruction_with_context.end;
            let instruction_index = self.ir.len();

            if instruction.is_open() {
                brackets.push(JumpIndex::new(
                    instruction.clone(),
                    instruction_index,
                    beginning,
                ));
            } else if instruction.is_close() {
                let open = brackets
                    .pop()
                    .ok_or_else(|| Fatal::MismatchedBracket(beginning, instruction.to_string()))?
                    .jump_index;

                if let Ok(meta_instruction) = Instruction::try_from(&self.ir.vec()[open + 1..]) {
                    self.ir.mut_vec().drain(open..);
                    instruction = meta_instruction;
                } else {
                    // normal loop
                    self.ir[open].modify_argument(|_| instruction_index);
                    instruction.modify_argument(|_| open);
                }
            } else if instruction.is_nop() {
                self.warnings.push(warning::Warning::NOP(
                    beginning,
                    end,
                    String::from(&string[beginning..=end]),
                ));
            }

            self.ir.push(instruction);
        }

        if let Some(mismatched) = brackets.pop() {
            Err(Fatal::MismatchedBracket(
                mismatched.index,
                mismatched.instruction.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn warnings(&self) -> &Vec<warning::Warning> {
        &self.warnings
    }

    pub fn ir(&self) -> &IR {
        &self.ir
    }
}

impl fmt::Debug for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.ir)
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ir)
    }
}

struct JumpIndex {
    instruction: Instruction,
    jump_index: usize,
    index: usize,
}

impl JumpIndex {
    pub fn new(instruction: Instruction, jump_index: usize, index: usize) -> Self {
        Self {
            instruction: instruction,
            jump_index: jump_index,
            index: index,
        }
    }
}
