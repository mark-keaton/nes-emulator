#[derive(Copy, Clone, Debug)]
pub struct Memory(pub [u8; 0xFFFF]);

impl Memory {
    pub fn new() -> Self {
        Memory([0; 0xFFFF])
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.0[addr as usize] = data;
        ()
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        let le_data: [u8; 2] = data.to_le_bytes();
        self.write(addr, le_data[0]);
        self.write(addr.wrapping_add(1), le_data[1]);
        ()
    }

    /// Copy program into reserved memory locations for the Program
    /// Reserved space from 0x8000 -> 0xFFFF
    ///
    /// But only as much space as needed for the program is used
    pub fn load_program(&mut self, program: Vec<u8>) {
        let reserved_program_addresses = 0x8000..(0x8000 + program.len());
        let program_copy = &program[..];
        self.0[reserved_program_addresses].copy_from_slice(program_copy);
    }

    pub fn push_to_stack(&mut self, stack_pointer: u8, data: u8) {
        let stack_addr = (0x0100 + stack_pointer as u16) as usize;
        self.0[stack_addr] = data;
    }

    pub fn pull_from_stack(&self, stack_pointer: u8) -> u8 {
        let stack_addr = (0x0100 + stack_pointer as u16) as usize;
        self.0[stack_addr]
    }
}
