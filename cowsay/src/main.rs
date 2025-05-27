use std::env;

fn main() {
    // Get message from command line arguments or use default
    let args: Vec<String> = env::args().collect();
    let message = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "pep-os!".to_string()
    };

    // Create the speech bubble
    let message_len = message.len();
    println!(" {}", "-".repeat(message_len + 2));
    println!("< {} >", message);
    println!(" {}", "-".repeat(message_len + 2));

    // Draw the cow
    println!("        \\   ^__^");
    println!("         \\  (oo)\\_______");
    println!("            (__)\\       )\\/\\");
    println!("                ||----w |");
    println!("                ||     ||");
}
