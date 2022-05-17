pub const MEM_SIZE: usize = 0x4000;
pub const ROM_SIZE: usize = 0x2000;
pub const RAM_SIZE: usize = 0x2000;
pub const RAM_VIDEO: usize = 0x2400;

pub struct Memory {
  rom: [u8; ROM_SIZE],
  ram: [u8; RAM_SIZE],
}

impl Memory {
  pub fn new(rom: [u8; ROM_SIZE]) -> Memory {
    Memory { rom, ram: [0x0; RAM_SIZE] }
  }

  /// Read from RAM and ROM
  pub fn read(&self, hblb: usize) -> u8 {
    if hblb < ROM_SIZE {
      self.rom[hblb]
    } else if hblb < MEM_SIZE {
      self.ram[hblb - ROM_SIZE]
    } else {
      0
    }
  }

  /// Only write to RAM
  pub fn write(&mut self, hblb: usize, db: u8) {
    if hblb >= ROM_SIZE && hblb < MEM_SIZE {
      self.ram[hblb - ROM_SIZE] = db;
    }
  }
}