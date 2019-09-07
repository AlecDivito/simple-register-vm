
/// Opcode are the possible commands can be used to tell the virtual machine to
/// do something
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// halt (stop) the currently running program
    HLT, 
    /// Illegal opcode, opcode could not be found
    IGL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
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