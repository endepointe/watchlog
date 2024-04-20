use std::path::Path;
use tokio::net::UnixListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket_path = "socket";

    let listener = UnixListener::bind(socket_path)?;

    println!("Server listening on {:?}", socket_path);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(_) => return,
                };

                let msg = String::from_utf8_lossy(&buf[..n]);
                let num: u32 = match msg.trim().parse() {
                    Ok(num) if num >= 1 && num <= 10 => num,
                    _ => continue,
                };

                println!("Received request: {}", num);

                if let Err(_) = socket.write_all(format!("Received request: {}\n", num).as_bytes()).await {
                    return;
                }
            }
        });
    }
    drop(listener); // needed this to clear weird err
}

