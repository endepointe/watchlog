#![allow(unused_imports)]
use std::io::prelude::*;
use std::io::{self, BufRead,BufReader,Write};
use std::fs::{self,File, OpenOptions};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::process::{Command, Stdio};

struct App;

impl App {
    fn run_background() {
        let file_paths = vec![
            "log1.txt",
            "log2.txt",
            "log3.txt",
        ];
        let mut count = 0;

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
                count += 1;
            });
            println!("Running background task for file: {}", file_path);

        }

        // TODO: Move the running task functionality here.
        //if count == 2 {
        lop {
            println!("Loop running that listens for signals ...");
            thread::sleep(Duration::from_secs(1));
        }
        //}
        //println!("App is running in the background ...");
        // Wait for all threads to finish
        thread::park();
    }
}
fn running_task(terminate_flag: Arc<Mutex<bool>>) {
    let arc_file = Arc::new(std::fs::File::options().append(true).create(true).open("newfile.txt").unwrap());
    
    loop {
        let mut arc_file_clone = Arc::clone(&arc_file);
        arc_file_clone.write_all(b"perform some work\n");
        
        thread::sleep(Duration::from_secs(1));

        if *terminate_flag.lock().unwrap() {
            arc_file_clone.write_all(b"terminating task\n");
            return;
        }
    }
}

fn start_running_task() -> Arc<Mutex<bool>> {
    let terminate_flag = Arc::new(Mutex::new(false));
    let terminate_flag_clone = terminate_flag.clone();

    thread::spawn(move || {
        running_task(terminate_flag_clone);
    });

    App::run_background();

    terminate_flag
}

fn main() {

    // A list, could be a vector, collection, should maintain a list of log 
    // collection tasks. Each task should continue to run while in the 
    // background.
    //
    // src_list: [src_0, src_1, ..., src_n]
    // dst_list: [dst_0, dst_1, ..., dst_n]
    
    // There should be a sink/bucket/collection pool that recieves the output
    // and prepares the data to be sent to a long-term storage solution.
   
    // decide whether using a Rc or Arc will be suitable for controlling 
    // ownership of a log reading and collection stream.
    // | src_0 | ---> | dst_0 | 
    // | src_1 | ---> | dst_1 | 
    // | src_2 | ---> | dst_2 | 
    //

    let terminate_flag = start_running_task();
    println!("{}",std::process::id());
   
    let sigquit = Arc::new(AtomicBool::new(false));
    let sigusr1 = Arc::new(AtomicBool::new(false));
    let sigusr2 = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGQUIT, Arc::clone(&sigquit));
    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&sigusr1));
    signal_hook::flag::register(signal_hook::consts::SIGUSR2, Arc::clone(&sigusr2));

    loop {

        println!("Main function running ...");

        if sigquit.load(Ordering::Relaxed) {
            println!("SIGQUIT signal received.");
            break;
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
    *terminate_flag.lock().unwrap() = true;
    thread::sleep(Duration::from_secs(2));
    println!("Main function terminated.");
}

// Test the memory consumption of the running program over a 1 minute period.
mod tests {
    #[test]
    fn test_memory_consumption() {
        let mut child = Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to start the process");
        let output = child.stdout;
        println!("{:?}", output);
    }
}
