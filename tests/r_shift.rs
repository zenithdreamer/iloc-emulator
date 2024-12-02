use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn r_shift_positive() {
    let program = "
    loadI 4 => r1
    loadI 1 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 2); // 4 >> 1 = 2
}

#[test]
fn r_shift_zero_shift() {
    let program = "
    loadI 2 => r1
    loadI 0 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 2); // 2 >> 0 = 2
}

#[test]
fn r_shift_negative() {
    let program = "
    loadI -4 => r1
    loadI 1 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], -2); // Arithmetic right shift preserves the sign bit
}

#[test]
fn r_shift_large_value() {
    let program = "
    loadI 1024 => r1
    loadI 5 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 1024 >> 5); // 1024 >> 5 = 32
}

#[test]
fn r_shift_large_shift_amount() {
    let program = "
    loadI 1024 => r1
    loadI 31 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 0); // 1024 >> 31 = 0
}

#[test]
fn r_shift_zero_value() {
    let program = "
    loadI 0 => r1
    loadI 3 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 0); // 0 >> 3 = 0
}

#[test]
fn r_shift_self_shift() {
    let program = "
    loadI 56 => r1
    loadI 3 => r2
    rshift r1, r2 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 56 >> 3); // 56 >> 3 = 7
}
