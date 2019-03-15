use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian};

struct Machine {
    mem: Vec<u8>,
    pc: usize,
}

impl Machine {
    fn from_args() -> Result<Machine, Box<Error>> {
        let args: Vec<_> = env::args().collect();
        let programfile = &args[1];

        let mut memfile = File::open(programfile)?;

        let mut mem = Vec::new();
        memfile.read_to_end(&mut mem)?;

        Ok(Machine { mem, pc: 0 })
    }

    fn fetch_pointer(&mut self) -> usize {
        let bytes = &self.mem[self.pc..self.pc + 4];
        self.pc += 4;
        LittleEndian::read_i32(bytes) as usize
    }

    fn run(&mut self) -> Result<(), Box<Error>> {
        loop {
            let inst = self.mem[self.pc];
            self.pc += 1;

            match inst {
                0 => {
                    // HALT
                    std::process::exit(0);
                }

                1 => {
                    let ptr = self.fetch_pointer();
                    let ch = self.mem[ptr] as char;
                    print!("{}", ch);
                }
                _ => {
                    eprintln!("Unknown instruction {}", inst);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let mut machine = Machine::from_args()?;
    machine.run()?;

    Ok(())
}
