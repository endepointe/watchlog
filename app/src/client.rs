use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5001")?;
    // this header information contains the name of the directory to save the file to.
    // Example: if the transfer inverval is set to every hour, the directory name will be the 
    // current date and the child files will be saved in that directory.
    stream.write_all(b"send over header information")?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer));
    Ok(())
}
