
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
    /// Equal, EQ $0 $1
    /// Check if two values are equal
    EQ,
    /// Not Equal, NEQ $0 $1
    /// Check if two values are not equal
    NEQ,
    /// Greater Then, GT $0 $1
    /// Check if the left side is greater then the right side
    GT,
    /// Less Then, LT $0 $1
    /// Check if the left side is less then the right side
    LT,
    /// Greater Then OR Equal To, GTEQ $0 $1
    /// Check if the left side is greater then or equal to the right side
    GTEQ,
    /// Less Then OR Equal To, LTEQ $0 $1
    /// Check if the left side is less then or equal to the right side
    LTEQ,
    /// Jump If Equal, JEQ $0
    /// If the equal flag is true, jump to instruction
    JEQ,
    /// Jump If Not Equal, JNEQ $0
    /// If the equal flag is false, jump to instruction
    JNEQ,
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
            10 => return Opcode::EQ,
            11 => return Opcode::NEQ,
            12 => return Opcode::GT,
            13 => return Opcode::LT,
            14 => return Opcode::GTEQ,
            15 => return Opcode::LTEQ,
            16 => return Opcode::JEQ,
            17 => return Opcode::JNEQ,
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