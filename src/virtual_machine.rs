use std::io;

use crate::ir::{Instruction, IR};

const TAPE_LENGTH: usize = 30_000;

pub struct VM<'a> {
    pc: usize,
    head: usize,
    tape: [u8; TAPE_LENGTH],
    ir: &'a IR,
}

impl<'a> VM<'a> {
    pub fn cell(&self) -> u8 {
        self.tape[self.head]
    }

    pub fn cell_mut(&mut self) -> &mut u8 {
        &mut self.tape[self.head]
    }

    pub fn head_to(&mut self, position: usize) {
        self.head = position
    }

    pub fn jump_to(&mut self, position: usize) {
        self.pc = position
    }

    pub fn increase_pc(&mut self) {
        self.pc += 1
    }

    pub fn done(&self) -> bool {
        self.pc >= self.ir.len()
    }

    pub fn new(ir: &'a IR) -> Self {
        Self {
            pc: 0,
            head: 0,
            tape: [0; TAPE_LENGTH],
            ir: ir,
        }
    }

    pub fn run<R, W>(&mut self, writer: &mut W, reader: &mut R)
    where
        R: io::Read,
        W: io::Write,
    {
        while !self.done() {
            match self.ir[self.pc] {
                Instruction::NOP => (),

                Instruction::Left(a) => {
                    if a > self.head {
                        self.head_to(30_000 - (a - self.head))
                    } else {
                        self.head_to(self.head - a)
                    }
                }

                Instruction::Right(a) => self.head_to((self.head + a) % 30_000),

                Instruction::Add(a) => *self.cell_mut() = self.cell().wrapping_add(a as u8),

                Instruction::Sub(a) => *self.cell_mut() = self.cell().wrapping_sub(a as u8),

                Instruction::Input(times) => {
                    for _ in 0..times {
                        writer.flush().unwrap();
                        let mut buffer = [0; 1];
                        reader.read_exact(&mut buffer).unwrap();
                        *self.cell_mut() = buffer[0];
                    }
                }

                Instruction::Output(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        buffer[0] = self.cell();
                        writer.write(&mut buffer).unwrap();
                    }
                }

                Instruction::Open(close) => {
                    if self.cell() == 0 {
                        self.jump_to(close as usize - 1);
                    }
                }

                Instruction::Close(open) => {
                    if self.cell() != 0 {
                        self.jump_to(open as usize);
                    }
                }
            }

            self.increase_pc();
        }
    }
}
