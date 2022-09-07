mod handler;
mod server;
use server::Server;
mod router;

fn main() {
    // println!("Hello, world!");
    // start server
    let server = Server::new("localhost:3000");
    server.run();
}
