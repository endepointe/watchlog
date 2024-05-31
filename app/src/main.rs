#![allow(unused_imports)]
use std::io::prelude::*;
use std::io::{self, Read,Write, BufRead,BufReader};
use std::fs::{self,File, OpenOptions};
use std::net::{TcpListener, TcpStream,Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::process::{Command, Stdio};
use serde::Deserialize;
use serde::Serialize;

use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;

#[derive(Debug,Deserialize)]
struct 
Source 
{
    name: String,
    path: String
}

/* Use ipv4 for now. In the future, detect the type of address. */
#[derive(Debug,Deserialize)]
struct 
Destination 
{
    address: Ipv4Addr,
    port: u16
}

#[derive(Debug,Deserialize)]
struct 
Log 
{
    source: Source,
    destination: Destination,
    compression_level: Option<u8>,
    key: Option<String>,
    tx_buffer: Option<String>,
}

trait LogHandler {
    fn get_tx_buffer(&self) -> usize;
    fn get_source_path(&self) -> String;
    fn get_destination_address(&self) -> Ipv4Addr;
}
impl LogHandler for Log {
    fn get_tx_buffer(&self) -> usize {
        let mut bsize = 0;
        let val = self.tx_buffer.clone().unwrap_or("stream".to_string());
        match val.as_str() {
            "1kb" => bsize = 1024,
            "4kb" => bsize = 4096,
            "1mb" => bsize = 1024 * 1024,
            _ => bsize = 0, // stream the data
        }
        bsize
    }
    fn get_source_path(&self) -> String {
        self.source.path.to_string()
    }
    fn get_destination_address(&self) -> Ipv4Addr {
        self.destination.address
    } 
}

// fallback values
#[derive(Debug, Deserialize)]
struct 
Defaults 
{
    compression_level: u8,
    key: String,
    tx_buffer: String,
}

#[derive(Debug, Deserialize)]
struct 
Config 
{
    logs: Vec<Log>,
    defaults: Defaults,
}

#[derive(Debug, Serialize)]
struct
Header
{
    name: String,
    date: String,
}

fn 
dbg_print(value: String, file: &str, line: u32) 
{
    println!("{}:{}: {}", file, line, value);
}

fn
add_header(name: &String) -> String 
{
    let n = Path::new(name).file_name().unwrap().to_str().unwrap();
    let header = Header {
        name: String::from(n),
        date: format_date(),
    };
    let header = serde_json::to_string(&header).unwrap();
    header
}

fn
format_date() -> String 
{
    let date = chrono::Local::now();
    let date = date.format("%Y-%m-%d-%H-%M");
    date.to_string()
}

fn
write_error_log(message: String) 
{
    let message = format!("{}: {}\n", chrono::Local::now().to_string(), message);
    let mut file = OpenOptions::new().append(true).create(true).open("error.log").unwrap();
    file.write_all(message.as_bytes()).unwrap();
}

fn
write_status_log(message: String) 
{
    let message = format!("{}: {}\n", chrono::Local::now().to_string(), message);
    let mut file = OpenOptions::new().append(true).create(true).open("status.log").unwrap();
    file.write_all(message.as_bytes()).unwrap();
}

fn
encrypt(buffer: String) -> Vec<u8>  
{
    let private_key = fs::read("private.pem").unwrap();
    let public_key = fs::read("public.pem").unwrap();

    let pkey = Rsa::private_key_from_pem(&private_key).unwrap();
    let key = Rsa::public_key_from_pem(&public_key).unwrap();

    let mut buf = vec![0; key.size() as usize];
    let enc_len = key.public_encrypt(&buffer.as_bytes(), &mut buf, Padding::PKCS1);
    
    match enc_len {
        Ok(v) => {
            return buf;
        },
        Err(e) => {
            write_error_log(e.to_string());
            return vec![0];
        }
    }
}

fn
compress(buffer: Vec<u8>, level: u8) -> Vec<u8> 
{
    let result: Vec<u8> = zstd::stream::encode_all(&buffer[..], 3).unwrap();
    result
}

fn
send(buffer: Vec<u8>) 
{
    let mut stream = TcpStream::connect("127.0.0.1:5001").unwrap();
    stream.write_all(&buffer).unwrap();
}

fn 
transmit(buffer: Vec<String>) -> std::io::Result<()> 
{
    let (tx,rx) = std::sync::mpsc::sync_channel::<Vec<u8>>(1);
    let bufferheader = buffer[0].as_bytes();
    let buffer = buffer[1..].to_vec();

    thread::spawn( move || {
        dbg_print(buffer.join(","), file!(), line!());
        let enc = encrypt(buffer.join(","));
        tx.send(enc).unwrap();
    });
    match rx.recv() {
        Ok(v) => {
            let msg: Vec<u8> = compress(v, 3);
            let mut msg = [bufferheader, &msg].concat();
            println!("{:?}", msg.len());
            println!("----\nmsg{:?}", std::str::from_utf8(&msg[..bufferheader.len()]).unwrap().to_string());
            send(msg);
        },
        Err(e) => {
            write_error_log(e.to_string());
        }
    }

    Ok(())
}

fn 
collector(log: Log) 
{

    let path = log.get_source_path();

    thread::spawn(move || {
        let mut tail_process = Command::new("tail")
            .args(&["-f", "-n0", "-q", &path]).stdout(Stdio::piped()).spawn()
            .expect("Failed to execute tail command");

        let tail_stdout = tail_process.stdout.take().unwrap();

        let reader = io::BufReader::new(tail_stdout);
        
        // read byte cap from config file.
        let cap = log.get_tx_buffer();//256;
        let mut buffer: Vec<String> = Vec::with_capacity(cap);
        let mut size = 0;
        let header = add_header(&path);
        buffer.push(header);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.len() + size < cap {
                    buffer.push(line.to_string());
                } else {
                    let b = buffer.to_vec();
                    transmit(b);                    
                    buffer.clear();
                    let header = add_header(&path);
                    buffer.push(header);
                    buffer.push(line.to_string());
                    size = 0;
                } 
                size += line.len();
            } else {
                write_error_log(line.unwrap());
            }
        }
        let _ = tail_process.wait();
    });
}

fn 
read_config() -> Config 
{
    let mut file = std::fs::File::open("config.json").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let config : Config = serde_json::from_str(&buffer).unwrap();

    config
}

fn 
watch_logs() -> Arc<Mutex<bool>> 
{
    let config : Config = read_config();
    let terminate_flag = Arc::new(Mutex::new(false));
    let terminate_flag_clone = Arc::clone(&terminate_flag);

    for log in config.logs {
        let path = log.source.path.to_string();
        collector(log);
    }
    terminate_flag
}


fn 
unix_app() 
{
    let terminate_flag = watch_logs();
    io::stdout().flush().unwrap();  

    /* Register signal handlers 
     * Overall, the watch-log binary will be controlled by systemd. These signals become useful
     * to restart the systemd watch-log service when a change is detected or errors occur. 
     *
     * For now, the sigquit should graceflly shutdown. That means communicating with the client
     * that sends the logs to the server then mark and preserve any logs that have yet to be sent.
     * */
    let sigquit = Arc::new(AtomicBool::new(false));
    let sigusr1 = Arc::new(AtomicBool::new(false));
    let sigusr2 = Arc::new(AtomicBool::new(false));

    signal_hook::flag::register(signal_hook::consts::SIGQUIT, Arc::clone(&sigquit));
    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&sigusr1));
    signal_hook::flag::register(signal_hook::consts::SIGUSR2, Arc::clone(&sigusr2));

    write_status_log("Starting watch-log ...".to_string());

    while !*terminate_flag.lock().unwrap() {
        io::stdout().flush().unwrap();
        if sigquit.load(Ordering::Relaxed) {
            write_status_log("SIGQUIT signal received.".to_string());
            *terminate_flag.lock().unwrap() = true;
        }
        if sigusr1.load(Ordering::Relaxed) {
            write_status_log("SIGUSR1 signal received ... Continue running.".to_string());
        }
        if sigusr2.load(Ordering::Relaxed) {
            write_status_log("SIGUSR2 signal received ... Continue running.".to_string());
        }
        thread::sleep(Duration::from_secs(1));
    }

    // terminate task gracefully
    thread::sleep(Duration::from_secs(1));
    io::stdout().flush().unwrap();
    write_status_log("Main function terminated.".to_string());
}


fn 
main() 
{
    if  cfg!(unix) {
        unix_app();
    } else {
        println!("Windows not yet supported.");
    }
}

mod tests {
    #[test]
    fn test_memory_consumption() {
        println!("tests memory consumption over a period of time.");
    }

    #[test]
    fn test_read_config() {
        use crate::{Config, read_config};
        let config : Config = read_config();
        println!("{:?}", config);
    }

    #[test]
    fn test_watch_logs() {
        use crate::{Config, read_config,watch_logs};
        let config : Config = read_config();
        println!("{:?}", config);
        let terminate_flag = watch_logs();
        println!("{:?}", terminate_flag);
    }

    #[test]
    fn test_systemd_file() {
        println!("test systemd file");
    }

    #[test]
    fn test_encrypt() {
        use crate::encrypt;
        let buffer = "Hello World".to_string();
        let result = encrypt(buffer);
        println!("{:?}", result);
    }

    #[test]
    fn test_compress() {
        use crate::compress;
        use std::io::Read;
        let size = 4096; // size of compression test file.
        let mut file = std::fs::File::open("compression.test").unwrap();
        let mut buffer = vec![0; size];
        let orginal_size = buffer.len();
        file.read(&mut buffer).unwrap();
        let result: Vec<u8> = compress(buffer, 3);
        println!("test general compression");
        assert_eq!(result.len() < orginal_size, true); 
        println!("general compression passed");
        println!("test 50% compression");
        assert_eq!(result.len() < orginal_size / 2, true);
        println!("50% compression passed");
        println!("test 40% compression");
        assert_eq!(result.len() < (5 * orginal_size) / 2, true);
        println!("40% compression passed");
        println!("test 30% compression");
        assert_eq!(result.len() < orginal_size / 3, true);
        println!("30% compression passed");
    }

    #[test]
    fn test_log_file_exists() {
        println!("write test to check if log file exists.");
        println!("If it does not, put a wait flag for the file to exist.");
    }

    #[test]
    fn test_decryption() {
        use openssl::encrypt::Decrypter; 
        println!("test decryption of the existing encrypted file: storage.test.");
        let private_key = std::fs::read("private.pem").unwrap();
        let file = std::fs::read("storage.test").unwrap();
        let pkey = openssl::rsa::Rsa::private_key_from_pem(&private_key).unwrap();
        // decompress the file using zstd
        let decompressed = zstd::stream::decode_all(&file[..]).unwrap();
        // decrupt the decompressed data
        let mut buf = vec![0; pkey.size() as usize];
        let dec_len = pkey.private_decrypt(&decompressed, &mut buf, openssl::rsa::Padding::PKCS1).unwrap();
        println!("{:?}", dec_len);
        println!("{:?}", buf);
        // convert the decrypted buffer to a string
        let result = std::str::from_utf8(&buf[..dec_len]).unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn test_add_header() {
        use crate::add_header;
        use serde::Serialize;
        let result = add_header(&"path/to/test.log".to_string());
        println!("{:?}", result);
    }

    #[test]
    fn test_get_tx_buffer() {
        use crate::LogHandler;
        use crate::Log;
        use crate::Source;
        use crate::Destination;
        let log = Log {
            source: Source {
                name: "test".to_string(),
                path: "path/to/test.log".to_string(),
            },
            destination: Destination {
                address: std::net::Ipv4Addr::new(127,0,0,1),
                port: 5001,
            },
            compression_level: Some(3),
            key: Some("key".to_string()),
            tx_buffer: Some("1kb".to_string()),
        };
        let result = log.get_tx_buffer();
        println!("{:?}", result);
    }
}

