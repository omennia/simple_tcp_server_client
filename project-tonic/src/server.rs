use config::monitor_client::MonitorClient;
use config::HelloRequest;
use std::env;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

pub mod config {
    tonic::include_proto!("config");
}

async fn start_server(port: u16) {
    let addr = format!("{}:{}", "0.0.0.0", port);
    println!("Address: {}", addr);
    let listener = TcpListener::bind(addr).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                println!("Successfully got the stream yeah! {:?}", stream);
                if let Err(e) = handle_client(stream).await {
                    println!("{:?}", e);
                };
            }
        }
    }
    println!("Finished the whole process...");
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    if let Err(e) = connect_to_count().await {
        eprintln!("Erro ao connectar com o monitor {:?}", e);
    };
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

        stream.write_all(my_sentence.as_bytes())?;
    }
}

async fn connect_to_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MonitorClient::connect("http://127.0.0.1:9999").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let _response = client.contact(request).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // default configuration
    let port: u16;

    if let Some(arg1) = env::args().nth(1) {
        port = arg1.parse().unwrap();
        start_server(port).await;
    } else {
        println!("Cannot run sever, must provide a port number");
    }

    Ok(())
}
