use std::fs::{self};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

pub fn execute() -> Result<(), io::Error> {
    recieve()
}

fn recieve() -> Result<(), io::Error> {
    let stdin = io::stdin();
    //BufReader::new takes something which implement Read Trait
    let mut reader = BufReader::new(stdin);
    loop {
        let mut header = String::new();
        //read_line Reads all bytes until a newline
        let bytes = reader.read_line(&mut header)?;
        if bytes == 0 {
            break;
        }
        //Splits a string at most n times, producing ≤ n parts. split by ' '
        let parts: Vec<&str> = header.trim().splitn(4, ' ').collect();

        if parts.len() < 4 || parts[0] != "FILE" {
            eprintln!("Invalid Header: {}", header);
            return Ok(());
        }
        let path = parts[2];
        let size: usize = match parts[3].parse() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Invalid size in header: {}", header);
                return Ok(());
            }
        };

        let path_ref = Path::new(path);
        //Recursively create a directory and all of its parent components if they are missing.
        if let Some(parent) = path_ref.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path_ref)?;

        let mut remaining = size;
        let mut buffer = [0; 1024];
        while remaining > 0 {
            let read_size = remaining.min(1024);
            let bytes_read = reader.read(&mut buffer[..read_size])?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
            remaining -= bytes_read;
        }
    }
    Ok(())
}
