use crate::instructions::Opcode;

#[derive(PartialEq, Debug)]
pub enum Token {
    Op(Opcode),
    Register(u8),
    IntegerOperand(i32),
}

impl Token {
    /// Parse string in the format of a register
    /// opcode[in] must be lower case
    /// ex. "load"
    pub fn parse_opcode(opcode: &str) -> Token {
        Token::Op(Opcode::from(opcode))
    }

    /// Parse string in the format of a register
    /// ex. "$5"
    pub fn parse_register(register: &str) -> Option<Token> {
        let mut register_iter = register.chars().peekable();
        if register_iter.peek().is_none()
            || (register_iter.peek().is_some() && register_iter.next().unwrap() != '$')
        {
            return None;
        }

        if register_iter.peek().is_none() {
            return None;
        }

        let mut i = 1;
        while register_iter.peek().is_some() {
            if Token::is_digit(register_iter.next().unwrap()) {
                i = i + 1;
            } else {
                println!("not a digit");
                return None;
            }
        }
        match register[1..i].parse::<u8>() {
            Ok(n) => Some(Token::Register(n)),
            Err(_) => None,
        }
    }

    /// Parse string in the format of an operand
    /// ex. "#300"
    pub fn parse_integer_operand(operand: &str) -> Option<Token> {
        // check to see if operand is formatted correctly
        let mut operand_iter = operand.chars().peekable();
        if operand_iter.peek().is_none()
            || (operand_iter.peek().is_some() && operand_iter.next().unwrap() != '#')
        {
            return None;
        }

        if operand_iter.peek().is_none() {
            return None;
        }

        // parse the number from the string
        let mut i = 1;
        while operand_iter.peek().is_some() {
            if Token::is_digit(operand_iter.next().unwrap()) {
                i = i + 1;
            } else {
                return None;
            }
        }
        match operand[1..i].parse::<i32>() {
            Ok(n) => Some(Token::IntegerOperand(n)),
            Err(_) => None,
        }
    }

    pub fn opcode(&self) -> Option<Opcode> {
        match self {
            Token::Op(o) => Some(o.clone()),
            _ => None,
        }
    }

    pub fn register(&self) -> Option<u8> {
        match *self {
            Token::Register(r) => Some(r),
            _ => None,
        }
    }

    pub fn integer_operand(&self) -> Option<i32> {
        match *self {
            Token::IntegerOperand(i) => Some(i),
            _ => None,
        }
    }

    fn is_digit(character: char) -> bool {
        character >= '0' && character <= '9'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let token = Token::parse_opcode("load");
        assert_eq!(token, Token::Op(Opcode::LOAD));

        // test that invalid opcode isn't recognized
        let token = Token::parse_opcode("aold");
        assert_eq!(token, Token::Op(Opcode::IGL));
    }

    #[test]
    fn test_parse_register() {
        let result = Token::parse_register("$0");
        assert!(result.is_some(), true);
        let token = result.unwrap();
        assert_eq!(token, Token::Register(0));

        let result = Token::parse_register("0");
        assert_eq!(result.is_some(), false);

        let result = Token::parse_register("$a");
        assert_eq!(result.is_some(), false);
    }

    #[test]
    fn test_parse_integer_operand() {
        let result = Token::parse_integer_operand("#500");
        assert!(result.is_some(), true);
        let token = result.unwrap();
        assert_eq!(token, Token::IntegerOperand(500));

        let result = Token::parse_integer_operand("500");
        assert_eq!(result.is_some(), false);

        let result = Token::parse_integer_operand("#abc");
        assert_eq!(result.is_some(), false);

        let result = Token::parse_integer_operand("$500");
        assert_eq!(result.is_some(), false);
    }
}
