// machine.rs
pub struct Mix {
    a: i32,
    x: i32,
    i: [i32; 6],
    memory: [i32; 4000],
    comparison: i32,
    location: i32,
}
use super::instructions::{LDA,STA,ADD,SUB,DIV,JMP,JZ,JL,CMP,HALT};

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


    pub fn get_comparison(&self) -> i32 {
        self.comparison
    }

    pub fn get_location(&self) -> usize {
        self.location as usize
    }

    pub fn set_comparison(&mut self, value: i32) {
        self.comparison = value;
    }

    pub fn set_location(&mut self, address: i32) -> Result<(), &'static str> {
        if address < 0 || address >= self.memory.len() as i32 {
            Err("Memory location out of range")
        } else {
            self.location = address;
            Ok(())
        }
    }

    // load a value in the register A
    pub fn load_a(&mut self, value: i32) {
        self.a = value;
    }

    pub fn set_memory(&mut self, address: usize, value: i32) -> Result<(), &'static str> {
        if let Some(cell) = self.memory.get_mut(address) {
            *cell = value;
            Ok(())
        } else {
            Err("Memory address out of range")
        }
    }

    pub fn read_a(&self) -> i32 {
        self.a
    }

    // load a value in the register X
    pub fn load_x(&mut self, value: i32) {
        self.x = value;
    }

    pub fn read_x(&self) -> i32 {
        self.x
    }

    // load a value in the index register i
    pub fn load_i(&mut self, index: usize, value: i32) -> Result<(), &'static str> {
        if let Some(i) = self.i.get_mut(index) {
            *i = value;
            Ok(())
        } else {
            Err("Index out of range")
        }
    }

    pub fn read_i(&self, index: usize) -> Option<i32> {
        self.i.get(index).copied()
    }

    pub fn read_memory(&self, address: usize) -> Option<i32> {
        self.memory.get(address).copied()
    }

    pub fn load_program(&mut self, program: &[i32]) -> Result<(), &'static str> {
        if program.len() > self.memory.len() {
            return Err("Program is too large to fit in memory");
        }
        for (i, instruction) in program.iter().enumerate() {
            self.set_memory(i as usize, *instruction)?;
        }

        Ok(())
    }

    pub fn execute(&mut self) -> Result<bool, &'static str> {
        let instruction = self.read_memory(self.get_location()).ok_or("Memory address out of range")?;
        match instruction {
            HLT => {
                self.set_location(self.get_location() as i32 + 1);
                Ok(true)
            },
            LDA => {
                self.lda(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            STA => {
                let address = self.read_memory(self.get_location() + 1).ok_or("Memory address out of range")?;
                self.sta(address as usize);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            ADD => {
                self.add(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            SUB => {
                self.sub(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            DIV => {
                self.div(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            JMP => {
                self.jmp(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            JZ => {
                self.jz(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            JL => {
                self.jl(self.get_location() + 1);
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            CMP => {
                self.cmp(self.get_location() + 1)?;
                self.set_location(self.get_location() as i32 + 1);
                Ok(false)
            },
            _ => {
                println!("Unkown instruction {}", instruction);
                Err("Unknown instruction")
            }
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            if self.execute()? {
                break;
            }
            self.set_location(self.get_location() as i32 + 1)?;
        }
        Ok(())
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

