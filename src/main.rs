mod ops;
mod parser;

use ops::*;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let cmd_args: Vec<String> = env::args().collect();
    if cmd_args.len() < 2 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    let filename = &cmd_args[1];
    let code = parser::parse(filename)?;
    println!("program = {:?}", code);

    let mut stack = vec![];
    let mut ip = 0;
    let mut exit = false;

    while ip < code.len() || !exit {
        let op: Opcode = get_op(code[ip] as usize);
        match op.op {
            OpEnum::Noop => {}
            OpEnum::Push => {
                ip += 1;
                stack.push(code[ip]);
            }
            OpEnum::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            OpEnum::Print => {
                let val = stack.pop().unwrap();
                println!("{:?}", val);
            }
            OpEnum::Halt => exit = true,
            _ => {
                println!("Unknown opcode");
            }
        }
        ip += 1;
    }
    Ok(())
}
