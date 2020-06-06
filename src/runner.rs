use crate::ops::*;
use std::io;

#[derive(Debug)]
pub enum RunnerError {
    NoInputProvided,
}

struct Stack {
    cursor: usize,
    array: [i32; 500],
}

impl Stack {
    pub fn create() -> Self {
        Self {
            cursor: 0,
            array: [0; 500],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.array[self.cursor] = val;
        self.cursor += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if self.cursor > 0 {
            self.cursor -= 1;
            let last = self.array[self.cursor];
            self.array[self.cursor] = 0;
            last
        } else {
            let tmp = self.array[self.cursor];
            self.array[self.cursor] = 0;
            tmp
        }
    }

    pub fn print(&self, count: usize) {
        let to_idx = if count > self.array.len() {
            self.array.len()
        } else {
            count
        };
        let slice = &self.array[0..to_idx];
        println!("Stack {:?}", slice);
    }
}

fn read_stdin() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    inp
}

fn parse_num(a: &str) -> i32 {
    a.trim().parse().unwrap()
}

pub fn run(code: Vec<i32>, debug: bool) -> Result<(), RunnerError> {
    let mut stack = Stack::create();
    let mut exit = false;
    let mut ip = 0;

    while !exit && ip < code.len() {
        let op: Opcode = get_op(OpEnum::from(code[ip]));
        if debug {
            print!("{:?}\t", op.op);
            stack.print(10);
        }

        match op.op {
            OpEnum::Noop => {}
            OpEnum::Push => {
                ip += 1;
                stack.push(code[ip]);
            }
            OpEnum::Add => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(a + b);
            }
            OpEnum::Sub => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(b - a);
            }
            OpEnum::Print => {
                let val = stack.pop();
                println!("{:?}", val);
            }
            OpEnum::IRead => {
                let input = read_stdin();
                let num = parse_num(&input);
                stack.push(num);
            }
            OpEnum::Halt => exit = true,
        }
        ip += 1;
    }
    Ok(())
}
