use crate::pipe::pipe;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn execute(files: Vec<String>) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    if files.is_empty() {
        let mut stdin = io::stdin();
        pipe(&mut stdin, &mut stdout)?;
        return Ok(());
    }
    for path in files {
        let path_ref = Path::new(&path);
        if path_ref.is_file() {
            send_file(path_ref, &mut stdout)?;
        } else if path_ref.is_dir() {
            send_dir(path_ref, &mut stdout)?;
        } else {
            eprintln!("Invalid path {}", path)
        }
    }
    Ok(())
}

fn send_file(path: &Path, stdout: &mut impl Write) -> Result<(), io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {} {}", path.display(), e);
            return Ok(());
        }
    };
    pipe(&mut file, stdout)?;
    Ok(())
}

fn send_dir(path: &Path, stdout: &mut impl Write) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            send_file(&entry_path, stdout)?;
        } else if entry_path.is_dir() {
            send_dir(&entry_path, stdout)?;
        }
    }
    Ok(())
}

