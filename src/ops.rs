#[derive(Debug, Clone)]
pub enum OpEnum {
    Noop,
    Push,
    Add,
    Sub,
    Print,
    Halt,
}

impl From<i32> for OpEnum {
    fn from(val: i32) -> Self {
        match val {
            0 => OpEnum::Noop,
            1 => OpEnum::Push,
            2 => OpEnum::Add,
            3 => OpEnum::Sub,
            4 => OpEnum::Print,
            5 => OpEnum::Halt,
            _ => OpEnum::Noop,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Opcode {
    pub op: OpEnum,
    pub arg_count: i32,
}

const OP_TABLE: [Opcode; 6] = [
    Opcode {
        op: OpEnum::Noop,
        arg_count: 0,
    },
    Opcode {
        op: OpEnum::Push,
        arg_count: 1,
    },
    Opcode {
        op: OpEnum::Add,
        arg_count: 0,
    },
    Opcode {
        op: OpEnum::Sub,
        arg_count: 0,
    },
    Opcode {
        op: OpEnum::Print,
        arg_count: 0,
    },
    Opcode {
        op: OpEnum::Halt,
        arg_count: 0,
    },
];

pub fn get_op(i: OpEnum) -> Opcode {
    OP_TABLE[i as usize].clone()
}
