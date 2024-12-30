use ansi_term::Colour;
use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        info();
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        if input.trim() == "exit" {
            break;
        }
        proccess_input(input.trim());
    }
}

fn info() {
    match env::current_dir() {
        Ok(path) => {
            // Extract the last component of the path (i.e., the current directory name)
            if let Some(dir_name) = path.file_name() {
                println!(
                    "{}",
                    Colour::RGB(255, 140, 140).paint(dir_name.to_string_lossy())
                );
            } else {
                println!("Error: Unable to get directory name.");
            }
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}

fn proccess_input(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let command = parts[0];
    let args = &parts[1..];

    match command {
        "cd" => change_directory(args),
        "pwd" => print_working_directory(),
        _ => execute_command(command, args),
    }
}

fn change_directory(args: &[&str]) {
    if args.is_empty() {
        println!("cd: missing argument");
        return;
    }

    if let Err(e) = env::set_current_dir(args[0]) {
        println!("cd: {}: {}", args[0], e);
    }
}

fn print_working_directory() {
    if let Ok(path) = env::current_dir() {
        println!("{}", path.display());
    }
}

fn execute_command(command: &str, args: &[&str]) {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if args.last() == Some(&"&") {
        cmd.spawn().expect("Failed to start process in background");
    } else {
        let status = cmd.status().expect("Failed to execute command");
        if !status.success() {
            eprintln!("Command failed with status: {}", status);
        }
    }
}
