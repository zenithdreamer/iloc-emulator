use iloc;
use std::sync::{Arc, Mutex};

#[test]
fn l_shift_i_positive() {
    let program = "
    loadI 2 => r1
    lshiftI r1, 1 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 4); // 2 << 1 = 4
}

#[test]
fn l_shift_i_zero_shift() {
    let program = "
    loadI 2 => r1
    lshiftI r1, 0 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 2); // 2 << 0 = 2
}

#[test]
fn l_shift_i_negative() {
    let program = "
    loadI -2 => r1
    lshiftI r1, 1 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], -4); // Two's complement left shift
}

#[test]
fn l_shift_i_large_shift() {
    let program = "
    loadI 1024 => r1
    lshiftI r1, 5 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 1024 << 5); // 1024 << 5 = 32768
}

#[test]
fn l_shift_i_large_shift_amount() {
    let program = "
    loadI 1 => r1
    lshiftI r1, 31 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    let expected_result = 1 << 31; // May overflow if using 32-bit signed integers
    assert_eq!(registers["r3"], expected_result);
}

#[test]
fn l_shift_i_zero_value() {
    let program = "
    loadI 0 => r1
    lshiftI r1, 3 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 0); // 0 << 3 = 0
}

#[test]
fn l_shift_i_self_shift() {
    let program = "
    loadI 7 => r1
    lshiftI r1, 3 => r3
    ";
    let vm = Arc::new(Mutex::new(iloc::vm::VM::new(1024)));
    vm.lock()
        .unwrap()
        .load_program(iloc::parser::parse_iloc(program).unwrap());

    vm.lock().unwrap().run();

    let binding = vm.lock().unwrap();
    let state = binding.get_state();
    let registers = &state.0;

    assert_eq!(registers["r3"], 7 << 3); // 7 << 3 = 56
}
