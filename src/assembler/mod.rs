//! Assembler
//!
//! Parse code and convert human readable assembly to hex code
//!
//! Lexer Grammar
//! 1. A `Program` is composed of `Instructions`
//! 2. An `Instruction` is composed of:
//!     - An `Opcode`
//!     - A `Register`
//!     - A `IntegerOperand`
//!     - A `Newline`
//! 3. An `Opcode is composed of:
//!     - One or more `Letters` in a row
//!     - A `space`
//! 4. A `Register` is composed of:
//!     - The symbol `$`
//!     - A `Number`
//!     - A `Space`
//! 5. A `IntegerOperand` is composed of:
//!     - The symbol `#`
//!     - A `Number`
//! 6. A `Number is composed of:
//!     - The symbols `0-9`
//! 7. A `Newline is composed of:
//!     - The symbol `\` followed by the symbol `n`

pub mod instruction_parser;
pub mod program_parser;
pub mod token_parser;
