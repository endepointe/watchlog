use std::io::{self, Write};
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket_path = "./socket.sock";

    loop {
        println!("Menu:");
        println!("1. Start logs");
        println!("2. Stop logs");
        println!("3. Terminate server");
        println!("4. Close menu");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => send_command(&socket_path, "start_logging").await?,
            "2" => send_command(&socket_path, "stop_logging").await?,
            "3" => send_command(&socket_path, "terminate_server").await?,
            "4" => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}

async fn send_command(socket_path: &str, command: &str) -> io::Result<()> {
    let mut stream = UnixStream::connect(socket_path).await?;
    stream.write_all(command.as_bytes()).await?;
    println!("Command '{}' sent.", command);
    Ok(())
}
