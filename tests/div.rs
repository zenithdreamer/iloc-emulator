use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn div_positive() {
    let program = "
    loadI 10 => r0
    loadI 5 => r1
    div r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 2);
}

#[test]
fn div_negative() {
    let program = "
    loadI -10 => r0
    loadI 5 => r1
    div r0, r1 => r2
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
fn div_zero() {
    let program = "
    loadI 0 => r0
    loadI 5 => r1
    div r0, r1 => r2
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
fn div_by_negative() {
    let program = "
    loadI 10 => r0
    loadI -5 => r1
    div r0, r1 => r2
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
fn div_zero_by_number() {
    let program = "
    loadI 0 => r0
    loadI 10 => r1
    div r0, r1 => r2
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
fn div_large_positive() {
    let program = "
    loadI 1000000000 => r0
    loadI 1000000 => r1
    div r0, r1 => r2
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r2"], 1000);
}

#[test]
fn div_by_zero() {
    let program = "
    loadI 10 => r0
    loadI 0 => r1
    div r0, r1 => r2
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
fn div_number_by_itself() {
    let program = "
    loadI 7 => r0
    div r0, r0 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], 1);
}

#[test]
fn div_number_by_larger() {
    let program = "
    loadI 7 => r0
    loadI 10 => r1
    div r0, r1 => r0
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
fn div_larger_by_number() {
    let program = "
    loadI 10 => r0
    loadI 7 => r1
    div r0, r1 => r0
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r0"], 1);
}
