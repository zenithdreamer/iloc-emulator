use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn mult_i_positive() {
    let program = "
    loadI 10 => r0
    multI r0, 5 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 50);
}

#[test]
fn mult_i_negative() {
    let program = "
    loadI -10 => r0
    multI r0, 5 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -50);
}

#[test]
fn mult_i_zero() {
    let program = "
    loadI 0 => r0
    multI r0, 5 => r1
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
fn mult_i_positive_and_negative() {
    let program = "
    loadI 10 => r0
    multI r0, -5 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -50);
}

#[test]
fn mult_i_large_positive() {
    let program = "
    loadI 1000000 => r0
    multI r0, 2000000 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let result = 1000000i64 * 2000000i64;
    let wrapped_result = result as i32;
    assert_eq!(registers["r1"], wrapped_result);
}

#[test]
fn mult_i_overflow() {
    let program = "
    loadI 2147483647 => r0
    multI r0, 2 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let result = (2147483647i64 * 2i64) as i32;
    assert_eq!(registers["r1"], result);
}

#[test]
fn mult_i_zero_and_zero() {
    let program = "
    loadI 0 => r0
    multI r0, 0 => r1
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
fn mult_i_number_by_itself() {
    let program = "
    loadI 7 => r0
    multI r0, 7 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 49);
}
