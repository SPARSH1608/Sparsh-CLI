use crate::commands::{get::receive_from, send::send_to};
use std::{
    error::Error,
    io,
    net::{TcpListener, TcpStream},
};

pub fn start_server(port: u16) -> Result<(), io::Error> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr)?;

    eprintln!(" Listening on port {}", port);
    //Accept a new incoming connection from this listener.
    let (stream, _) = listener.accept()?;
    receive_from(stream)
}
pub fn send_tcp(files: Vec<String>, addr: &str) -> Result<(), io::Error> {
    let mut stream = TcpStream::connect(&addr)?;
    send_to(files, &mut stream)
}
