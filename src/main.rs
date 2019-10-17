pub mod vm;
pub mod instructions;
pub mod repl;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
