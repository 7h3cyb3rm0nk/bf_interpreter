use bf_interpreter::Runtime;
use std::{env, fs, process};

fn main() {
    let mut runtime = Runtime::default();
    if let Some(path) = env::args().nth(1) {
        match fs::read(path) {
            Ok(code) => runtime.run(code),
            _ => {
                eprintln!("failed to open file!");
                process::exit(1);
            }
        }
    } else {
        eprintln!("expected file path as an argument");
        process::exit(1);
    }
}
