use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn r_div_i_positive() {
    let program = "
    loadI 5 => r0
    rdivI r0, 20 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 4);
}

#[test]
fn r_div_i_negative_divisor() {
    let program = "
    loadI -5 => r0
    rdivI r0, 20 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -4);
}

#[test]
fn r_div_i_zero_divisor() {
    let program = "
    loadI 0 => r0
    rdivI r0, 20 => r1
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

#[test]
fn r_div_i_negative_dividend() {
    let program = "
    loadI 5 => r0
    rdivI r0, -20 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -4);
}

#[test]
fn r_div_i_negative_both() {
    let program = "
    loadI -5 => r0
    rdivI r0, -20 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 4);
}

#[test]
fn r_div_i_large_positive() {
    let program = "
    loadI 100000 => r0
    rdivI r0, 1000000000 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let expected_result = 1000000000 / 100000;
    assert_eq!(registers["r1"], expected_result);
}

#[test]
fn r_div_i_large_negative() {
    let program = "
    loadI -100000 => r0
    rdivI r0, 1000000000 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let expected_result = 1000000000 / -100000;
    assert_eq!(registers["r1"], expected_result);
}

#[test]
fn r_div_i_division_by_one() {
    let program = "
    loadI 1 => r0
    rdivI r0, 7 => r1
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
fn r_div_i_self_division() {
    let program = "
    loadI 7 => r0
    rdivI r0, 7 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 1);
}
