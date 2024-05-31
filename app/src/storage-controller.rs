mod types;
use crate::types::{Header};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::thread;
use serde::{Deserialize};

fn 
handle_client(mut stream: TcpStream) 
{
    println!("Incoming data from: {:?}", stream);
    let mut file = std::fs::OpenOptions::new().append(true).create(true).open("storage.test").unwrap();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let mut header = String::new();
        for byte in &buffer[..bytes_read] {
            if *byte == 0x7D {
                header.push(*byte as char);
                break;
            }
            header.push(*byte as char);
        }
        let header: Header = serde_json::from_str(&header).unwrap();
        println!("Header: {:?}", &header);
        file.write(&buffer[..bytes_read]).unwrap();
    }
}

fn 
main() -> std::io::Result<()> 
{
    let listener = TcpListener::bind("127.0.0.1:5001")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });

            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
