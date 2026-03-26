use crate::pipe::pipe;
use std::fs::File;
use std::io;

pub fn execute(files: Vec<String>) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    if files.is_empty() {
        let mut stdin = io::stdin();
        pipe(&mut stdin, &mut stdout)?;
        return Ok(());
    }
    for path in files {
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open {} ,{}", path, e);
                continue;
            }
        };
        pipe(&mut file, &mut stdout)?;
    }
    Ok(())
}
