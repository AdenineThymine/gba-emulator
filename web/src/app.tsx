import React, { useEffect, useRef, useState } from 'react';
import { GBAEmulator } from './wasm';
import './app.css';

export function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const emulatorRef = useRef<GBAEmulator | null>(null);
  const animationRef = useRef<number | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [romLoaded, setRomLoaded] = useState(false);

  useEffect(() => {
    emulatorRef.current = new GBAEmulator();
  }, []);

  const handleRomUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file || !emulatorRef.current) return;

    const arrayBuffer = await file.arrayBuffer();
    const romData = new Uint8Array(arrayBuffer);

    try {
      emulatorRef.current.load_rom(romData);
      setRomLoaded(true);
      setIsRunning(false);
    } catch (error) {
      console.error('Failed to load ROM:', error);
      alert('Failed to load ROM file');
    }
  };

  const startEmulation = () => {
    if (!canvasRef.current || !emulatorRef.current || !romLoaded) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    setIsRunning(true);

    const run = () => {
      const emulator = emulatorRef.current;
      if (!emulator) return;

      // Run one frame
      emulator.run_frame();

      // Get framebuffer and render
      const framebuffer = emulator.get_framebuffer();
      const imageData = ctx.createImageData(240, 160);
      imageData.data.set(framebuffer);
      ctx.putImageData(imageData, 0, 0);

      animationRef.current = requestAnimationFrame(run);
    };

    animationRef.current = requestAnimationFrame(run);
  };

  const stopEmulation = () => {
    setIsRunning(false);
    if (animationRef.current !== null) {
      cancelAnimationFrame(animationRef.current);
      animationRef.current = null;
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    // Map keyboard keys to GBA buttons
    // A=Z, B=X, Select=Backspace, Start=Enter, etc.
    const keyMap: { [key: string]: number } = {
      'z': 0, // A
      'x': 1, // B
      'a': 2, // L
      's': 3, // R
      'ArrowUp': 4,
      'ArrowDown': 5,
      'ArrowLeft': 6,
      'ArrowRight': 7,
      'Backspace': 8, // Select
      'Enter': 9, // Start
    };

    const buttonBit = keyMap[e.key];
    if (buttonBit !== undefined && emulatorRef.current) {
      // Send input to emulator
      console.log('Key pressed:', e.key);
    }
  };

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  return (
    <div className="app">
      <h1>GBA Emulator</h1>
      <div className="controls">
        <label htmlFor="rom-upload">Load ROM:</label>
        <input
          id="rom-upload"
          type="file"
          accept=".gba,.bin"
          onChange={handleRomUpload}
          disabled={isRunning}
        />
        <button onClick={startEmulation} disabled={!romLoaded || isRunning}>
          Start
        </button>
        <button onClick={stopEmulation} disabled={!isRunning}>
          Stop
        </button>
      </div>
      <canvas
        ref={canvasRef}
        width={240}
        height={160}
        className="screen"
      />
      <div className="info">
        <p>Use arrow keys to move, Z/X for A/B buttons, Enter for Start</p>
      </div>
    </div>
  );
}
