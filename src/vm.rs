use std::collections::HashMap;

pub struct VM {
    registers: HashMap<String, i32>,
    memory: Vec<u8>,
    pc: usize,
    program: Vec<String>,
}

impl VM {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: HashMap::new(),
            memory: vec![0; memory_size],
            pc: 0,
            program: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<String>) {
        self.program = program.clone();
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        let instruction = self.program[self.pc].clone();
        self.execute(&instruction);
        self.pc += 1;
        true
    }

    #[allow(dead_code)]
    pub fn run(&mut self) {
        while self.step() {}
    }

    fn execute(&mut self, instruction: &str) {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        match parts[0] {
            "nop" => {
                // No operation
            }
            "add" => {
                // add r1, r2 => r3
                // Meaning: r1 + r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1.wrapping_add(r2);
                self.registers.insert(reg.to_string(), result);
            }
            "sub" => {
                // sub r1, r2 => r3
                // Meaning: r1 - r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1.wrapping_sub(r2);
                self.registers.insert(reg.to_string(), result);
            }
            "mult" => {
                // mult r1, r2 => r3
                // Meaning: r1 * r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1.wrapping_mul(r2);
                self.registers.insert(reg.to_string(), result);
            }
            "div" => {
                // div r1, r2 => r3
                // Meaning: r1 / r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();

                // Check for division by zero
                if r2 == 0 {
                    // Halt the program, throw an error
                    panic!("Division by zero");
                }

                let result = r1.wrapping_div(r2);
                self.registers.insert(reg.to_string(), result);
            }
            "addI" => {
                // addI r1, c2 => r3
                // Meaning: r1 + c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1.wrapping_add(c2);
                self.registers.insert(reg.to_string(), result);
            }
            "subI" => {
                // subI r1, c2 => r3
                // Meaning: r1 - c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1.wrapping_sub(c2);
                self.registers.insert(reg.to_string(), result);
            }
            "rsubI" => {
                // rsubI r1, c2 => r3
                // Meaning: c2 - r1 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = c2.wrapping_sub(r1);
                self.registers.insert(reg.to_string(), result);
            }
            "multI" => {
                // multI r1, c2 => r3
                // Meaning: r1 * c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1.wrapping_mul(c2);
                self.registers.insert(reg.to_string(), result);
            }
            "divI" => {
                // divI r1, c2 => r3
                // Meaning: r1 / c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();

                // Check for division by zero
                if c2 == 0 {
                    // Halt the program, throw an error
                    panic!("Division by zero");
                }

                let result = r1.wrapping_div(c2);
                self.registers.insert(reg.to_string(), result);
            }
            "rdivI" => {
                // rdivI r1, c2 => r3
                // Meaning: c2 / r1 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();

                // Check for division by zero
                if r1 == 0 {
                    // Halt the program, throw an error
                    panic!("Division by zero");
                }

                let result = c2.wrapping_div(r1);
                self.registers.insert(reg.to_string(), result);
            }
            // Below are untested instructions
            // Bitwise operations
            "lshift" => {
                // lshift r1, r2 => r3
                // Meaning: r1 << r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1 << r2;
                self.registers.insert(reg.to_string(), result);
            }
            "lshiftI" => {
                // lshiftI r1, c2 => r3
                // Meaning: r1 << c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1 << c2;
                self.registers.insert(reg.to_string(), result);
            }
            "rshift" => {
                // rshift r1, r2 => r3
                // Meaning: r1 >> r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1 >> r2;
                self.registers.insert(reg.to_string(), result);
            }
            "rshiftI" => {
                // rshiftI r1, c2 r3
                // Meaning: r1 >> c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = operands[1].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1 >> c2;
                self.registers.insert(reg.to_string(), result);
            }
            // Bitwise logical operations
            "and" => {
                // and r1, r2 r3
                // Meaning: r1 & r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1 & r2;
                self.registers.insert(reg.to_string(), result);
            }
            "andI" => {
                // andI r1, c2 r3
                // Meaning: r1 & c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = parts[2].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1 & c2;
                self.registers.insert(reg.to_string(), result);
            }
            "or" => {
                // or r1, r2 r3
                // Meaning: r1 | r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1 | r2;
                self.registers.insert(reg.to_string(), result);
            }
            "orI" => {
                // orI r1, c2 r3
                // Meaning: r1 | c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = parts[2].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1 | c2;
                self.registers.insert(reg.to_string(), result);
            }
            "xor" => {
                // xor r1, r2 r3
                // Meaning: r1 ^ r2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let r2 = self.registers[operands[1]];
                let reg = parts.last().unwrap();
                let result = r1 ^ r2;
                self.registers.insert(reg.to_string(), result);
            }
            "xorI" => {
                // xorI r1, c2 r3
                // Meaning: r1 ^ c2 => r3
                let operands: Vec<&str> = parts[1].split(',').collect();
                let r1 = self.registers[operands[0]];
                let c2: i32 = parts[2].parse().unwrap();
                let reg = parts.last().unwrap();
                let result = r1 ^ c2;
                self.registers.insert(reg.to_string(), result);
            }

            // Data transfer operations
            // Data transfer operations
            "loadI" => {
                // loadI c1 => r2
                let value: i32 = parts[1].parse::<i64>().unwrap_or(0) as i32;
                let reg = parts.last().unwrap();
                self.registers.insert(reg.to_string(), value);
            }

            "load" => {
                // load r1 => r2
                // Meaning: MEMORY[r1] => r2
                // Load the 4-byte value from the memory location specified by r1 to r2
                // If r2 does not exist, create it
                let r1 = self.registers[parts[1]] as usize;
                let reg = parts.last().unwrap();
                if r1 + 3 < self.memory.len() {
                    let bytes = [
                        self.memory[r1] as u8,
                        self.memory[r1 + 1] as u8,
                        self.memory[r1 + 2] as u8,
                        self.memory[r1 + 3] as u8,
                    ];
                    let value = i32::from_le_bytes(bytes);
                    self.registers.insert(reg.to_string(), value);
                }
            }
            "loadAI" => {
                // loadAI r1, c2 => r3
                // Meaning: MEMORY[r1 + c2] => r3
                // Load the 4-byte value from the memory location specified by r1 + c2 to r3
                // If r3 does not exist, create it
                let r1 = self.registers[parts[1]] as usize;
                let c2: i32 = parts[3].parse().unwrap();
                let reg = parts.last().unwrap();
                let address = r1 + c2 as usize;
                if address + 3 < self.memory.len() {
                    let bytes = [
                        self.memory[address] as u8,
                        self.memory[address + 1] as u8,
                        self.memory[address + 2] as u8,
                        self.memory[address + 3] as u8,
                    ];
                    let value = i32::from_le_bytes(bytes);
                    self.registers.insert(reg.to_string(), value);
                }
            }
            "loadAO" => {
                // loadAO r1, r2 => r3
                // Meaning: MEMORY[r1 + r2] => r3
                // Load the 4-byte value from the memory location specified by r1 + r2 to r3
                // If r3 does not exist, create it
                let r1 = self.registers[parts[1]] as usize;
                let r2 = self.registers[parts[3]] as usize;
                let reg = parts.last().unwrap();
                let address = r1 + r2;
                if address + 3 < self.memory.len() {
                    let bytes = [
                        self.memory[address] as u8,
                        self.memory[address + 1] as u8,
                        self.memory[address + 2] as u8,
                        self.memory[address + 3] as u8,
                    ];
                    let value = i32::from_le_bytes(bytes);
                    self.registers.insert(reg.to_string(), value);
                }
            }
            "cload" => {
                // cload r1 => r2
                // Meaning: MEMORY[r1] => r2
                // Load the 4-byte character from the memory location specified by r1 to r2
                // If r2 does not exist, create it
                let r1 = self.registers[parts[1]] as usize;
                let reg = parts.last().unwrap();
                if r1 + 3 < self.memory.len() {
                    let bytes = [
                        self.memory[r1] as u8,
                        self.memory[r1 + 1] as u8,
                        self.memory[r1 + 2] as u8,
                        self.memory[r1 + 3] as u8,
                    ];
                    let value = i32::from_le_bytes(bytes);
                    self.registers.insert(reg.to_string(), value);
                }
            }
            "cloadAI" => {
                panic!("Not implemented");
            }
            "cloadAO" => {
                panic!("Not implemented");
            }
            "store" => {
                // store r1 => r2
                // Meaning: r1 => MEMORY[r2]
                // Store the 4-byte value in r1 to the memory location specified by r2
                let r1 = self.registers[parts[1]];
                let r2 = self.registers[parts[3]] as usize;
                if r2 + 3 < self.memory.len() {
                    let bytes = r1.to_le_bytes();
                    self.memory[r2] = bytes[0];
                    self.memory[r2 + 1] = bytes[1];
                    self.memory[r2 + 2] = bytes[2];
                    self.memory[r2 + 3] = bytes[3];
                }
            }
            "storeAI" => {
                // storeAI r1 => r2, c3
                // Meaning: r1 => MEMORY[r2 + c3]
                // Store the 4-byte value in r1 to the memory location specified by r2 + c3
                let r1 = self.registers[parts[1]];
                let r2 = self.registers[parts[3]] as usize;
                let c3: i32 = parts[5].parse().unwrap();
                let address = r2 + c3 as usize;
                if address + 3 < self.memory.len() {
                    let bytes = r1.to_le_bytes();
                    self.memory[address] = bytes[0];
                    self.memory[address + 1] = bytes[1];
                    self.memory[address + 2] = bytes[2];
                    self.memory[address + 3] = bytes[3];
                }
            }
            "storeAO" => {
                // storeAO r1 => r2, r3
                // Meaning: r1 => MEMORY[r2 + r3]
                // Store the 4-byte value in r1 to the memory location specified by r2 + r3
                let r1 = self.registers[parts[1]];
                let r2 = self.registers[parts[3]] as usize;
                let r3 = self.registers[parts[5]] as usize;
                let address = r2 + r3;
                if address + 3 < self.memory.len() {
                    let bytes = r1.to_le_bytes();
                    self.memory[address] = bytes[0];
                    self.memory[address + 1] = bytes[1];
                    self.memory[address + 2] = bytes[2];
                    self.memory[address + 3] = bytes[3];
                }
            }

            _ => {
                // Unsupported instruction
            }
        }
    }

    pub fn get_state(&self) -> (&HashMap<String, i32>, &[u8], usize) {
        (&self.registers, &self.memory, self.pc)
    }

    pub fn get_program(&self) -> Vec<String> {
        self.program.clone()
    }
}
