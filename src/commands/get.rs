use std::fs::File;
use std::io;

use crate::pipe::pipe;
pub fn execute() -> Result<(), io::Error> {
    let mut input = io::stdin();
    let mut output = File::create("out.txt")?;
    pipe(&mut input, &mut output)?;
    Ok(())
}
