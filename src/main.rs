
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f3xx_hal::{
    pac,
    prelude::*,
    delay::Delay,
};


#[panic_handler]
fn panic( info: &core::panic::PanicInfo) -> ! {
    loop {
        continue;
    }
}

#[entry]
fn main() -> ! {
    loop {
        // TODO: implement blinking LED
        let dp = pac::Peripherals::take().unwrap();
    }
}

