use std::fs::{self};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

pub fn execute() -> Result<(), io::Error> {
    // receive()
    let stdin = io::stdin();
    receive_from(stdin)
    // Ok(())
}

pub fn receive_from<R: Read>(input: R) -> Result<(), io::Error> {
    // let stdin = io::stdin();
    //instead of using stdin we gonna make input Generic
    //input can be anythng that implements the read trait
    //BufReader::new takes something which implement Read Trait
    let mut reader = BufReader::new(input);
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

        if parts.len() != 4 || parts[0] != "FILE" {
            eprintln!("Invalid Header: {}", header);
            return Ok(());
        }
        let expected_hash = parts[3];
        let mut hasher = Sha256::new();
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
        while remaining > 0 {
            let mut chunk_header = String::new();
            let bytes = reader.read_line(&mut chunk_header)?;
            if bytes == 0 {
                eprintln!("Unexpected EOF while reading chunk header");
                return Ok(());
            }
            let parts: Vec<&str> = chunk_header.trim().split_whitespace().collect();
            if parts.len() != 2 || parts[0] != "CHUNK" {
                eprintln!("Invalid chunk header: {}", chunk_header);
                return Ok(());
            }
            let chunk_size: usize = match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid chunk size");
                    return Ok(());
                }
            };
            let mut chunk_buffer = vec![0u8; chunk_size];
            let mut read_total = 0;
            while read_total < chunk_size {
                let n = reader.read(&mut chunk_buffer[read_total..])?;
                if n == 0 {
                    eprintln!("Unexpected EOF in chunk data");
                    return Ok(());
                }
                read_total += n;
            }
            file.write_all(&chunk_buffer)?;
            hasher.update(&chunk_buffer);
            remaining -= chunk_size;
        }
        let computed_hash = format!("{:x}", hasher.finalize());
        if computed_hash != expected_hash {
            eprintln!("Hash mismatched for file {:?}", path);
            fs::remove_file(path)?;
            return Ok(());
        }
    }
    Ok(())
}
