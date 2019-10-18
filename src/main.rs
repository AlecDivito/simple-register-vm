pub mod assembler;
pub mod instructions;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
