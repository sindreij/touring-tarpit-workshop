use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_mem() -> Result<Vec<u8>, Box<Error>> {
    let args: Vec<_> = env::args().collect();
    let programfile = &args[1];

    let mut memfile = File::open(programfile)?;

    let mut mem = Vec::new();
    memfile.read_to_end(&mut mem)?;

    Ok(mem)
}

fn main() -> Result<(), Box<Error>> {
    let mem = read_mem()?;
    let mut pc = 0;

    loop {
        let inst = mem[pc];
        pc = pc + 1;
        match inst {
            0 => {
                // HALT
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown instruction {}", inst);
                std::process::exit(1);
            }
        }
    }
}
