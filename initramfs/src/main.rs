use anyhow::{Result, bail};

fn set_env_var(key: &str, value: &str) -> Result<()> {
    unsafe {
        std::env::set_var(key, value);
    }

    match std::env::var(key) {
        Ok(val) if val == value => Ok(()),
        Ok(_) => bail!("Failed to set environment variable {}", key),
        Err(e) => bail!("Failed to set environment variable {}: Error: {}", key, e),
    }
}

fn spawn_shell() -> Result<std::process::ExitStatus, std::io::Error> {
    println!("Starting shell...");
    std::process::Command::new("/bin/rash").status()
}

fn main() {
    println!("┌────────────────────────────────────┐");
    println!("│        Welcome to pep-os!          │");
    println!("└────────────────────────────────────┘");

    // Initialize environment
    println!("Setting up environment variables...");
    set_env_var("PATH", "/bin").expect("path failed to be set");

    println!("Spawning shell for now...");
    match spawn_shell() {
        Ok(status) => {
            println!("Shell exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Unable to start rash :D Error: {}", e);
            std::process::exit(1);
        }
    }

    println!("you shouldn't see this... at least, I don't think so... uhhh... nice one!")
}
