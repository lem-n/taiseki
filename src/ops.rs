#[derive(Debug, Clone)]
pub enum OpEnum {
    Noop,
    Push,
    Add,
    Sub,
    Print,
    Halt,
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

pub fn get_op(i: usize) -> Opcode {
    OP_TABLE[i].clone()
}
