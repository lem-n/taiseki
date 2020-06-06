use crate::ops::*;

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
}

#[allow(dead_code)]
fn print_stack_slice(stack: &Stack, count: usize) {
    let to = if count > stack.array.len() {
        stack.array.len()
    } else {
        count
    };
    let slice = &stack.array[0..to];
    print!("stack [");
    for n in slice {
        print!(" {},", n);
    }
    println!(" ]");
}

pub fn run(code: Vec<i32>) -> Result<(), RunnerError> {
    let mut stack = Stack::create();
    let mut ip = 0;
    let mut exit = false;

    while !exit && ip < code.len() {
        let op: Opcode = get_op(OpEnum::from(code[ip]));
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
            OpEnum::Halt => exit = true,
        }
        ip += 1;
    }
    Ok(())
}
