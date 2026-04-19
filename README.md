# chip8-rs

A CHIP-8 emulator written in Rust.

## What is CHIP-8?

CHIP-8 is an interpreted virtual machine from the 1970s, originally designed to run on microcomputers like the COSMAC VIP. It has a 64×32 monochrome display, 16 general-purpose registers, 35 opcodes, and a 16-key hexadecimal keypad. It's one of the classic first emulator projects.

## Features

- Full implementation of all 35 CHIP-8 opcodes
- 64×32 display rendered in a native window via [minifb](https://github.com/emoon/rust_minifb)
- Delay and sound timers running on a background thread
- Configurable CPU clock rate
- Pluggable renderer — the display callback is fully decoupled from the emulator core
- Command-line interface with ROM path, clock speed, and debug logging flags

## Requirements

- Rust (stable)
- Cargo

## Building

```bash
git clone https://github.com/yourname/chip8-rs
cd chip8-rs
cargo build --release
```

## Usage

```bash
cargo run --release -- <rom_path> [--clock <hz>] [--log]
```

### Arguments

| Argument | Description | Default |
|---|---|---|
| `<rom_path>` | Path to a `.ch8` ROM file | required |
| `--clock <hz>` | CPU clock speed in Hz | 600 |
| `--log` | Enable debug logging of CPU state each cycle | off |

### Examples

```bash
# Run a ROM at default speed
cargo run --release -- roms/spaceinvaders.ch8

# Run at a higher clock rate
cargo run --release -- roms/spaceinvaders.ch8 --clock 1000

# Run with debug logging
cargo run --release -- roms/test.ch8 --log
```

## Keypad Mapping

CHIP-8 uses a 16-key hexadecimal keypad. The mapping to a standard keyboard is:

```
CHIP-8    Keyboard
1 2 3 C   1 2 3 4
4 5 6 D   Q W E R
7 8 9 E   A S D F
A 0 B F   Z X C V
```

## Project Structure

```
src/
  main.rs         — entry point, window setup, CLI argument parsing
  chip8.rs        — Chip8 struct, fetch/decode/execute loop
  instructions.rs — all 35 opcode implementations
  utils.rs        — Hertz clock helper, Timer with background thread
```

## ROMs

CHIP-8 ROMs are widely available online. A good starting point is the [chip8-roms](https://github.com/kripod/chip8-roms) repository which includes classic games like Space Invaders, Tetris, and Pong.

## License

MIT
