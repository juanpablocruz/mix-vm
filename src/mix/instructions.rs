use super::machine::Mix;

pub const LDA: i32 = 1;
pub const STA: i32 = 2;
pub const ADD: i32 = 3;
pub const SUB: i32 = 4;
pub const DIV: i32 = 5;
pub const JMP: i32 = 6;
pub const JZ: i32 = 7;
pub const JL: i32 = 8;
pub const CMP: i32 = 9;
pub const HLT: i32 = 0;

impl Mix {
    // here are the methods for the instructions of the machine


    // LDA: loads a value
    pub fn lda(&mut self, address: usize) -> Result<(), &'static str> {
        let value = self.read_memory(address).ok_or("Memory address out of range")?;
        self.load_a(value);
        Ok(())
    }

    // STA: Stores the value of register A into memory
    pub fn sta(&mut self, address: usize) -> Result<(), &'static str> {
        let a = self.read_a();
        self.set_memory(address, a)
    }

    // ADD: Adds a value of the memory to the register A
    pub fn add(&mut self, address: usize) -> Result<(), &'static str> {
        let value = self.read_memory(address).ok_or("Memory address out of range")?;
        let a = self.read_a();
        self.load_a(a + value);
        Ok(())
    }

    pub fn sub(&mut self, address: usize) -> Result<(), &'static str> {
        let value = self.read_memory(address).ok_or("Memory address out of range")?;
        let a = self.read_a();
        self.load_a(a - value);
        Ok(())
    }

    pub fn div(&mut self, address: usize) -> Result<(), &'static str> {
        let value = self.read_memory(address).ok_or("Memory address out of range")?;
        if value == 0 {
            return Err("Division by zero");
        }
        let a = self.read_a();
        self.load_a(a / value);
        Ok(())
    }


    // JMP: changes the location of the next instruction to execute
    pub fn jmp(&mut self, address: usize) {
        self.set_location(address as i32);
    }

    pub fn jz(&mut self, address: usize) {
        if self.read_a() == 0 {
            self.set_location(address as i32);
        }
    }

    pub fn jl(&mut self, address: usize) {
      if self.read_a() < 0 {
          self.set_location(address as i32);
      }
    }

    pub fn cmp(&mut self, address: usize) -> Result<(), &'static str> {
        let value = self.read_memory(address).ok_or("Memory address out of range")?;
        let a = self.read_a();
        self.set_comparison(a - value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_works() {
        let mut mix = Mix::new();
        mix.set_memory(0, 123).unwrap();
        mix.display_memory();

        mix.lda(0).unwrap();
        mix.display_registers();

        assert_eq!(mix.read_a(), 123);
    }
    #[test]
    fn test_lda_out_of_range_address() {
        let mut mix = Mix::new();
        assert!(mix.lda(5000).is_err());
    }

    #[test]
    fn test_sta_works() {
        let mut mix = Mix::new();
        mix.load_a(123);
        mix.sta(0).unwrap();
        assert_eq!(mix.read_memory(0).unwrap(), 123);
    }

    #[test]
    fn test_sta_out_of_range_address() {
        let mut mix = Mix::new();
        assert!(mix.sta(5000).is_err());
    }

}
