use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn rsubi_positive() {
    let program = "
    loadI 10 => r0
    rsubI r0, 5 => r1
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
fn rsubi_negative() {
    let program = "
    loadI -10 => r0
    rsubI r0, -5 => r1
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
fn rsubi_zero() {
    let program = "
    loadI 0 => r0
    rsubI r0, 0 => r1
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
fn rsubi_negative_to_positive() {
    let program = "
    loadI 10 => r0
    rsubI r0, -5 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], -15);
}

#[test]
fn rsubi_positive_to_negative() {
    let program = "
    loadI -10 => r0
    rsubI r0, 5 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 15);
}

#[test]
fn rsubi_large_positive() {
    let program = "
    loadI 1000000000 => r0
    rsubI r0, 2000000000 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let result = 2000000000i64 - 1000000000i64;
    let wrapped_result = result as i32;

    assert_eq!(registers["r1"], wrapped_result);
}

#[test]
fn rsubi_underflow() {
    let program = "
    loadI -2147483648 => r0
    rsubI r0, -1 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 2147483647);
}

#[test]
fn rsubi_overflow() {
    let program = "
    loadI -2147483648 => r0
    rsubI r0, -1 => r1
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r1"], 2147483647);
}

#[test]
fn rsubi_zero_plus_zero() {
    let program = "
    loadI 0 => r0
    rsubI r0, 0 => r1
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
fn rsubi_positive_to_itself() {
    let program = "
    loadI 7 => r0
    rsubI r0, 7 => r0
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
