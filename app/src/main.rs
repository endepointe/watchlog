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

/* Use ipv4 for now. In the future, detect the type of address.
 */
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


// function that writes to an error log
fn
write_error_log(message: String) 
{
    let mut file = OpenOptions::new().append(true).open("error.log").unwrap();
    file.write_all(message.as_bytes()).unwrap();
}

fn
encrypt(buffer: &String) -> String 
{
    // open the key file and read the key
    let private_key = fs::read("private.pem").unwrap();
    println!("private key: {:?}", private_key);

    println!("encryptbuffer ... {:?}", buffer);
    let keypair = Rsa::generate(2048).unwrap();
    println!("keypair: {:?}", keypair);
    let keypair = PKey::from_rsa(keypair).unwrap();
    // convert a vector of strings to a single string
    println!("public key: {:?}", keypair);
    let mut encrypter = Encrypter::new(&keypair).unwrap();
    String::from("encrypted")
}

fn
compress(buffer: Vec<u8>, level: u8) -> Vec<u8> 
{
    let result: Vec<u8> = zstd::stream::encode_all(&buffer[..], 3).unwrap();
    result
}

fn 
transmit(buffer: Vec<String>) -> std::io::Result<()> 
{
    //let mut stream = TcpStream::connect("127.0.0.1:5001")?;
    // this header information contains the name of the directory to save the file to.
    // Example: if the transfer inverval is set to every hour, the directory name will be the 
    // current date and the child files will be saved in that directory.
    //stream.write_all(b"send over header information")?;
    //let mut buffer = [0; 1024];
    //stream.read(&mut buffer)?;
    //println!("Received: {}", String::from_utf8_lossy(&buffer));
    let (tx,rx) = std::sync::mpsc::sync_channel::<u8>(1);
    println!("{:?} {:?}", tx,rx);
    thread::sleep(Duration::from_secs(5));  
    tx.send(3).unwrap();
    println!("sent 3 to channel");

    thread::spawn( move || {
        tx.send(5).unwrap();
        println!("call encrypt for: {:?}", buffer);
        let buffer = buffer.join("");
        encrypt(&buffer);
    });
    println!("recvd: {}",rx.recv().unwrap());
    println!("recvd: {}",rx.recv().unwrap());
    println!("call encrypt");
    println!("call compress");
    println!("send");

    Ok(())
}

fn 
collector(log: Log) 
{
    let path = log.source.path.to_string();

    thread::spawn(move || {
        let mut tail_process = Command::new("tail")
            .args(&["-f", "-n0", "-q", &path]).stdout(Stdio::piped()).spawn()
            .expect("Failed to execute tail command");

        let tail_stdout = tail_process.stdout.take().unwrap();

        let reader = io::BufReader::new(tail_stdout);
        
        // read byte cap from config file.
        let cap = 16;
        let mut buffer: Vec<String> = Vec::with_capacity(cap);
        let mut size = 0;

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.len() + size < cap {
                    buffer.push(line.to_string());
                } else {
                    let b = buffer.to_vec();
                    transmit(b);                    
                    buffer.clear();
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
set_tx_buffer(string: Option<String>) -> u32  
{
    let mut bsize = 0;
    let val = string.unwrap_or("stream".to_string());

    match val.as_str() {
        "1kb" => bsize = 1024,
        "4kb" => bsize = 4096,
        "1mb" => bsize = 1024 * 1024,
        _ => bsize = 0, // stream the data
    }
    
    bsize
}

fn 
main() 
{
    let terminate_flag = watch_logs();
    io::stdout().flush().unwrap();  
    println!("{}",std::process::id());

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

    println!("Starting watch-log ...");

    while !*terminate_flag.lock().unwrap() {
        io::stdout().flush().unwrap();
        if sigquit.load(Ordering::Relaxed) {
            println!("SIGQUIT signal received.");
            *terminate_flag.lock().unwrap() = true;
        }
        if sigusr1.load(Ordering::Relaxed) {
            println!("SIGUSR1 signal received ... Continue running.");
        }
        if sigusr2.load(Ordering::Relaxed) {
            println!("SIGUSR2 signal received ... Continue running.");
        }
        thread::sleep(Duration::from_secs(1));
    }

    // terminate task gracefully
    thread::sleep(Duration::from_secs(1));
    io::stdout().flush().unwrap();
    println!("Main function terminated.");
}

// Test the memory consumption of the running program over a 1 minute period.
mod tests {
    #[test]
    fn test_memory_consumption() {
        println!("tests memory consumption");
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
        // the unit file no longer needs to redirect output to output.log
        println!("test systemd file");
    }

    #[test]
    fn test_start_interval() {
        use crate::set_tx_buffer;
        //let stop_interval = start_interval(Some("1m".to_string()));
        let stop_buffer = set_tx_buffer(None);
        println!("{:?}", stop_buffer);
    }

    #[test]
    fn test_encrypt() {
        use crate::encrypt;
        let buffer = "hello world".to_string();
        //let enc = encrypt(&buffer);
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
}
