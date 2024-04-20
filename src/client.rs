use std::path::Path;
use tokio::net::UnixStream;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket_path = "socket"; // this file needs to be deleted when server stops.

    let mut stream = UnixStream::connect(socket_path).await?;

    loop {
        println!("Menu:");
        println!("1. Load log source");
        println!("2. Stop log source");
        println!("3. Reload log source");
        println!("4. Get log source status");
        println!("Enter your choice:");

        let mut input = String::new();
        let mut stdin = io::BufReader::new(io::stdin());
        stdin.read_line(&mut input).await?;

        let num: u32 = match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 4 => num,
            _ => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        stream.write_all(format!("{}\n", num).as_bytes()).await?;

        let mut response = String::new();
        stream.read_to_string(&mut response).await?;
        print!("{}", response);
    }
}

