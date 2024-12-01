use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn add_positive() {
    let program = "
    loadI 10 => r0
    add r0, r0 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], 20);
}

#[test]
fn add_negative() {
    let program = "
    loadI -10 => r0
    add r0, r0 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], -20);
}

#[test]
fn add_zero() {
    let program = "
    loadI 0 => r0
    add r0, r0 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], 0);
}

#[test]
fn add_positive_to_negative() {
    let program = "
    loadI 10 => r0
    loadI -5 => r1
    add r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 5);
}

#[test]
fn add_negative_to_positive() {
    let program = "
    loadI -10 => r0
    loadI 5 => r1
    add r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -5);
}

#[test]
fn add_negative_to_negative() {
    let program = "
    loadI -10 => r0
    loadI -5 => r1
    add r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -15);
}

#[test]
fn add_zero_to_positive() {
    let program = "
    loadI 10 => r0
    loadI 0 => r1
    add r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 10);
}

#[test]
fn add_zero_to_negative() {
    let program = "
    loadI -10 => r0
    loadI 0 => r1
    add r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -10);
}

#[test]
fn add_positive_overflow() {
    let program = "
    loadI 2147483647 => r0
    loadI 1 => r1
    add r0, r1 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], -2147483648);
}

#[test]
fn add_negative_overflow() {
    let program = "
    loadI -2147483648 => r0
    loadI -1 => r1
    add r0, r1 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 2147483647);
}

#[test]
fn add_two_large_positive_overflow() {
    let program = "
    loadI 1000000000 => r0
    loadI 2000000000 => r1
    add r0, r1 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], -1294967296);
}

#[test]
fn add_two_large_negative_underflow() {
    let program = "
    loadI -1000000000 => r0
    loadI -2000000000 => r1
    add r0, r1 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 1294967296);
}

#[test]
fn add_zero_plus_zero() {
    let program = "
    loadI 0 => r0
    loadI 0 => r1
    add r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 0);
}

#[test]
fn add_positive_to_itself() {
    let program = "
    loadI 7 => r0
    add r0, r0 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], 14);
}
