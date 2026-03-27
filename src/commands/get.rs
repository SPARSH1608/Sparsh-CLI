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
        //read_line appends bytes from stdin into this buffer
        let bytes = reader.read_line(&mut header)?;
        if bytes == 0 {
            break;
        }
        //Splits a string at most n times, producing ≤ n parts. split by ' '
        let parts: Vec<&str> = header.trim().split_whitespace().collect();

        if parts.len() != 3 || parts[0] != "FILE" {
            eprintln!("Invalid Header: {}", header);
            return Ok(());
        }
        let path_len: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid path_len: {}", header);
                return Ok(());
            }
        };

        let size: usize = match parts[2].parse() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Invalid size in header: {}", header);
                return Ok(());
            }
        };
        let mut path_buf = vec![0u8; path_len];
        let mut read_total = 0;
        while read_total < path_len {
            let n = reader.read(&mut path_buf[read_total..])?;
            if n == 0 {
                eprintln!("Unexpected EOF while reading path");
                return Ok(());
            }
            read_total += n;
        }

        let path_str = match String::from_utf8(path_buf) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Invalid UTF-8 path");
                return Ok(());
            }
        };

        let path = Path::new(&path_str);
        //Recursively create a directory and all of its parent components if they are missing.
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;

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
