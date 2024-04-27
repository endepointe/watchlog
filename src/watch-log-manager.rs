use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

fn get_pid() -> io::Result<u32> {
    let output = Command::new("ps")
        .arg("-C")
        .arg("watch-log")
        .arg("-o")
        .arg("pid=")
        .output()?;
    let pid = String::from_utf8_lossy(&output.stdout).trim().parse().unwrap();
    Ok(pid)
}

fn main() -> io::Result<()> {
    loop {
        println!("1. send sigquit to watch-log");
        println!("2. send sigusr1 to watch-log");
        println!("3. send sigusr2 to watch-log");
        println!("4. exit");

        print!("Enter your choice: ");
        io::stdout().flush()?;
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                let pid = get_pid()?;
                let output = Command::new("kill")
                    .arg("-SIGQUIT")
                    .arg(pid.to_string())
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "2" => {
                let pid = get_pid()?;
                let output = Command::new("kill")
                    .arg("-SIGUSR1")
                    .arg(pid.to_string())
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "3" => {
                let pid = get_pid()?;
                let output = Command::new("kill")
                    .arg("-SIGUSR2")
                    .arg(pid.to_string())
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "4" => break,
            _ => println!("Invalid input"),
        }
    }
    Ok(())
}
