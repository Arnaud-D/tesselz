use crate::exec::{exec, get_test_program};

mod exec;
mod defs;
mod builtins;
mod expr;

fn main() {
    let program = get_test_program();
    exec(program);
}
