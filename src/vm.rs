/// VM represents our CPU that emulates the functionality of a hardware CPU
pub struct VM {
    /// registers available to our VM
    registers: [i32; 32]
}

impl VM {
    /// Create a new registered-based virtual machine
    pub fn new() -> VM {
        VM {
            registers: [0; 32]
        }
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
}