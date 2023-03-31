// Client

use std::env;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
// use std::{thread, time};

fn start_client(port: u16, iterations: u16) -> Result<(), Error> {
    let addr = format!("{}:{}", "127.0.0.1", port);
    match TcpStream::connect(addr) {
        Ok(stream) => {
            eprintln!("Successfully connected to server in port {}", port);
            if let Err(e) = handle_response(stream, iterations) {
                eprintln!("Error {:?} has occurred.", e);
            };
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
    eprintln!("Terminated.");
    Ok(())
}

fn handle_response(mut stream: TcpStream, iterations: u16) -> Result<(), Error> {
    for iteration in 0..iterations {
        let mut buf = [0; 512];
        let itr: String = "Iteration ".to_owned() + &format!("{}", iteration);

        stream.write_all(itr.as_bytes()).unwrap();
        eprintln!("Sent Hello_world, awaiting reply...");
        let bytes_read = stream.read(&mut buf)?;
        let server_response = std::str::from_utf8(&buf[..bytes_read]).unwrap();
        eprintln!("Server responded with: {}", server_response);
    }
    Ok(())
}

fn main() {
    let port: u16;
    let iterations: u16 = 1;

    if let Some(arg1) = env::args().nth(1) {
        port = arg1.parse().unwrap();
        eprintln!("Starting a client...");
        match start_client(port, iterations){
          Ok(_) => eprintln!("OK!"),
          _ => eprintln!("ERRRO")
        }
    } else {
        eprintln!("Cannot run sever, must provide a port number");
    }
}
