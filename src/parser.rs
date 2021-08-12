use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::str;

use thiserror::Error;

use crate::ir_instruction::IRInstruction;
use crate::parser_error::ParserError;
use crate::parser_warning::ParserWarning;
use crate::some_from::SomeFrom;
use crate::string_utils::{CharPosition, CharPositionEnumerate};

#[derive(Debug)]
struct IRInstructionWithContext {
    expression: IRInstruction,
    beginning: CharPosition,
    end: CharPosition,
}

#[derive(Error, Debug)]
enum IRInstructionWithContextFromIterError {
    #[error("not a brainfuck expression")]
    NotAnIRInstruction,
    #[error("the expression beginning at {} and ending at {} 
            evaluates to the null operation", .0.beginning, .0.end)]
    DegenerateIRInstruction(IRInstructionWithContext),
    #[error("consumed all of the iterator")]
    EndOfIterator,
}

impl IRInstructionWithContext {
    fn new(expression: IRInstruction, beginning: CharPosition, end: CharPosition) -> Self {
        Self {
            expression: expression,
            beginning: beginning,
            end: end,
        }
    }
}

impl<I> TryFrom<&mut iter::Peekable<I>> for IRInstructionWithContext
where
    I: iter::Iterator<Item = (CharPosition, char)>,
{
    type Error = IRInstructionWithContextFromIterError;

    fn try_from(iter: &mut iter::Peekable<I>) -> Result<Self, Self::Error> {
        let (beginning, first_character) = iter.next().ok_or(Self::Error::EndOfIterator)?;

        let mut expression =
            IRInstruction::some_from(first_character).ok_or(Self::Error::NotAnIRInstruction)?;
        let mut end = beginning.clone();

        while let Some(next_expression) = iter
            .peek()
            .map(|(_, c)| IRInstruction::some_from(c))
            .flatten()
            .map(|e| expression.combine(&e))
            .flatten()
        {
            end = iter.next().unwrap().0;

            if next_expression.is_degenerate() {
                return Err(Self::Error::DegenerateIRInstruction(Self::new(
                    next_expression,
                    beginning,
                    end,
                )));
            }

            expression = next_expression;
        }

        return Ok(Self::new(expression, beginning, end));
    }
}

pub struct Parser {
    expressions_raw: Vec<IRInstruction>,
    warnings: Vec<ParserWarning>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            expressions_raw: Vec::with_capacity(100),
            warnings: Vec::with_capacity(10),
        }
    }
}
impl Parser {
    pub fn parse(&mut self, string: &str) -> Result<(), ParserError> {
        let mut brackets = Vec::with_capacity(20);
        let mut positions = CharPositionEnumerate::from(string).peekable();

        loop {
            match IRInstructionWithContext::try_from(&mut positions) {
                Ok(expression_with_context) => {
                    let mut expression = expression_with_context.expression;
                    let position = expression_with_context.beginning;

                    if expression.is_open() {
                        brackets.push(JumpAndCharPosition::new(
                            self.expressions_raw.len(),
                            position,
                        ));
                    } else if expression.is_close() {
                        let open = brackets
                            .pop()
                            .ok_or_else(|| ParserError::MismatchedClose(position))?
                            .jump_to;

                        let close = self.expressions_raw.len();

                        self.expressions_raw[open].modify_argument(|_| close);
                        expression.modify_argument(|_| open);
                    } else if expression.is_left() || expression.is_right() {
                        expression.modify_argument(|a| a % 30_000);
                    } else if expression.is_add() || expression.is_sub() {
                        expression.modify_argument(|a| a % 256);
                    }

                    self.expressions_raw.push(expression);
                }

                Err(e) => match e {
                    IRInstructionWithContextFromIterError::NotAnIRInstruction => {}
                    IRInstructionWithContextFromIterError::DegenerateIRInstruction(e) => {
                        self.warnings
                            .push(ParserWarning::UselessExpression(e.beginning, e.end));
                    }
                    IRInstructionWithContextFromIterError::EndOfIterator => break,
                },
            }
        }

        if let Some(mismatched) = brackets.pop() {
            Err(ParserError::MismatchedOpen(mismatched.position))
        } else {
            Ok(())
        }
    }

    pub fn warnings(&self) -> &Vec<ParserWarning> {
        &self.warnings
    }
}

impl fmt::Debug for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for expression in &self.expressions_raw {
            result = format!("{}{:?}\n", result, expression)
        }

        write!(f, "{}", result)
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.expressions_raw
                .iter()
                .fold(String::new(), |a, l| a + &l.to_string())
        )
    }
}

struct JumpAndCharPosition {
    jump_to: usize,
    position: CharPosition,
}

impl JumpAndCharPosition {
    pub fn new(jump_to: usize, position: CharPosition) -> Self {
        Self {
            jump_to: jump_to,
            position: position,
        }
    }
}

