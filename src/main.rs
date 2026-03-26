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
        if args.len()>=3 {
            let file_path=&args[2];
            let mut file=File::open(file_path)?;
            send_stream(&mut file)?;
        }else {
            let mut stdin=io::stdin();
            send_stream(&mut stdin)?;
        }
    } else {
        println!("unknown command")
    }
    Ok(())
}

//reader is any type T which implements the Read Trait
fn send_stream<T:Read>(reader:&mut T)->Result<(), io::Error>{
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        println!("Sending {} bytes", bytes_read);
    }

    Ok(())
}
