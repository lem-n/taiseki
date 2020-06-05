use crate::ops::*;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn parse(filename: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let mut code = vec![];
    for line in lines {
        if let Ok(ln) = line {
            let split: Vec<&str> = ln.split(' ').collect();
            let op = match split[0] {
                "noop" => get_op(OpEnum::Noop as usize),
                "push" => get_op(OpEnum::Push as usize),
                "add" => get_op(OpEnum::Add as usize),
                "sub" => get_op(OpEnum::Sub as usize),
                "print" => get_op(OpEnum::Print as usize),
                "halt" => get_op(OpEnum::Halt as usize),
                _ => get_op(OpEnum::Noop as usize),
            };

            code.push(op.op as i32);
            // also push args
            if op.arg_count > 0 {
                let arg_slice = &split[1..];
                let args: Vec<i32> = arg_slice
                    .iter()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                code.extend_from_slice(&args[..]);
            }
        }
    }
    Ok(code)
}
