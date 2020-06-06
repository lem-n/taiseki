use crate::ops::*;
use crate::runner::RunnerError;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub struct Program {
    code: Vec<i32>,
    text: HashMap<String, String>,
}

// fn parse_arg(x: &str) -> i32 {
// }

pub fn parse(filename: &str) -> Result<Vec<i32>, RunnerError> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => panic!(
            "Invalid input file, file '{}' does seem to exist!",
            filename
        ),
    };
    let lines = BufReader::new(file).lines();

    let mut code = vec![];
    for line in lines {
        if let Ok(ln) = line {
            let split: Vec<&str> = ln.split(' ').collect();
            let op = match split[0] {
                "noop" => get_op(OpEnum::Noop),
                "push" => get_op(OpEnum::Push),
                "add" => get_op(OpEnum::Add),
                "sub" => get_op(OpEnum::Sub),
                "print" => get_op(OpEnum::Print),
                "iread" => get_op(OpEnum::IRead),
                "halt" => get_op(OpEnum::Halt),
                _ => get_op(OpEnum::Noop),
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
