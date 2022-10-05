use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Child, Stdio};

fn main() {
   loop { 
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        Command::new(command)
            .spawn()
            .unwrap();

        child.wait();
    }
}
