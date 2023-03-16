// Client

use std::io::{Error, Read, Write};
use std::net::TcpStream;
// use std::thread;

fn start_client(port: u16) {
    let addr = format!("{}:{}", "0.0.0.0", port);
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected to server in port {}", port);
            if let Err(e) = handle_response(stream) {
                println!("Error {:?} has occurred.", e);
            };
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn handle_response(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];
    let msg = b"Hello_world";

    stream.write_all(msg).unwrap();
    println!("Sent Hello_world, awaiting reply...");
    let bytes_read = stream.read(&mut buf)?;
    let server_response = std::str::from_utf8(&buf[..bytes_read]).unwrap();
    println!("Server responded with: {}", server_response);

    return Ok(());
}

fn main() {
    let port = 1234;
    // let handler_client = thread::spawn(move || start_client(port));
    // if let Err(e) = handler_client.join() {
    //     println!("Error in join client thread. . Error {:?}", e)
    // };
    start_client(port);
}
