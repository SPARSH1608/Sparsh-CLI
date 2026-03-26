use std::{
    env,
    fs::{self, File},
    io::{self, Read},
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    //env::args() produces owned Strings.
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Usage: sparsh send <file>");
        return Ok(());
    }

    let command = &args[1];

    if command == "send" {
        if args.len() < 3 {
            println!("No file found")
        }
        let file_path = &args[2];
        send_file(file_path)?;
    } else {
        println!("unknown command")
    }
    Ok(())
}

fn send_file(path: &str) -> Result<(), io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        println!("Sending {} bytes", bytes_read);
    }

    Ok(())
}
