use std::io;

use crate::ir_instruction::IRInstruction;

const TAPE_LENGTH: usize = 30_000;

pub struct VM<'a> {
    pc: usize,
    head: usize,
    tape: [u8; TAPE_LENGTH],
    program: &'a Vec<IRInstruction>,
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
        self.pc >= self.program.len()
    }

    pub fn new(program: &'a Vec<IRInstruction>) -> Self {
        Self {
            pc: 0,
            head: 0,
            tape: [0; TAPE_LENGTH],
            program: program,
        }
    }

    pub fn run<R, W>(&mut self, writer: &mut W, reader: &mut R)
    where
        R: io::Read,
        W: io::Write,
    {
        while !self.done() {
            match self.program[self.pc] {
                IRInstruction::NOP => (),

                IRInstruction::Left(a) => {
                    if a > self.head {
                        self.head_to(30_000 - (a - self.head))
                    } else {
                        self.head_to(self.head - a)
                    }
                }

                IRInstruction::Right(a) => self.head_to((self.head + a) % 30_000),

                IRInstruction::Add(a) => *self.cell_mut() = self.cell().wrapping_add(a as u8),

                IRInstruction::Sub(a) => *self.cell_mut() = self.cell().wrapping_sub(a as u8),

                IRInstruction::Input(times) => {
                    for _ in 0..times {
                        writer.flush().unwrap();
                        let mut buffer = [0; 1];
                        reader.read_exact(&mut buffer).unwrap();
                        *self.cell_mut() = buffer[0];
                    }
                }

                IRInstruction::Output(times) => {
                    for _ in 0..times {
                        let mut buffer = [0; 1];
                        buffer[0] = self.cell();
                        writer.write(&mut buffer).unwrap();
                    }
                }

                IRInstruction::Open(close) => {
                    if self.cell() == 0 {
                        self.jump_to(close as usize - 1);
                    }
                }

                IRInstruction::Close(open) => {
                    if self.cell() != 0 {
                        self.jump_to(open as usize);
                    }
                }
            }

            self.increase_pc();
        }
    }
}
