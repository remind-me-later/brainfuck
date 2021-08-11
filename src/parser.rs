use crate::expression::Expression;
use crate::string_utils::{CharPosition, CharPositionEnumerate};
use crate::virtual_machine::{VMRegisters, VMRunnable};
use std::convert::TryFrom;
use std::fmt;
use std::str;
use thiserror::Error;

#[derive(Debug)]
pub struct ExpressionWithContext {
    expression: Expression,
    beginning: CharPosition,
    end: CharPosition,
}

#[derive(Error, Debug)]
pub enum ExpressionWithContextFromIterError {
    #[error("not a brainfuck expression")]
    NotAnExpression,
    #[error("the expression beginning at {} and ending at {} 
            evaluates to the null operation", .0.beginning, .0.end)]
    DegenerateExpression(ExpressionWithContext),
    #[error("consumed all of the iterator")]
    EndOfIterator,
}

impl ExpressionWithContext {
    fn new(expression: Expression, beginning: CharPosition, end: CharPosition) -> Self {
        Self {
            expression: expression,
            beginning: beginning,
            end: end,
        }
    }
}

impl<I> TryFrom<&mut std::iter::Peekable<I>> for ExpressionWithContext
where
    I: std::iter::Iterator<Item = (CharPosition, char)>,
{
    type Error = ExpressionWithContextFromIterError;

    fn try_from(iter: &mut std::iter::Peekable<I>) -> Result<Self, Self::Error> {
        let (beginning, first_character) = iter.next().ok_or(Self::Error::EndOfIterator)?;

        let mut expression =
            Expression::try_from(first_character).map_err(|_| Self::Error::NotAnExpression)?;
        let mut end = beginning.clone();

        while let Some((_, character)) = iter.peek() {
            if let Some(next_expression) = Expression::try_from(character)
                .map(|e| expression.combine(&e))
                .ok()
                .flatten()
            {
                if next_expression.is_degenerate() {
                    return Err(Self::Error::DegenerateExpression(Self::new(
                        next_expression,
                        beginning,
                        end,
                    )));
                }

                expression = next_expression;
            } else {
                return Ok(Self::new(expression, beginning, end));
            }

            end = iter.next().unwrap().0;
        }

        return Ok(Self::new(expression, beginning, end));
    }
}

pub struct Expressions {
    expressions_raw: Vec<Expression>,
}

impl fmt::Debug for Expressions {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for expression in &self.expressions_raw {
            result = format!("{}{:?}\n", result, expression)
        }

        write!(formatter, "{}", result)
    }
}

impl fmt::Display for Expressions {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.expressions_raw
                .iter()
                .fold(String::new(), |a, l| a + &l.to_string())
        )
    }
}

pub struct JumpAndCharPosition {
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

#[derive(Error, Debug)]
pub enum ExpressionsParseError {
    #[error("unbalanced \"[\" at {}", .0)]
    MismatchedOpen(CharPosition),
    #[error("unbalanced \"]\" at {}", .0)]
    MismatchedClose(CharPosition),
}

impl str::FromStr for Expressions {
    type Err = ExpressionsParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut expressions_raw: Vec<Expression> = Vec::with_capacity(100);
        let mut brackets = Vec::with_capacity(20);
        let mut positions = CharPositionEnumerate::from(string).peekable();

        loop {
            match ExpressionWithContext::try_from(&mut positions) {
                Ok(expression_with_context) => {
                    let mut expression = expression_with_context.expression;
                    let position = expression_with_context.beginning;

                    if expression.is_open() {
                        brackets.push(JumpAndCharPosition::new(expressions_raw.len(), position));
                    } else if expression.is_close() {
                        let open = brackets
                            .pop()
                            .ok_or_else(|| Self::Err::MismatchedClose(position))?
                            .jump_to;

                        let close = expressions_raw.len();

                        expressions_raw[open].modify_argument(|_| close);
                        expression.modify_argument(|_| open);
                    } else if expression.is_left() || expression.is_right() {
                        expression.modify_argument(|a| a % 30_000);
                    } else if expression.is_add() || expression.is_sub() {
                        expression.modify_argument(|a| a % 256);
                    }

                    expressions_raw.push(expression);
                }

                Err(e) => match e {
                    ExpressionWithContextFromIterError::NotAnExpression => {}
                    ExpressionWithContextFromIterError::DegenerateExpression(_) => {
                        panic!();
                    }
                    ExpressionWithContextFromIterError::EndOfIterator => break,
                },
            }
        }

        if let Some(mismatched) = brackets.pop() {
            Err(Self::Err::MismatchedOpen(mismatched.position))
        } else {
            Ok(Expressions {
                expressions_raw: expressions_raw,
            })
        }
    }
}

impl VMRunnable for Expressions {
    fn run<R, W>(&self, registers: &mut VMRegisters, writer: &mut W, reader: &mut R)
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        let len = self.expressions_raw.len();

        while registers.pc() < len {
            match self.expressions_raw[registers.pc()] {
                Expression::Left(a) => {
                    if a > registers.head() {
                        registers.head_to(30_000 - (a - registers.head()))
                    } else {
                        registers.head_to(registers.head() - a)
                    }
                }

                Expression::Right(a) => registers.head_to((registers.head() + a) % 30_000),

                Expression::Add(a) => {
                    *registers.cell_mut() = registers.cell().wrapping_add(a as u8)
                }

                Expression::Sub(a) => {
                    *registers.cell_mut() = registers.cell().wrapping_sub(a as u8)
                }

                Expression::Input(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        reader.read_exact(&mut buffer).unwrap();
                        *registers.cell_mut() = buffer[0];
                    }
                }

                Expression::Output(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        buffer[0] = registers.cell();
                        writer.write(&mut buffer).unwrap();
                    }
                }

                Expression::Open(close) => {
                    if registers.cell() == 0 {
                        registers.jump_to(close as usize - 1);
                    }
                }

                Expression::Close(open) => {
                    if registers.cell() != 0 {
                        registers.jump_to(open as usize);
                    }
                }
            }

            registers.increase_pc();
        }
    }
}
