use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}
 impl<'a> Server<'a> {
     pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr: socket_addr }
     }
     pub fn run(&self) {
        // start a server listening in socket address
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("running on {}", self.socket_addr);
        // listen to incoming connection in a loop
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("connection established");
            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();
            // convert HTTP request to rust data structure
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            // route request appropriate handler
            Router::route(req, &mut stream);
        }
     }
 }