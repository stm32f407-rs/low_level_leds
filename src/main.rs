#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use volatile_register::{RW, RO, WO};
use core::ptr;

#[repr(C)]
struct Gpio {
    pub moder: RW<u32>,
    pub otyper: RW<u32>,
    pub ospeedr: RW<u32>,
    pub pupdr: RW<u32>,
    pub idr: RO<u32>,
    pub odr: RW<u32>, // output data register
    pub bsrr: WO<u32>, // output data register
    pub lckr: RW<u32>, // output data register
    pub afrl: RW<u32>, // output data register
    pub afrh: RW<u32>, // output data register
}


#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    // enable GPIOD
    const RCC_GPIO: *mut u32 = (0x4002_3800 + 0x30) as *mut u32;
    unsafe {
        ptr::write_volatile(RCC_GPIO, 0b1000);
    }

    let gpio_d = 0x4002_0C00 as *const Gpio;
    unsafe {
        (*gpio_d).moder.write(0x5500_0000);

        // write 1 to leds
        (*gpio_d).bsrr.write(0xF000);

        // you can write 1 also with odr register:
        // (*gpio_d).odr.write(0xF000);
    }

    loop {
        // do nothing
    }
}
