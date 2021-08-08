use std::io::{Read, Write};

const TAPE_LENGTH: usize = 30_000;

pub struct VMRegisters {
    pc: usize,
    head: usize,
    tape: [u8; TAPE_LENGTH],
}

impl Default for VMRegisters {
    fn default() -> Self {
        VMRegisters {
            pc: 0,
            head: 0,
            tape: [0; TAPE_LENGTH],
        }
    }
}

impl VMRegisters {
    pub fn cell(&self) -> u8 {
        self.tape[self.head]
    }

    pub fn cell_mut(&mut self) -> &mut u8 {
        &mut self.tape[self.head]
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn head_to(&mut self, position: usize) {
        self.head = position
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn jump_to(&mut self, position: usize) {
        self.pc = position
    }

    pub fn increase_pc(&mut self) {
        self.pc += 1
    }
}

pub struct VM<'a, P: VMRunnable> {
    registers: VMRegisters,
    parts: &'a P,
}

impl<'a, P> VM<'a, P>
where
    P: VMRunnable,
{
    pub fn new(parts: &'a P) -> Self {
        Self {
            registers: VMRegisters::default(),
            parts: parts,
        }
    }

    pub fn run<R, W>(&mut self, write: &mut W, read: &mut R)
    where
        R: Read,
        W: Write,
    {
        self.parts.run(&mut self.registers, write, read);
    }
}

pub trait VMRunnable {
    fn run<R, W>(&self, registers: &mut VMRegisters, writer: &mut W, reader: &mut R)
    where
        R: Read,
        W: Write;
}
