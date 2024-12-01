use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn mult_positive() {
    let program = "
    loadI 10 => r0
    loadI 5 => r1
    mult r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 50);
}

#[test]
fn mult_negative() {
    let program = "
    loadI -10 => r0
    loadI 5 => r1
    mult r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -50);
}

#[test]
fn mult_zero() {
    let program = "
    loadI 0 => r0
    loadI 5 => r1
    mult r0, r1 => r2
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
fn mult_positive_and_negative() {
    let program = "
    loadI 10 => r0
    loadI -5 => r1
    mult r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -50);
}

#[test]
fn mult_negative_and_positive() {
    let program = "
    loadI -10 => r0
    loadI 5 => r1
    mult r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -50);
}

#[test]
fn mult_zero_and_negative() {
    let program = "
    loadI 0 => r0
    loadI -5 => r1
    mult r0, r1 => r2
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
fn mult_large_positive() {
    let program = "
    loadI 1000000000 => r0
    loadI 2000000000 => r1
    mult r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let result = 1000000000i64 * 2000000000i64;
    assert_eq!(registers["r2"], result as i32);
}

#[test]
fn mult_overflow() {
    let program = "
    loadI 2147483647 => r0
    loadI 2 => r1
    mult r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], -2);
}

#[test]
fn mult_negative_overflow() {
    let program = "
    loadI -2147483648 => r0
    loadI 2 => r1
    mult r0, r1 => r2
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
fn mult_zero_plus_zero() {
    let program = "
    loadI 0 => r0
    loadI 0 => r1
    mult r0, r1 => r2
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
fn mult_number_by_itself() {
    let program = "
    loadI 7 => r0
    mult r0, r0 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], 49);
}
