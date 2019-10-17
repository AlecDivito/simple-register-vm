
/// Opcode are the possible commands can be used to tell the virtual machine to
/// do something
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// load a number into the specified register
    /// (ex. LOAD $0 #500, load 500 into register 0)
    LOAD,
    
    /// Add 2 registers together
    /// (ex. ADD $0 $1 $2, Add register 0 and 1, store result in register 3)
    ADD,
    /// Subtract 2 registers together
    /// (ex. SUB $0 $1 $2, Subtract register 0 and 1, store result in register 3)
    SUB,
    /// Multiply 2 registers together
    /// (ex. MUT $0 $1 $2, Multiply register 0 and 1, store result in register 3)
    MUL,
    /// Divide 2 registers together
    /// (ex. DIV $0 $1 $2, Divide register 0 and 1, store result in register 3)
    DIV,
    /// jump straight to a new instruction
    JMP,
    /// jump moves forward x amount of instructions relative to the program counter position
    /// (ex. JMP $0, the value inside of register 0 represents how many positions to move)
    JMPF,
    /// jump moves backwards x amount of instructions relative to the program counter position
    /// (ex. JMP $0, the value inside of register 0 represents how many positions to move)
    JMPB,
    /// halt (stop) the currently running program
    HLT,
    /// Illegal opcode, opcode could not be found
    IGL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            7 => return Opcode::JMP,
            8 => return Opcode::JMPF,
            9 => return Opcode::JMPB,
            _ => return Opcode::IGL
        }
    }
}

/// Instruction is the struct that will tell the virtual machine what to do
#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// Operation to execute
    opcode: Opcode
}

impl Instruction {
    /// Create a new Instruction
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}