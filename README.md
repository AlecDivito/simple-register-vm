# Simple Virtual Machine

this is a virtual machine that is implemented fallowing "[So you Want to build a Language VM](https://blog.subnetzero.io/post/building-language-vm-part-01/)".

The virtual machine will be based on a register-based virtual machine

## Details

Our virtual CPUs will have the ability to take 32 bits of data at a time, execute
it, and then go get another group of 32 bits. At a very general level, that is
all hardware processors do:

1. Read next instruction
2. Execute instruction
3. Repeat

A grouping of 32-bits is an Instructions.
1. The first 8 are the Opcode
2. The remaining bits will be the Operands
Using this design our opcodes can have up to 3 Operands (arguments)

### What's an Opcode?

A opcode is a command that is represented by a byte. In this virtual machine
it will be possible to have up to 255 Opcodes

### What are problems with the current implementation?

If we were to load a number, we out only have enough space to store 2 bytes
of data (16 bits) because 1 byte will specify the Opcode and 1 byte will
specify the register to target 

# Goals from Tutorial
We want our VM to bee

1. Reasonably performant compared to modern implementations. We'll use Python as our comparison.
2. Fault tolerant
3. Serves as command and control platform for running applications
4. Clustering of VM's running on different physical servers

# Goals

1. Get used to writing a VM and learn some domain specific topics
2. Research Other VM's and compare them to the simple VM
3. Research Webassembly's VM and see if we can create it

# Types of Interpreters

1. Tree-walking
    - normally the first interpreter's people code
    - transform source code into tree like structure and walk though all the
      operations
    - simple
    - flexible
    - Slow compared to other types
2. Stack-based
    - Most common interpreters (JVM, Python)
    - store operations on a stack, pop and push value to operate on it
    - easier to code than register-based VM
    - Decent performance
    - Does not map to real hardware; CPUs use registers
    - Most tasks require more instructions
3. Register-based
    - Least common interpreters (BEAM VM, Lua VM, Dalvik VM)
    - Register-based VMs are much closer to how actual hardware works
    - More performant
    - More complex to code
    - Have to worry about register allocation and de-allocation when writing the compiler

# References

1. [Writing an Interpreter in GO (in RUST, Github)](https://github.com/Rydgel/monkey-rust)
2. [The BEAM VM (Erlang)](https://en.wikipedia.org/wiki/BEAM_(Erlang_virtual_machine))
3. The Lua VM
    - [byte code](http://files.catwell.info/misc/mirror/lua-5.2-bytecode-vm-dirk-laurie/lua52vm.html) 
    - [Instruction set](http://luaforge.net/docman/83/98/ANoFrillsIntroToLua51VMInstructions.pdf)
4. [The Wren VM](https://github.com/munificent/wren)
5. [The Dis VM](http://www.vitanuova.com/inferno/papers/dis.html)