mod parser;
mod tui;
mod vm;

use std::sync::{Arc, Mutex};
use tui::run_tui;

fn main() {
    let program = std::fs::read_to_string("program.iloc").expect("Failed to read program file");

    let vm = Arc::new(Mutex::new(vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(parser::parse_iloc(&program).unwrap());

    run_tui(vm).unwrap();
}
