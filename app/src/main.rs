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
struct 
Log {
    source: Source,
    destination: Destination,
    compression_level: Option<u8>,
    key: Option<String>,
    tx_interval: Option<String>,
}

#[derive(Debug, Deserialize)]
struct 
Defaults {
    compression_level: u8,
    key: String,
    tx_interval: String,
}

#[derive(Debug, Deserialize)]
struct 
Config {
    logs: Vec<Log>,
    defaults: Defaults,
}

/* how do i avoid heep allocation with str? for now, using String
 * */
fn 
run_tail(log: Log) {

    let path = log.source.path.to_string();

    thread::spawn(move || {
        let mut tail_process = Command::new("tail")
            .args(&["-f", "-n0", "-q", &path]).stdout(Stdio::piped()).spawn()
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
start_watcher() -> Arc<Mutex<bool>> {

    let config : Config = read_config();
    let terminate_flag = Arc::new(Mutex::new(false));
    let terminate_flag_clone = Arc::clone(&terminate_flag);

    /* need to find a way to better control these threads. 
     * considering using park: https://doc.rust-lang.org/std/thread/fn.park.html
     * */
    for log in config.logs {
        let path = log.source.path.to_string();
        run_tail(log);
    }
    terminate_flag
}

fn 
main() {
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

    #[test]
    fn test_systemd_file() {
        // the unit file no longer needs to redirect output to output.log
        println!("test systemd file");
    }
}
