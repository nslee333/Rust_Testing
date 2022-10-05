use std::env;
use std::env::args;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Child, Stdio};

fn main() {
   loop {
    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }

                previous_command = None;
            },
            "exit" = return,
            command => {
                
            }
        }
    }
   }
}
