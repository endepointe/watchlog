use std::io;
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncReadExt};
use tokio::net::UnixListener;
use tokio::process::Command;
use tokio::sync::mpsc;

async fn consumer_function(mut receiver: mpsc::Receiver<String>, db_path: PathBuf) {
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&db_path)
        .await
        .expect("Error opening database file");

    while let Some(new_data) = receiver.recv().await {
        if let Err(err) = file.write_all(new_data.as_bytes()).await {
            eprintln!("Error writing to database file: {}", err);
        }
    }
}

async fn start_collection(path: PathBuf, db_path: PathBuf) -> tokio::io::Result<()> {
    let (tx, rx) = mpsc::channel::<String>(100);

    let consumer_handle = tokio::spawn(consumer_function(rx, db_path));

    let mut child = Command::new("tail")
        .args(&["-f", "-n0", "-q", path.to_str().unwrap()])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn tail process");

    let stdout = child.stdout.take().expect("Failed to get stdout");

    let mut reader = tokio::io::BufReader::new(stdout);
    let mut line = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line).await {
        if bytes_read == 0 {
            break;
        }
        tx.send(line.clone()).await.expect("Error sending data to consumer");
        line.clear();
    }

    consumer_handle.await.expect("Consumer panicked");

    Ok(())
}

async fn handle_client(mut sockstream: tokio::net::UnixStream, db_path: PathBuf) -> io::Result<()> {
    let mut buf = [0; 1024];
    let n = sockstream.read(&mut buf).await?;
    let msg = String::from_utf8_lossy(&buf[..n]);
    println!("Received message: {}", msg);

    match msg.trim() {
        "start_logging" => {
            println!("Starting logging...");
            let log_path = Path::new("/var/log/logminer/crontest.log"); 
            start_collection(log_path.to_path_buf(), db_path).await?;
            println!("Logging started");
            sockstream.write_all(b"Logging started").await?;
        }
        "stop_logging" => {
            sockstream.write_all(b"Logging stopped").await?;
        }
        "terminate_server" => {
            println!("Received termination signal. Shutting down...");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Server terminated by client",
            ));
        }
        _ => {
            sockstream.write_all(b"Invalid command").await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket_path = "./socket.sock";
    let db_path = Path::new("storage.db").to_path_buf();

    let listener = UnixListener::bind(socket_path)?;

    println!("Server listening on {:?}", socket_path);

    loop {
        let (mut sockstream, _sockaddr) = listener.accept().await?;

        println!("Accepted connection from {:?}", _sockaddr);

        let db_path_clone = db_path.clone();
        tokio::spawn(async move {
            let handler = handle_client(sockstream, db_path_clone);
            match handler.await {
                Ok(client) => {
                    println!("{:?} handled successfully", client);
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::Other {
                        if let Err(err) = std::fs::remove_file(Path::new(socket_path)) {
                            eprintln!("Error removing socket file: {:?}", err);
                        }
                    }
                    eprintln!("Error handling client: {:?}", err);
                }
            }
        });
    }
}

