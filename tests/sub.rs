use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn sub_positive() {
    let program = "
    loadI 10 => r0
    loadI 5 => r1
    sub r0, r1 => r2
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
fn sub_negative() {
    let program = "
    loadI -10 => r0
    loadI -5 => r1
    sub r0, r1 => r2
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
fn sub_zero_from_positive() {
    let program = "
    loadI 10 => r0
    loadI 0 => r1
    sub r0, r1 => r2
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
fn sub_positive_and_negative() {
    let program = "
    loadI 10 => r0
    loadI -5 => r1
    sub r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 15);
}

#[test]
fn sub_negative_and_positive() {
    let program = "
    loadI -10 => r0
    loadI 5 => r1
    sub r0, r1 => r2
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
fn sub_zero_from_negative() {
    let program = "
    loadI -10 => r0
    loadI 0 => r1
    sub r0, r1 => r2
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
fn sub_positive_overflow() {
    let program = "
    loadI 2147483647 => r0
    loadI -1 => r1
    sub r0, r1 => r3
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
fn sub_negative_underflow() {
    let program = "
    loadI -2147483648 => r0
    loadI 1 => r1
    sub r0, r1 => r3
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
fn sub_zero_from_zero() {
    let program = "
    loadI 0 => r0
    loadI 0 => r1
    sub r0, r1 => r2
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
fn sub_number_from_itself() {
    let program = "
    loadI 7 => r0
    sub r0, r0 => r0
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
