use crate::assembler::instruction_parser::AssemblerInstruction;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn parse(commands: &str) -> Result<Program, &'static str> {
        let instructions = commands
            .split('\n')
            .filter(|c| c.trim() != "")
            .map(|c| AssemblerInstruction::parse_one_line(c).unwrap())
            .collect::<Vec<AssemblerInstruction>>();
        Ok(Program { instructions })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }

    pub fn get_instructions(&self) -> &Vec<AssemblerInstruction> {
        &self.instructions
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Opcode;

    #[test]
    fn test_parse_program() {
        let result = Program::parse("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let program = result.unwrap();
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = Program::parse("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let program = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        assert_eq!(bytecode[0], Opcode::LOAD as u8);
        println!("{:?}", bytecode);
        assert!(false);
    }
}
