use super::instructions::Opcode;

/// VM represents our CPU that emulates the functionality of a hardware CPU
#[derive(Debug)]
pub struct VM {
    /// registers available to our VM
    registers: [i32; 32],
    /// program counter that tracks what instruction we are currently pointed at
    pc: usize,
    /// Stack of instructions to execute
    program: Vec<u8>
}

impl VM {
    /// Create a new registered-based virtual machine
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
        }
    }
    
    /// Run the VM and start executing instructions
    /// This is meant to be the most performance-critical part of our language interpreter
    pub fn run(&mut self) {
        loop {
            // If our program counter has exceeded the length of the program itself,
            // something has gone awry
            if self.pc >= self.program.len() {
                println!("Program counter exceeded program instruction stack");
                break;
            }
            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered");
                    return;
                },
                _ => {
                    println!("Unrecognized opcode found! Terminating!");
                    return;
                }
            }
        }
    }

    /// decode opcode will Increment the program counter to the next instruction
    /// and return the resulting opcode
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}