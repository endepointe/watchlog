#![allow(unused_imports)]
use std::io::prelude::*;
use std::io::{self, Read,BufRead,BufReader,Write};
use std::fs::{self,File, OpenOptions};
use std::net::{TcpListener, TcpStream,Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct Source {
    name: String,
    path: PathBuf
}

/* Use ipv4 for now. In the future, detect the type of address.
 */
#[derive(Debug,Deserialize)]
struct Destination {
    address: Ipv4Addr,
    port: u16
}

#[derive(Debug,Deserialize)]
struct Log {
    source: Source,
    destination: Destination,
    compression_level: Option<u8>,
    key: Option<PathBuf>,
    tx_interval: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Defaults {
    compression_level: Option<u8>,
    key: Option<PathBuf>,
    tx_interval: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    logs: Vec<Log>,
    defaults: Defaults,
}

struct App;

impl App {
    fn run_background(terminate_flag: Arc<Mutex<bool>>) -> Arc<Mutex<bool>>{
        let file_paths = vec![
            "log1.txt",
            "log2.txt",
            "log3.txt",
        ];

        for file_path in file_paths {
            let file_path_clone = file_path.to_string();
            thread::spawn(move || {
                let mut tail_process = Command::new("tail")
                    .args(&["-f", "-n0", "-q", &file_path_clone]).stdout(Stdio::piped()).spawn()
                    .expect("Failed to execute tail command");

                let tail_stdout = tail_process.stdout.take().unwrap();

                let mut save_file = OpenOptions::new()
                    .create(true).append(true).open(format!("{}.save", &file_path_clone))
                    .expect("Failed to open or create .save file");

                let reader = io::BufReader::new(tail_stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Err(e) = writeln!(save_file, "{}", line) {
                            eprintln!("Failed to write to .save file: {}", e);
                        }
                    }
                }
                let _ = tail_process.wait();
            });

        }
        thread::park();   
        //terminate_flag
        loop {
            thread::sleep(Duration::from_secs(1));
            if *terminate_flag.lock().unwrap() {
                return terminate_flag;
            }
        }        

    }
}

fn start_watcher() -> Arc<Mutex<bool>> {
    let terminate_flag = Arc::new(Mutex::new(false));
    let terminate_flag_clone = terminate_flag.clone();

    thread::spawn(move || {
        App::run_background(terminate_flag_clone);
    });

    terminate_flag
}

fn read_config() {
    let mut file = std::fs::File::open("config.json").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let config : Config = serde_json::from_str(&buffer).unwrap();
    println!("{:?}",config);
}

fn main() {

    let terminate_flag = start_watcher();
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
    fn test_config() {
        use crate::read_config;
        read_config();
    }
}
