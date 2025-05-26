use anyhow::Result;
use std::env;
use std::io::{self, BufRead};
use std::path::PathBuf;

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

fn main() {
    println!("Hello from pep-os!");

    eprint!("{} ", generate_prompt());

    for line in io::stdin().lock().lines() {
        match line {
            Ok(input) => {
                println!("You wrote: '{}'", input);
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        // https://unix.stackexchange.com/questions/380012/why-does-bash-interactive-shell-by-default-write-its-prompt-and-echoes-its-inter
        eprint!("{} ", generate_prompt())
    }
}
