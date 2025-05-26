use std::io::{self, BufRead};

fn generate_prompt() -> String {
    String::from(">")
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
