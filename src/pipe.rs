use std::io::{self, Read, Write};

pub fn pipe<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), io::Error> {
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
