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

mod constants;
use constants::*;

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
    };

    let usb_bus = UsbBus::new(usb);

    let mut xinput = XInputClass::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(VENDOR_ID, PRODUCT_ID))
        .device_class(DEVICE_CLASS)
        .device_sub_class(DEVICE_SUB_CLASS)
        .device_protocol(DEVICE_PROTOCOL)
        .usb_rev(UsbRev::Usb200)
        .max_packet_size_0(MAX_PACKET_SIZE_0).unwrap()
        .device_release(DEVICE_RELEASE)
        .supports_remote_wakeup(true)
        .max_power(MAX_POWER_MA).unwrap()
        .strings(&[StringDescriptors::default()
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(SERIAL_NUMBER)])
        .unwrap()
        .build();

    loop {
        usb_dev.poll(&mut [&mut xinput]);
    }
}
