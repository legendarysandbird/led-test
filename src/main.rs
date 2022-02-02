#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

#[no_mangle]
pub extern fn main() {
    port::D6::set_output();
    port::D5::set_output();
    port::D3::set_output();

    loop {
        port::D3::set_low();
        port::D6::set_high();

        ruduino::delay::delay_ms(1000);

        port::D6::set_low();
        port::D5::set_high();

        ruduino::delay::delay_ms(1000);

        port::D5::set_low();
        port::D3::set_high();

        ruduino::delay::delay_ms(1000);
    }
}
