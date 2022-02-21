# stm32f411 Rust Blinky LED

A Rust microcrontroller "hello, world" for the STM32f411 "Black Pill" board.

These are my first steps learning embedded Rust, so my confidence on correctly setting
everything up here is fairly low, however at least as of Feb 2022, this project setup
was working for me.

I hope this repo is useful to you as you get started with embedded Rust.

## Steps

- clone this repo
- Plug your STLink-v2 into your computer with the headers connected to the 4 pins of the Black Pill board. Match up GND to GND, SWSCK to SWSCK, etc.
- `cargo install probe-run"
- `cargo install cargo-embed`

## Building and Flashing

You should be able to build using `cargo build`, and to flash the board using `cargo embed`.
After flashing you should see "hello world" printed to the terminal every 2 seconds.
