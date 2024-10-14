# TunePulseRs

This is a Rust firmware for the Creapunk CLN17 v2.0 stepper motor driver.

## Features

It prints hello world (for now)

## Development

### Prerequisites

- [Rustup](https://rustup.rs)
- [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html) ST-Link drivers

### Setup

1. Make sure you are using nightly Rust:

   ```sh
   rustup install nightly
   ```

2. Install the `armv7em-none-eabihf` target:

   ```sh
   rustup target add armv7em-none-eabihf
   ```

3. Install the `probe-rs` CLI tool:

   ```sh
   cargo install cargo-binstall
   cargo binstall probe-rs-tools
   ```

### Flashing/Running

Connect the ST-Link port to computer, and connect the port on the other end to the CLN17 board.

Don't connect the device port (the one next to the ST-Link port) to anything.

To flash the firmware and run it, use:

```sh
cargo run
```

Serial output will automatically be streamed to the terminal.

**Make sure the code always contains an infinite loop somewhere, otherwise you will have to press the reset button on the board in order to flash it again.**

You have to hold down reset until executing `cargo run`. If you try to hold reset during flashing, it won't work.
