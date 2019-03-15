use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian};

struct Machine {
    mem: Vec<u8>,
    pc: i32,
}

enum Instruction {
    Halt,
    Out(i32),
    BranchIfPlus(i32, i32),
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

    fn fetch_pointer(&mut self) -> i32 {
        let res = LittleEndian::read_i32(&self.mem[(self.pc as usize)..]);
        self.pc += 4;
        res
    }

    fn next_instruction(&mut self) -> Result<Instruction, String> {
        let inst = self.mem[self.pc as usize];
        self.pc += 1;

        use Instruction::*;
        match inst {
            0 => Ok(Halt),
            1 => Ok(Out(self.fetch_pointer())),
            2 => Ok(BranchIfPlus(self.fetch_pointer(), self.fetch_pointer())),
            _ => Err(format!("Unknown instruction {}", inst)),
        }
    }

    fn run(&mut self) -> Result<(), Box<Error>> {
        loop {
            // let inst = self.mem[self.pc];
            let inst = self.next_instruction()?;

            use Instruction::*;
            match inst {
                Halt => {
                    std::process::exit(0);
                }
                Out(ptr) => {
                    let ch = self.mem[ptr as usize] as char;
                    print!("{}", ch);
                }
                BranchIfPlus(jmpptr, srcptr) => {
                    if self.mem[srcptr as usize] < 128 {
                        self.pc = jmpptr;
                    }
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
