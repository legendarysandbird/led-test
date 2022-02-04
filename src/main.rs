#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

enum Color {
    Red,
    Green,
    Blue,
}

#[no_mangle]
pub extern fn main() {
    port::D6::set_output();
    port::D5::set_output();
    port::D3::set_output();

    port::B5::set_input();

    let mut just_pressed = false;
    let mut color: Color = Color::Red;

    port::D3::set_low();
    port::D6::set_high();

    loop {

        if port::B5::is_low() && !just_pressed {
            just_pressed = true;
        } else if port::B5::is_high() {
            just_pressed = false;
        }


        if just_pressed {
            match color {
                Color::Red => {
                    color = Color::Green;
                    port::D6::set_low();
                    port::D5::set_high();
                },
                Color::Green => {
                    color = Color::Blue;
                    port::D5::set_low();
                    port::D3::set_high();
                },
                Color::Blue => {
                    color = Color::Red;
                    port::D3::set_low();
                    port::D6::set_high();
                },
            }
            ruduino::delay::delay_ms(500);
        }
    }
}
