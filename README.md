# stm32f411 Rust Blinky LED w/ Interrupts

A Rust microcrontroller minimal interrupt example for the STM32f411 "Black Pill" board.

![Blinking LED photo of the board](https://user-images.githubusercontent.com/175496/154872263-3b432685-2bfe-4557-bd3e-8ba7c4261147.jpg)

These are my first steps learning embedded Rust, so my confidence on correctly setting
everything up here is fairly low, however at least as of Feb 2022, this project setup
was working for me.

I hope this repo is useful to you as you get started with embedded Rust.

## Steps

- clone this repo
- Plug your STLink-v2 into your computer with the headers connected to the 4 pins of the Black Pill board. Match up GND to GND, SWSCK to SWSCK, etc.
- `cargo install probe-run`
- `cargo install cargo-embed`

## Building and Flashing

You should be able to build using `cargo build`, and to flash the board using `cargo embed`.
After flashing you should see "registered interupt!" printed to the terminal, and pressing the built-in user button should light up the built-in LED.
