use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
// use std::thread;

fn start_server(port: u16) {
    let addr = format!("{}:{}", "0.0.0.0", port);
    println!("Address: {}", addr);
    let listener = TcpListener::bind(addr).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                println!("Successfully got the stream yeah! {:?}", stream);
                if let Err(e) = handle_client(stream) {
                    println!("{:?}", e)
                }; // manda converter para maiusculas
                break;
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        let my_sentence = std::str::from_utf8(&buf[..bytes_read])
            .unwrap()
            .to_uppercase();

        println!("Aqui vão os bytes read: {}", bytes_read);
        if bytes_read == 0 {
            return Ok(());
        }

        // stream.write(&buf[..bytes_read])?;
        stream.write_all(my_sentence.as_bytes())?;
    }
}

fn main() {
    // default configuration
    let port = 1234;
    start_server(port);
}
