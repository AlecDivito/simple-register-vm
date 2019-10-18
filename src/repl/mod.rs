use super::vm::VM;
use crate::assembler::program_parser::Program;
use std::io::Write;

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
            vm: VM::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to simple VM lang!");
        let mut buffer = String::new();
        loop {
            // Blocking call until the user types in a command
            let stdin = std::io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like
            // `println!` does, so we have to do that there for the user to see
            // our `>>> ` prompt.
            print!(">>> ");
            std::io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            buffer.clear();
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            // push command onto command stack
            self.command_buffer.push(buffer.to_string());

            // check if command exists
            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in self.vm.get_instructions() {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.get_registers());
                    println!("End of Register Listing");
                }
                _ => {
                    let program = match Program::parse(buffer) {
                        Ok(p) => p,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    let bytecode = program.to_bytes();
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 01 01 03 E8
    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}
