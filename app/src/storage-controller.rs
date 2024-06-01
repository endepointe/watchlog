mod types;
use crate::types::{Header};

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader};
use std::fs::File;
use std::thread;
use serde::{Deserialize};

fn
write_log_source(stream: &TcpStream)
{
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .append(true).create(true).open("sources.log").unwrap();
    let addr = stream.peer_addr().unwrap();
    writeln!(file, "{}", addr).unwrap();
}

fn
route_data(header: &Header, data: &[u8])
{
    // logs/name/date/
    let date_dir = format!("logs/{}/{}/", header.name, &header.date[..header.date.len()- 6]);

    if let Err(e) = std::fs::metadata(&date_dir) {
        std::fs::create_dir_all(&date_dir).unwrap();
    }

    // logs/name/date/hr-hr
    let date = header.date.split("-").collect::<Vec<&str>>();
    let curr_hour = date[date.len() - 2].parse::<i32>().unwrap();
    let next_hour = date[date.len() - 2].parse::<i32>().unwrap() + 1;
    let f = format!("{}-{}", curr_hour.to_string(), next_hour.to_string());

    let full_path = format!("{}{}", date_dir, f);
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .append(true).create(true).open(&full_path).unwrap();

    if let Err(e) = std::fs::metadata(&full_path) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }

    writeln!(file, "{}", header.date).unwrap();
    writeln!(file, "{:?}\n", std::str::from_utf8(&data).unwrap()).unwrap();
}

fn 
handle_client(mut stream: TcpStream) 
{
    write_log_source(&stream);

    let buf_reader = BufReader::new(&mut stream);
    let data: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if data.len() > 0 {
        let mut header = String::new();
        let mut i: usize = 1;
        for byte in data[0].as_bytes() {
            if *byte == 0x7D {
                i += 1;
                header.push(*byte as char);
                break;
            }
            i += 1;
            header.push(*byte as char);
        }

        let header: Header = serde_json::from_str(&header).unwrap();

        route_data(&header, &data[0][i..].as_bytes());
    }
}

fn 
main() -> std::io::Result<()> 
{
    if let Err(e) = std::fs::metadata("logs") {
        std::fs::create_dir("logs").unwrap();
    }

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
