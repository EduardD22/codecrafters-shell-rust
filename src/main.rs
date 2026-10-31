#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        let mut parts = command.split_whitespace();
        let program = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match program {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                let name = args[0];
                match name {
                    "exit" | "echo" | "type" => println!("{} is a shell builtin", name),
                    _ => println!("{}: not found", name),
                }
            },
            _ => println!("{}: command not found", command),
        }
    }
    
}
