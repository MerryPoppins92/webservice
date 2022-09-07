use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello fdp ffffffffffffffffffff".as_bytes()).unwrap();
    let mut buffer = [0; 9];
    stream.read(&mut buffer).unwrap();
    println!(
        "got response from server{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}