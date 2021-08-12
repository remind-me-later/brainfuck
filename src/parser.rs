use std::fmt;
use std::iter;
use std::str;

use crate::ir_instruction::IRInstruction;
use crate::parser_error::ParserError;
use crate::parser_warning::ParserWarning;
use crate::some_from::SomeFrom;

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

impl<I> SomeFrom<&mut iter::Peekable<I>> for IRInstructionWithIndex
where
    I: iter::Iterator<Item = (usize, char)>,
{
    fn some_from(iter: &mut iter::Peekable<I>) -> Option<Self> {
        while let Some((beginning, c)) = iter.next() {
            if let Some(mut instruction) = IRInstruction::some_from(c) {
                let mut end = beginning.clone();

                while let Some(next_instruction) = iter
                    .peek()
                    .map(|(_, c)| IRInstruction::some_from(c))
                    .flatten()
                    .map(|e| instruction.combine(&e))
                    .flatten()
                {
                    instruction = next_instruction;
                    end = iter.next().unwrap().0;
                }

                return Some(Self::new(instruction, beginning, end));
            }
        }

        return None;
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
        let mut indexs = string.char_indices().peekable();

        while let Some(instruction_with_context) = IRInstructionWithIndex::some_from(&mut indexs) {
            let mut instruction = instruction_with_context.instruction;
            let beginning = instruction_with_context.beginning;
            let end = instruction_with_context.end;

            if instruction.is_open() {
                brackets.push(JumpAndCharPosition::new(self.program.len(), beginning));
            } else if instruction.is_close() {
                let open = brackets
                    .pop()
                    .ok_or_else(|| ParserError::MismatchedClose(beginning))?
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
            Err(ParserError::MismatchedOpen(mismatched.index))
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
        let mut result = String::new();

        for instruction in &self.program {
            result = format!("{}{:?}\n", result, instruction)
        }

        write!(f, "{}", result)
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.program
                .iter()
                .fold(String::new(), |a, l| a + &l.to_string())
        )
    }
}

struct JumpAndCharPosition {
    jump_index: usize,
    index: usize,
}

impl JumpAndCharPosition {
    pub fn new(jump_index: usize, index: usize) -> Self {
        Self {
            jump_index: jump_index,
            index: index,
        }
    }
}
