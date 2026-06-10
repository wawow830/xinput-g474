#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_probe as _;

use stm32g4xx_hal as hal;

use hal::{
    prelude::*,
    pwr::PwrExt,
    rcc,
    stm32,
    usb::{Peripheral, UsbBus},
};

#[entry]
fn main() -> ! {
    let peripherals = stm32::Peripherals::take().unwrap();

    let pwr = peripherals.PWR.constrain().freeze();
    let mut rcc = peripherals.RCC.freeze(rcc::Config::hsi(), pwr);

    rcc.enable_hsi48();

    let gpioa = peripherals.GPIOA.split(&mut rcc);

    let usb_dm = gpioa.pa11.into_alternate(); // D-
    let usb_dp = gpioa.pa12.into_alternate(); // D+

    let usb = Peripheral {
        usb: peripherals.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    }

    let usb_bus = UsbBus::new(usb);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x045e, 0x028e))
    .device_class(0xff)
    .device_sub_class(0xff)
    .device_protocol(0xff)
    .usb_rev(UsbRev::Usb200)
    .max_packet_size_0(8).unwrap()
    .device_release(0x0114)
    .supports_remote_wakeup(true)
    .max_power(500).unwrap()
    .strings(&[StringDescriptors::default()
       .manufacturer("©Microsoft Corporation")
       .product("Controller")
       .serial_number("00000001")])
    .unwrap()
    .build()

    loop {

    }
}
