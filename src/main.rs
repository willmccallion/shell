use ansi_term::Colour;
use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    proccess_input("clear");
    loop {
        info();
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
            if let Some(dir_name) = path.file_name() {
                let git_branch = get_git_branch();

                let formatted_output;
                if git_branch == "no git" {
                    formatted_output = format!(
                        "{} {} {} ",
                        Colour::RGB(0, 100, 0).paint("➜"),
                        Colour::RGB(0, 255, 255).paint(dir_name.to_string_lossy()),
                        Colour::RGB(255, 140, 0).paint("✗")
                    )
                } else {
                    let start_italic = "\x1b[3m";
                    let end_italic = "\x1b[0m";

                    formatted_output = format!(
                        "{} {} {}{}{}{}{} {} ",
                        Colour::RGB(0, 100, 0).paint("➜"),
                        Colour::RGB(0, 255, 255).paint(dir_name.to_string_lossy()),
                        Colour::RGB(0, 128, 128).paint("git["),
                        start_italic,
                        Colour::RGB(255, 140, 140).paint(git_branch),
                        end_italic,
                        Colour::RGB(0, 128, 128).paint("]"),
                        Colour::RGB(255, 140, 0).paint("✗")
                    );
                }
                print!("{}", formatted_output);
            } else {
                println!("Error: Unable to get directory name.");
            }
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}

fn get_git_branch() -> String {
    match Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                branch
            } else {
                "no git".to_string()
            }
        }
        Err(_) => "no git".to_string(),
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
        println!("{}", e);
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
        match cmd.spawn() {
            Ok(_child) => {
                println!("Process started in background.");
            }
            Err(e) => {
                eprintln!("Failed to start process in background: {}", e);
            }
        }
    } else {
        match cmd.status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command failed with status: {}", status);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}
