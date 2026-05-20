#[derive(Clone)]
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_size: usize,
    ram_size: usize,
}

impl Cartridge {
    pub fn new(rom_data: Vec<u8>) -> Result<Self, String> {
        if rom_data.len() < 0xC0 {
            return Err("ROM too small".to_string());
        }

        // Determine RAM size from header (offset 0x1F0)
        let ram_size = match rom_data.get(0x1FA).copied().unwrap_or(0) {
            0 => 0,
            1 => 0x8000,       // 32 KB
            2 => 0x20000,      // 128 KB
            3 => 0x40000,      // 256 KB
            _ => 0x10000,      // 64 KB (default)
        };

        Ok(Cartridge {
            rom_size: rom_data.len(),
            rom: rom_data,
            ram: vec![0u8; ram_size],
            ram_size,
        })
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        if address >= 0x0800_0000 && address < 0x0800_0000 + self.rom_size as u32 {
            self.rom[(address - 0x0800_0000) as usize]
        } else if address >= 0x0E00_0000 && address < 0x0E00_0000 + self.ram_size as u32 {
            self.ram[(address - 0x0E00_0000) as usize]
        } else {
            0
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        if address >= 0x0E00_0000 && address < 0x0E00_0000 + self.ram_size as u32 {
            self.ram[(address - 0x0E00_0000) as usize] = value;
        }
    }
}
