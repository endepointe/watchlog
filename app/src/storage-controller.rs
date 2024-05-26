use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;


fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {:?}", stream);
    // replace file.txt with the header information sent from the client
    // replace with openoptions
    let mut file = File::create("file.txt").unwrap();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        file.write(&buffer[..bytes_read]).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5001")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
