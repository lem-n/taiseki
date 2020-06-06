mod ops;
mod parser;
mod runner;

use runner::RunnerError;
use std::env;

fn main() -> Result<(), RunnerError> {
    let cmd_args: Vec<String> = env::args().collect();
    if cmd_args.len() < 2 {
        return Err(RunnerError::NoInputProvided);
    }

    let filename = &cmd_args[1];
    let code = parser::parse(filename)?;

    println!("program = {:?}", code);
    runner::run(code)?;

    Ok(())
}
