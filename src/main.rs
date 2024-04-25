use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use std::fs::{File, OpenOptions};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug)]
struct Count {
    value: u32,
}
trait CountOps {
    fn new() -> Self;
    fn increment(&mut self);
    fn decrement(&mut self);
    fn get(&self) -> u32;
}
impl CountOps for Count {
    fn new() -> Self {
        Count {
            value: 0,
        }
    }

    fn increment(&mut self) {
        if self.value >= std::u32::MAX - 1 {
            self.value = 0;
        }
        self.value += 1;
    }

    fn decrement(&mut self) {
        if self.value >= 1 {
            self.value -= 1;
        }
    }
    
    fn get(&self) -> u32 {
        self.value
    }
}

#[derive(Debug)]
struct Database {
    local: Vec<u32>, 
}

trait DatabaseOps {
    fn new() -> Self;
    fn insert(&mut self, value: u32);
    fn delete(&mut self, value: u32);
    fn update(&mut self, value: u32);
    fn select(&self, value: u32) -> Option<u32>;
}

impl DatabaseOps for Database {
    fn new() -> Self {
        Database {
            local: Vec::new(),
        }
    }

    fn insert(&mut self, value: u32) {
        self.local.push(value);
    }

    fn delete(&mut self, value: u32) {
        self.local.retain(|&x| x != value);
    }

    fn update(&mut self, value: u32) {
        self.local.iter_mut().for_each(|x| *x = value);
    }

    fn select(&self, value: u32) -> Option<u32> {
        self.local.iter().find(|&&x| x == value).map(|&x| x)
    }
}

async fn listen_for_add_signal(func: Arc<Mutex<Database>>) {
    let (tx, mut rx) = mpsc::unbounded_channel::<u32>();

    tokio::spawn(async move {
        let mut sigusr2 = signal(SignalKind::user_defined2()).unwrap();

        while let Some(_) = sigusr2.recv().await {
            let mut db = func.lock().unwrap();
            // insert whatever value the counter is at.
            db.insert(33);
            println!("Database updated by signal: {:?}", db);
        }
    });
    rx.recv().await;
}

async fn listen_for_menu_signal(func: Arc<Mutex<Count>>) {
    let (tx, mut rx) = mpsc::unbounded_channel::<u32>();
    let mut sigusr1 = signal(SignalKind::user_defined1()).unwrap();

    tokio::spawn(async move {
    });
}

fn running_task(terminate_flag: Arc<Mutex<bool>>) {
    let arc_file = Arc::new(std::fs::File::options().append(true).create(true).open("newfile.txt").unwrap());
    
    loop {
        //println!("perform some work...");
        let mut arc_file_clone = Arc::clone(&arc_file);
        arc_file_clone.write_all(b"perform some work\n");

        thread::sleep(Duration::from_secs(1));

        if *terminate_flag.lock().unwrap() {
            //println!("Task terminated.");
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

    terminate_flag
}

#[tokio::main]
async fn main() {
    //let mut f = OpenOptions::new().append(true).open("input.log")?;
    //let echo_date = std::process::Command::new("date")
    //    .arg("+%Y-%m-%d %T:%N %Z")
    //    .output()
    //    .expect("Failed to execute command");
    //f.write_all(&echo_date.stdout)?;
    
    //let pid = std::process::id();
    //println!("Process ID: {}", pid);

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
    println!("Starting running task...");
    let terminate_flag = start_running_task();

    // user can press enter to terminate the task. This may be replaced by a signal
    // now that I think of it.
    println!("Press Enter to terminate the task...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    *terminate_flag.lock().unwrap() = true;

    // terminate task gracefully
    thread::sleep(Duration::from_secs(2));

    println!("Main function terminated.");

    // learning how signals work (rough and dirty)
    //let data = Arc::new(Mutex::new(Database::new()));
    //let count = Arc::new(Mutex::new(Count::new()));
    //let (tx, rx) = mpsc::unbounded_channel::<u32>();
    //let one = listen_for_add_signal(data.clone());
    //let two = listen_for_menu_signal(count.clone());
    //tokio::join!(one, two);
    
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
