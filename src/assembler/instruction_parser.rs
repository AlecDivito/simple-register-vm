use crate::assembler::token_parser::Token;
use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    /// Parse a line of assembly
    pub fn parse_one_line(line: &str) -> Result<AssemblerInstruction, &'static str> {
        // parse line of text
        let cleaned_line = line.trim().to_lowercase();
        let str_token: Vec<&str> = cleaned_line.split_whitespace().collect();
        // get the first argument (being the opcode)
        let opcode_token = Token::parse_opcode(str_token[0]);
        // get the value of the opcode token
        let opcode = match opcode_token.opcode() {
            Some(o) => o,
            _ => return Err("No available Opcode is available"),
        };
        // match opcodes
        match opcode {
            Opcode::LOAD => {
                let register = Token::parse_register(str_token[1]);
                let operand = Token::parse_integer_operand(str_token[2]);
                Ok(AssemblerInstruction {
                    opcode: opcode_token,
                    operand1: register,
                    operand2: operand,
                    operand3: None,
                })
            }
            _ => Err("Line is an Illegal operation"),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        let opcode = self.opcode.opcode().unwrap();
        if opcode == Opcode::IGL {
            println!("illegal opcode found in opcode field");
            std::process::exit(1);
        } else {
            results.push(opcode as u8)
        }
        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results);
            }
        }
        results
    }

    fn extract_operand(token: &Token, results: &mut Vec<u8>) {
        match token {
            Token::Register(r) => {
                results.push(*r);
            }
            Token::IntegerOperand(i) => {
                let converted = *i as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_one_line() {
        let line = "LOAD $0 #500\n";
        let token = AssemblerInstruction::parse_one_line(line);
        assert_eq!(token.is_ok(), true);
    }
}
