use crate::cartridge::Cartridge;

pub struct Memory {
    // GBA Memory Map (16 MB address space)
    bios: [u8; 0x4000],              // 0x0000_0000 - 0x0000_3FFF (16 KB)
    pub on_board_wram: [u8; 0x40000], // 0x0200_0000 - 0x0203_FFFF (256 KB)
    pub on_chip_wram: [u8; 0x8000],   // 0x0300_0000 - 0x0300_7FFF (32 KB)
    io_registers: [u8; 0x400],       // 0x0400_0000 - 0x0400_03FF (1 KB)
    pub palette_ram: [u8; 0x400],    // 0x0500_0000 - 0x0500_03FF (1 KB)
    pub vram: [u8; 0x18000],         // 0x0600_0000 - 0x0601_7FFF (96 KB)
    pub oam: [u8; 0x400],            // 0x0700_0000 - 0x0700_03FF (1 KB)
    
    cartridge: Option<Box<Cartridge>>,
    
    // Input register
    keyinput: u16,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            bios: [0u8; 0x4000],
            on_board_wram: [0u8; 0x40000],
            on_chip_wram: [0u8; 0x8000],
            io_registers: [0u8; 0x400],
            palette_ram: [0u8; 0x400],
            vram: [0u8; 0x18000],
            oam: [0u8; 0x400],
            cartridge: None,
            keyinput: 0x3FF,
        }
    }

    pub fn load_cartridge(&mut self, cartridge: &Cartridge) {
        self.cartridge = Some(Box::new(cartridge.clone()));
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        match address {
            0x0000_0000..=0x0000_3FFF => self.bios[address as usize],
            0x0200_0000..=0x0203_FFFF => self.on_board_wram[(address - 0x0200_0000) as usize],
            0x0300_0000..=0x0300_7FFF => self.on_chip_wram[(address - 0x0300_0000) as usize],
            0x0400_0000..=0x0400_03FF => self.io_registers[(address - 0x0400_0000) as usize],
            0x0500_0000..=0x0500_03FF => self.palette_ram[(address - 0x0500_0000) as usize],
            0x0600_0000..=0x0601_7FFF => self.vram[(address - 0x0600_0000) as usize],
            0x0700_0000..=0x0700_03FF => self.oam[(address - 0x0700_0000) as usize],
            0x0800_0000..=0x0DFF_FFFF => {
                if let Some(ref cartridge) = self.cartridge {
                    cartridge.read_byte(address)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            0x0000_0000..=0x0000_3FFF => {
                // BIOS is read-only
            }
            0x0200_0000..=0x0203_FFFF => {
                self.on_board_wram[(address - 0x0200_0000) as usize] = value;
            }
            0x0300_0000..=0x0300_7FFF => {
                self.on_chip_wram[(address - 0x0300_0000) as usize] = value;
            }
            0x0400_0000..=0x0400_03FF => {
                let offset = (address - 0x0400_0000) as usize;
                self.io_registers[offset] = value;
                self.handle_io_write(address, value);
            }
            0x0500_0000..=0x0500_03FF => {
                self.palette_ram[(address - 0x0500_0000) as usize] = value;
            }
            0x0600_0000..=0x0601_7FFF => {
                self.vram[(address - 0x0600_0000) as usize] = value;
            }
            0x0700_0000..=0x0700_03FF => {
                self.oam[(address - 0x0700_0000) as usize] = value;
            }
            _ => {}
        }
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, ((value >> 8) & 0xFF) as u8);
    }

    pub fn read_word(&self, address: u32) -> u32 {
        let low = self.read_halfword(address) as u32;
        let high = self.read_halfword(address + 2) as u32;
        (high << 16) | low
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        self.write_halfword(address, (value & 0xFFFF) as u16);
        self.write_halfword(address + 2, ((value >> 16) & 0xFFFF) as u16);
    }

    fn handle_io_write(&mut self, _address: u32, _value: u8) {
        // Handle special I/O register writes (interrupts, DMA, timers, etc.)
    }

    pub fn set_input(&mut self, input: u16) {
        self.keyinput = input;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
