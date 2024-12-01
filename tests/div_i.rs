use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn div_i_positive() {
    let program = "
    loadI 10 => r0
    divI r0, 2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 5);
}

#[test]
fn div_i_negative_dividend() {
    let program = "
    loadI -10 => r0
    divI r0, 2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -5);
}

#[test]
fn div_i_negative_divisor() {
    let program = "
    loadI 10 => r0
    divI r0, -2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -5);
}

#[test]
fn div_i_negative_both() {
    let program = "
    loadI -10 => r0
    divI r0, -2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 5);
}

#[test]
fn div_i_zero_dividend() {
    let program = "
    loadI 0 => r0
    divI r0, 5 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 0);
}

#[test]
fn div_i_large_positive() {
    let program = "
    loadI 1000000 => r0
    divI r0, 2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 500000);
}

#[test]
fn div_i_overflow() {
    let program = "
    loadI 2147483647 => r0
    divI r0, 2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let expected_result = 2147483647 / 2;
    assert_eq!(registers["r1"], expected_result);
}

#[test]
fn div_i_division_by_one() {
    let program = "
    loadI 7 => r0
    divI r0, 1 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 7);
}

#[test]
fn div_i_division_by_zero() {
    let program = "
    loadI 10 => r0
    divI r0, 0 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    let result = std::panic::catch_unwind(|| {
        vm.lock().unwrap().run();
    });

    assert!(result.is_err(), "Division by zero should panic");
}
