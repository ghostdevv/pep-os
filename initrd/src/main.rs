use anyhow::{Context, Result};
use shellwords;
use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn generate_prompt_path() -> Result<String> {
    let current_dir = env::current_dir()?;

    if current_dir == PathBuf::from("/") {
        return Ok(String::from("/"));
    }

    let home_dir = env::var("HOME").ok().map(PathBuf::from);

    if let Some(home) = home_dir {
        if home == current_dir {
            return Ok(String::from("~"));
        }
    }

    Ok(current_dir
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .to_string())
}

fn generate_prompt() -> String {
    let prompt_path = generate_prompt_path().unwrap_or(String::from("?"));
    format!("{} >", prompt_path)
}

fn execute_command(command_line: &str) -> Result<()> {
    // Parse the command line using shellwords
    let args = shellwords::split(command_line).context("Failed to parse command line")?;

    if args.is_empty() {
        return Ok(());
    }

    let program = &args[0];
    let args = &args[1..];

    // Create a new Command instance
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Execute the command and wait for it to complete
    let status = cmd
        .status()
        .context(format!("Failed to execute command: {}", program))?;

    if !status.success() {
        if let Some(code) = status.code() {
            eprintln!("Command exited with code: {}", code);
        } else {
            eprintln!("Command terminated by signal");
        }
    }

    Ok(())
}

fn main() {
    println!("Hello from pep-os!");

    eprint!("{} ", generate_prompt());
    io::stderr().flush().unwrap();

    for line in io::stdin().lock().lines() {
        match line {
            Ok(input) => {
                if !input.trim().is_empty() {
                    if let Err(e) = execute_command(&input) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        // https://unix.stackexchange.com/questions/380012/why-does-bash-interactive-shell-by-default-write-its-prompt-and-echoes-its-inter
        eprint!("{} ", generate_prompt());
        io::stderr().flush().unwrap();
    }
}
