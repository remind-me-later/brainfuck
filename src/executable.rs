use std::io::{Read, Write};

pub trait Executable {
    fn execute<R: Read, W: Write>(&self, write: &mut W, read: &mut R);
}
