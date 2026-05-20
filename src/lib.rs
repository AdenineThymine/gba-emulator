use wasm_bindgen::prelude::*;

pub mod cpu;
pub mod memory;
pub mod gpu;
pub mod apu;
pub mod cartridge;

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::gpu::GPU;
use crate::apu::APU;
use crate::cartridge::Cartridge;

#[wasm_bindgen]
pub struct GBAEmulator {
    cpu: CPU,
    memory: Memory,
    gpu: GPU,
    apu: APU,
    cartridge: Option<Cartridge>,
}

#[wasm_bindgen]
impl GBAEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GBAEmulator {
        GBAEmulator {
            cpu: CPU::new(),
            memory: Memory::new(),
            gpu: GPU::new(),
            apu: APU::new(),
            cartridge: None,
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), String> {
        self.cartridge = Some(Cartridge::new(rom_data.to_vec())?);
        self.memory.load_cartridge(self.cartridge.as_ref().unwrap());
        Ok(())
    }

    pub fn step(&mut self) {
        // Execute one CPU cycle
        self.cpu.step(&mut self.memory);
    }

    pub fn run_frame(&mut self) {
        // Run for one frame (~280,896 CPU cycles at 16.78 MHz)
        for _ in 0..280_896 {
            self.step();
        }
    }

    pub fn get_framebuffer(&self) -> Vec<u8> {
        self.gpu.get_framebuffer()
    }

    pub fn handle_input(&mut self, input: u16) {
        self.memory.set_input(input);
    }
}

impl Default for GBAEmulator {
    fn default() -> Self {
        Self::new()
    }
}
