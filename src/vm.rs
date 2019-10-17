use super::instructions::Opcode;

/// VM represents our CPU that emulates the functionality of a hardware CPU
#[derive(Debug)]
pub struct VM {
    /// registers available to our VM
    registers: [i32; 32],
    /// program counter that tracks what instruction we are currently pointed at
    pc: usize,
    /// Stack of instructions to execute
    program: Vec<u8>,
    /// keep extra data that a instruction may output (division)
    remainder: u32,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
}

impl VM {
    /// Create a new registered-based virtual machine
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false
        }
    }
    
    /// Run the VM and start executing instructions
    pub fn run(&mut self) {
        let mut is_not_done = true;
        while is_not_done {
            is_not_done = self.execute_instruction();
        }
    }

    /// Execute only one instruction of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Execute the instruction that is currently pointed to on the program stack
    /// return true if command was executed successfully and program is not finished
    fn execute_instruction(&mut self) -> bool {
        // If our program counter has exceeded the length of the program itself,
        // something has gone awry
        if self.pc >= self.program.len() {
            println!("Program counter exceeded program instruction stack");
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
                true
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                true
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
                true
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                true
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                true
            },

            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
                true
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize;
                true
            },
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize;
                true
            },
            Opcode::EQ => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.equal_flag = lhs == rhs;
                self.next_8_bits(); // eat bits
                true
            },
            Opcode::NEQ => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.equal_flag = lhs != rhs;
                self.next_8_bits(); // eat bits
                true
            },
            Opcode::GT => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.equal_flag = lhs > rhs;
                self.next_8_bits(); // eat bits
                true
            },
            Opcode::LT => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.equal_flag = lhs < rhs;
                self.next_8_bits(); // eat bits
                true
            },
            Opcode::GTEQ => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.equal_flag = lhs >= rhs;
                self.next_8_bits(); // eat bits
                true
            },
            Opcode::LTEQ => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.equal_flag = lhs <= rhs;
                self.next_8_bits(); // eat bits
                true
            },
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
                true
            },
            Opcode::JNEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
                true
            }
            Opcode::HLT => {
                println!("HLT encountered");
                false
            },
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                false
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

    /// Get the next byte off of the program stack
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    /// Get the next 2 bytes off of the program stack
    fn next_16_bits(&mut self) -> u16 {
        // read off 2 bytes from the stack, move the first byte up 8 bits
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        VM::new()
    }

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

    #[test]
    fn test_load_opcode() {
        let mut test_vm = get_test_vm();
        // 1, 244 is how to represent 500 using 2 u8s in little endian format
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 3;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 3);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.pc = 4;
        test_vm.registers[0] = 3;
        test_vm.program = vec![6, 0, 0, 0, 9, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 3);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 12;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 12;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 12;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gteq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 12;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lteq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![15, 0, 1, 0, 15, 0, 1, 0, 15, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 12;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![16, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = false;
        test_vm.program = vec![17, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}