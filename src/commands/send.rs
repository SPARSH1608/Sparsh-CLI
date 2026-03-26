use crate::pipe::pipe;
use std::fs::File;
use std::io;

pub fn execute(file: Option<String>) -> Result<(), io::Error> {
    match file {
        Some(path) => {
            let mut input = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open file '{}': {}", path, e);
                    return Ok(());
                }
            };

            let mut output = io::stdout();
            pipe(&mut input, &mut output)?;
        }
        None => {
            let mut input = io::stdin();
            let mut output = io::stdout();
            pipe(&mut input, &mut output)?;
        }
    }

    Ok(())
}
