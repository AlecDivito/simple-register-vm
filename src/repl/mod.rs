use std::io::Write;
use super::vm::VM;

/// Read, Evaluate, and Print Loop
/// Core structure for the REPL for the Assembler
pub struct REPL {
    /// Store all the commands the user inputs into the REPL
    command_buffer: Vec<String>,
    /// The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {

    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new()
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to simple VM lang!");
        loop {
            // This allocates a new String in which to store whatever the user
            // types each iteration.
            // TODO: Figure out how create this outside of the loop and re-use
            // it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = std::io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like
            // `println!` does, so we have to do that there for the user to see
            // our `>>> ` prompt.
            print!(">>> ");
            std::io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            
            // push command onto command stack
            self.command_buffer.push(buffer.to_string());
            
            // check if command exists
            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input")
                }
            }
        }
    }
}