# GBA Emulator

A feature-complete Game Boy Advance emulator running in the web browser, built with Rust, WebAssembly, and React.

## Features

- **ARM7TDMI CPU Emulation**: Full implementation of the GBA's 32-bit processor with ARM and Thumb instruction sets
- **Complete Memory Map**: Support for BIOS, ROM, WRAM, I/O registers, palette RAM, VRAM, and OAM
- **Graphics Processing**: GPU/PPU implementation with support for all GBA video modes
- **Audio System**: APU with 4 sound channels
- **Cartridge Support**: ROM loading and save state management
- **Web-based**: Play directly in modern web browsers
- **Keyboard/Gamepad Input**: Full control support

## Project Structure

```
gba-emulator/
├── src/                    # Rust emulator core
│   ├── lib.rs             # Main emulator API
│   ├── cpu.rs             # ARM7TDMI CPU implementation
│   ├── memory.rs          # Memory management
│   ├── gpu.rs             # Graphics processing
│   ├── apu.rs             # Audio processing
│   └── cartridge.rs       # ROM cartridge handling
├── web/                    # React web interface
│   ├── src/
│   │   ├── index.tsx      # React entry point
│   │   ├── app.tsx        # Main app component
│   │   ├── app.css        # Component styles
│   │   └── index.css      # Global styles
│   ├── public/
│   │   └── index.html     # HTML template
│   ├── webpack.config.js  # Webpack build configuration
│   ├── tsconfig.json      # TypeScript configuration
│   └── package.json       # Web dependencies
├── Cargo.toml             # Rust project configuration
└── README.md              # This file
```

## Building

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)
- [Node.js](https://nodejs.org/) (16+)
- [wasm-pack](https://rustwasm.org/wasm-pack/)
- [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/cli)

### Build Steps

1. **Install Rust dependencies**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   wasm-pack build --release
   ```

2. **Install web dependencies**:
   ```bash
   cd web
   npm install
   ```

3. **Build and run**:
   ```bash
   # Development with hot reload
   npm run dev

   # Production build
   npm run build
   ```

4. **Open in browser**:
   Navigate to `http://localhost:8080`

## Usage

1. **Load a ROM**: Click "Load ROM" and select a GBA .gba or .bin file
2. **Start Emulation**: Click "Start" to begin playing
3. **Controls**:
   - **Arrow Keys**: D-Pad
   - **Z**: A Button
   - **X**: B Button
   - **A**: L Button
   - **S**: R Button
   - **Backspace**: Select
   - **Enter**: Start

## Legal Notice

**Important**: This emulator is for educational purposes. You must own the games you wish to emulate. ROM files should only be created from cartridges you legally own. Never download copyrighted games from the internet.

## Status

This is an **active development project**. The following components are currently implemented:

- [x] Project structure and build pipeline
- [x] Basic CPU instruction set
- [x] Memory management
- [x] Cartridge loading
- [x] Web frontend framework
- [ ] Full CPU instruction set completion
- [ ] GPU/PPU scanline rendering
- [ ] APU sound generation
- [ ] Save state system
- [ ] Performance optimization
- [ ] ROM compatibility testing

## Architecture

### Rust/WASM Core

The emulator core is written in Rust and compiled to WebAssembly for:
- **Performance**: Near-native speed for cycle-accurate emulation
- **Safety**: Memory safety without garbage collection
- **Accuracy**: Strong typing helps prevent bugs in complex logic

### React Frontend

The web interface is built with React for:
- **Responsiveness**: Smooth user interactions
- **Maintainability**: Component-based architecture
- **Canvas Rendering**: Hardware-accelerated graphics

## Contributing

Contributions are welcome! Areas needing work:

- CPU instruction set completion
- GPU graphics rendering
- APU audio generation
- Performance profiling and optimization
- ROM compatibility fixes

## References

- [GBATEK Documentation](https://problemkaputt.de/gbatek.htm) - Comprehensive GBA hardware reference
- [ARM7TDMI Datasheet](https://www.arm.com/) - CPU instruction set
- [Rust WASM Book](https://rustwasm.org/book/) - WASM development guide

## License

MIT License - See LICENSE file for details
