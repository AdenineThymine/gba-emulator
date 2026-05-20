pub struct GPU {
    framebuffer: [u8; 240 * 160 * 4], // RGBA for all pixels
    pub scanline: u16,
    pub cycle: u32,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            framebuffer: [0u8; 240 * 160 * 4],
            scanline: 0,
            cycle: 0,
        }
    }

    pub fn get_framebuffer(&self) -> Vec<u8> {
        self.framebuffer.to_vec()
    }

    pub fn update(&mut self) {
        // Increment cycle counter
        self.cycle = self.cycle.wrapping_add(1);
        
        // GBA has 308 cycles per scanline
        if self.cycle >= 308 {
            self.cycle = 0;
            self.scanline = (self.scanline + 1) % 228;
        }

        // Render visible scanlines (0-159)
        if self.scanline < 160 {
            self.render_scanline(self.scanline);
        }
    }

    fn render_scanline(&mut self, _scanline: u16) {
        // Stub: Implement scanline rendering
        // This would involve:
        // 1. Reading background/sprite data from VRAM
        // 2. Applying palette lookups
        // 3. Blending if necessary
        // 4. Writing to framebuffer
    }
}

impl Default for GPU {
    fn default() -> Self {
        Self::new()
    }
}
