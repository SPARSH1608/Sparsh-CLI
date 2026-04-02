use crate::pipe::pipe;
use crate::utils::compute_hash;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn execute(files: Vec<String>) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    send_to(files, &mut stdout)
}
pub fn send_to<W: Write>(files: Vec<String>, writer: &mut W) -> Result<(), io::Error> {
    // let mut stdout = io::stdout();
    if files.is_empty() {
        let mut stdin = io::stdin();
        pipe(&mut stdin, writer)?;
        return Ok(());
    }
    for path in files {
        let path_ref = Path::new(&path);
        if path_ref.is_file() {
            let base = path_ref.parent().unwrap_or(Path::new(""));
            send_file(base, path_ref, writer)?;
        } else if path_ref.is_dir() {
            send_dir(path_ref, path_ref, writer)?;
        } else {
            eprintln!("Invalid path {}", path)
        }
    }
    Ok(())
}

fn send_file(base: &Path, path: &Path, writer: &mut impl Write) -> Result<(), io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {} {}", path.display(), e);
            return Ok(());
        }
    };
    let metadata = file.metadata()?;
    let size = metadata.len();

    let rel_path = path.strip_prefix(base).unwrap();
    let rel_str = rel_path.to_string_lossy();

    let path_bytes = rel_str.as_bytes();
    let path_len = rel_str.len();
    let hash = compute_hash(path)?;
    //FILE 9 5\n
    let header = format!("FILE {} {} {}\n", path_len, size, hash);
    writer.write_all(header.as_bytes())?;
    //sub/c.txt
    writer.write_all(path_bytes)?;

    // pipe(&mut file, writer)?;
    send_chunks(&mut file, writer)?;
    Ok(())
}

fn send_dir(base: &Path, path: &Path, stdout: &mut impl Write) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            send_file(base, &entry_path, stdout)?;
        } else if entry_path.is_dir() {
            send_dir(base, &entry_path, stdout)?;
        }
    }
    Ok(())
}
//file header FILE <path_length> <path> <size>\n
//File 9 5 \n
//sub/c.txthello

fn send_chunks(file: &mut impl Read, writer: &mut impl Write) -> Result<(), io::Error> {
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let header = format!("CHUNK {}\n", bytes_read);
        writer.write_all(header.as_bytes())?;
        writer.write_all(&buffer[..bytes_read])?;
    }
    Ok(())
}
