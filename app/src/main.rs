#![allow(unused_imports)]
use std::io::prelude::*;
use std::io::{self, Read,BufRead,BufReader,Write};
use std::fs::{self,File, OpenOptions};
use std::net::{TcpListener, TcpStream,Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct 
Source {
    name: String,
    path: String 
}

/* Use ipv4 for now. In the future, detect the type of address.
 */
#[derive(Debug,Deserialize)]
struct 
Destination {
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
struct 
Defaults {
    compression_level: u8,
    key: PathBuf,
    tx_interval: String,
}

#[derive(Debug, Deserialize)]
struct 
Config {
    logs: Vec<Log>,
    defaults: Defaults,
}

fn 
run_tail(path: String, terminate_flag: Arc<Mutex<bool>>) -> Arc<Mutex<bool>> {

    thread::spawn(move || {
        let mut tail_process = Command::new("tail")
            .args(&["-f", "-n0", "-q", &path.as_str()]).stdout(Stdio::piped()).spawn()
            .expect("Failed to execute tail command");

        let tail_stdout = tail_process.stdout.take().unwrap();

        let reader = io::BufReader::new(tail_stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                // send to encryption
                println!("{}", line);
            } else {
                // write to error log file perhaps
                println!("Error reading line");
            }
        }
        let _ = tail_process.wait();
    });

    thread::park();   
    loop {
        thread::sleep(Duration::from_secs(1));
        if *terminate_flag.lock().unwrap() {
            return terminate_flag;
        }
    }        
}

fn 
read_config() -> Config {
    let mut file = std::fs::File::open("config.json").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let config : Config = serde_json::from_str(&buffer).unwrap();

    config
}

fn 
start_watcher(logs: Vec<Log>) -> Arc<Mutex<bool>> {
    let terminate_flag = Arc::new(Mutex::new(false));
    let terminate_flag_clone = terminate_flag.clone();

    for log in logs {
        let arc_log = Arc::new(log);
        let arc_log_clone = arc_log.clone();
        let source = &arc_log_clone.source;
        //let destination = log.destination;
        //let compression_level = log.compression_level.unwrap_or(0);
        //let key = log.key.unwrap_or(PathBuf::new());
        //let tx_interval = log.tx_interval.unwrap_or(String::from("0"));

        //println!("{:?} {:?} {:?} {:?} {:?}", source, destination, compression_level, key, tx_interval);
        thread::spawn(move || {
            run_tail(source.path.clone(), terminate_flag_clone.clone());
        });
        //run_tail(source.path.clone(), terminate_flag_clone.clone());
    }

    terminate_flag
}

fn 
main() {
    let config : Config = read_config();
    let terminate_flag = start_watcher(config.logs);
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
    fn test_start_watcher() {
        use crate::{Config, read_config,start_watcher};
        let config : Config = read_config();
        println!("{:?}", config);
        let terminate_flag = start_watcher(config.logs);
        println!("{:?}", terminate_flag);
    }
}
