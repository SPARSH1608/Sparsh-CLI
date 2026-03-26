use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
};
//Testing main
// fn main()->Result<(),io::Error>{
// file → file
// let mut input = File::open("test.txt")?;
// let mut output = File::create("out.txt")?;

// stdin → stdout
// let mut input=io::stdin();
// let mut output=io::stdout();

// file → stdout
// let mut input=File::open("test.txt")?;
// let mut output=io::stdout();

// stdin -> file
//     let mut input = io::stdin();
//     let mut output=File::create("out.txt")?;
//     pipe(&mut input, &mut output)?;
//     Ok(())
// }

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
        if args.len() >= 3 {
            let file_path = &args[2];
            let mut input = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open file '{}': {}", file_path, e);
                    return Ok(());
                }
            };

            let mut output = io::stdout();
            pipe(&mut input, &mut output)?;
        } else {
            let mut input = io::stdin();
            let mut output = io::stdout();
            pipe(&mut input, &mut output)?;
        }
    } else if command == "get" {
        let mut input = io::stdin();
        let mut output = File::create("out.txt")?;
        pipe(&mut input, &mut output)?;
    } else {
        println!("unknown command")
    }
    Ok(())
}

//reader is any type T which implements the Read Trait
// fn send_stream<T:Read>(reader:&mut T)->Result<(), io::Error>{
//     let mut buffer = [0; 1024];

//     loop {
//         let bytes_read = reader.read(&mut buffer)?;

//         if bytes_read == 0 {
//             break;
//         }

//         println!("Sending {} bytes", bytes_read);
//     }

//     Ok(())
// }
fn pipe<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), io::Error> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
    }
    Ok(())
}
