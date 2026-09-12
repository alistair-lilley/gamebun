// GameBun RAM/ROM methods
use std::fs::File;
use crate::processor::{RAM, ROM};

pub mod ramrom;

impl Index<u32> for ROM {
    type Output = u8;

    fn index(&self, index: u32) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<u16> for RAM {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if index <= 0x3FFF {
            &self.rom[index]
        } else if index >= 0x4000 && index <= 0x7FFF {
            &self.rom[0x4000 * self.mcb + (index - 0x4000)]
        } else if index >= 0x8000 {
            &self.memory[index]
        }
    }
}

impl IndexMut<u16> for RAM {
    fn index_mut(&mut self, index: usize) {
        if index >= 0x2000 && index <= 0x3FFF {
            &mut self.mcb
        } else {
            &mut self.memory[index]
        }
    }
}

impl RAM {
    pub fn read_ROM(&mut self, filename: String) {
        let mut buffer = [0u8; 0xFFFFF];
        let mut file = File::open(filename)?;
        let bytes_read = file.read(&mut buffer);
        self.rom = ROM {
            data: bytes_read
        }
    }
}