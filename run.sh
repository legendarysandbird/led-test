#!/bin/bash

rustup override set nightly-2021-01-07
rustup component add rust-src
export AVR_CPU_FREQUENCY_HZ=16000000

cargo build -Z build-std=core --target avr-atmega328p.json --release && avrdude -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/blink.elf:e
