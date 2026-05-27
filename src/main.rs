use bf_interpreter::Result;
use bf_interpreter::Runtime;
use bf_interpreter::errs::Error;
use std::{env, fs, io};

fn main() -> Result<()> {
    let mut runtime = Runtime::default();

    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: bf_interpreter file.bf");
            return Err(Error::ArgumentError(
                "Expected file path as argument".into(),
            ));
        }
    };

    let code = fs::read(path)?;
    let _ = runtime.run(code);

    Ok(())
}
