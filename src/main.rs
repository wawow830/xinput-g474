#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_probe as _;

use stm32g4xx_hal as hal;

use hal::{
    prelude::*,
    pwr::PwrExt,
    rcc:Config,
    stm32,
    usb::{Peripheral, UsbBus},
    time::ExtU32,
};

#[entry]
fn main() -> ! {
}
