mod types;
use crate::types::{Header};

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::thread;
use serde::{Deserialize};

fn
route_data(header: &Header, data: &[u8])
{
    // logs/name/date/
    let date_dir = format!("logs/{}/{}/", header.name, &header.date[..header.date.len()- 6]);

    if let Err(e) = std::fs::metadata(&date_dir) {
        std::fs::create_dir_all(&date_dir).unwrap();
    }

    // name/date/hr-hr
    let date = header.date.split("-").collect::<Vec<&str>>();
    let curr_hour = date[date.len() - 2].parse::<i32>().unwrap();
    let next_hour = date[date.len() - 2].parse::<i32>().unwrap() + 1;
    let f = format!("{}-{}", curr_hour.to_string(), next_hour.to_string());

    let full_path = format!("{}{}", date_dir, f);
    let mut f = std::fs::OpenOptions::new().append(true).create(true).open(&full_path).unwrap();

    if let Err(e) = std::fs::metadata(&full_path) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }

    // append the data to this file
    println!("{:?}", &full_path);
    f.write(data).unwrap();
}

fn 
handle_client(mut stream: TcpStream) 
{
    println!("Incoming data from: {:?}", stream);
    //let mut file = std::fs::OpenOptions::new().append(true).create(true).open("storage.test").unwrap();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        println!("Buffer: {:?}", &buffer[..bytes_read]);

        let mut header = String::new();
        for byte in &buffer[..bytes_read] {
            if *byte == 0x7D {
                header.push(*byte as char);
                break;
            }
            header.push(*byte as char);
        }
        let header: Header = serde_json::from_str(&header).unwrap();

        route_data(&header, &buffer[..]);

        //file.write(&buffer[..bytes_read]).unwrap();
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
