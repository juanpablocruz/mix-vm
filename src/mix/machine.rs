// machine.rs
pub struct Mix {
    a: i32,
    x: i32,
    i: [i32; 6],
    memory: [i32; 4000],
    comparison: i32,
    location: i32,
}

impl Mix {
    pub fn new() -> Self {
        Mix {
            a: 0,
            x: 0,
            i: [0; 6],
            memory: [0; 4000],
            comparison: 0,
            location: 0,
        }
    }

    // load a value in the register A
    pub fn load_a(&mut self, value: i32) {
        self.a = value;
    }

    // Store the value of register A in the memory address
    pub fn store_a(&mut self, address: usize) {
        self.memory[address] = self.a;
    }

    pub fn read_a(&self) -> i32 {
        self.a
    }

    // load a value in the register X
    pub fn load_x(&mut self, value: i32) {
        self.x = value;
    }
    // Store the value of register X in the memory address
    pub fn store_x(&mut self, address: usize) {
        self.memory[address] = self.x;
    }

    // load a value in the index register i
    pub fn load_i(&mut self, index: usize, value: i32) {
        self.i[index] = value;
    }
    // Store the value of register i in the memory address
    pub fn store_i(&mut self, index: usize, address: usize) {
        self.memory[address] = self.i[index];
    }

    pub fn display_memory(&self) {
        println!("Memory:");
        for (i, word) in self.memory.iter().enumerate() {
            if i % 10 == 0 {
                println!();
                print!("{:04}: ", i);
            }
            print!("{:05} ", word);
        }
        println!();
    }

    pub fn display_registers(&self) {
        println!("Registers:");
        println!("A: {:05}", self.a);
        println!("X: {:05}", self.x);
        println!("Comparison: {}", self.comparison);
        println!("Location: {}", self.location);
        for (i, register) in self.i.iter().enumerate() {
            println!("I{}: {:05}", i + 1, register);
        }
    }
}

