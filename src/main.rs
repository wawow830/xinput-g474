#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_probe as _;

use stm32g4xx_hal as hal;

use usb_device::{device::UsbRev, prelude::*};

use hal::{
    adc::{AdcClaim, AdcCommonExt},
    prelude::*,
    pwr::PwrExt,
    rcc, stm32,
    timer::MonoTimer,
    usb::{Peripheral, UsbBus},
};

mod constants;
use constants::*;

mod controls;
mod xinput;
use controls::Controls;
use xinput::XInput;

#[entry]
fn main() -> ! {
    let peripherals = stm32::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let pwr = peripherals.PWR.constrain().freeze();
    let mut rcc = peripherals.RCC.freeze(rcc::Config::hsi(), pwr);

    rcc.enable_hsi48();

    let mono = MonoTimer::new(core.DWT, core.DCB, &rcc.clocks);
    let send_period_cycles = (mono.frequency().raw() / 1000) * SEND_PERIOD_MS;
    let mut last_send = mono.now();

    let mut delay = core.SYST.delay(&rcc.clocks);
    let adc12_common = peripherals.ADC12_COMMON.claim(Default::default(), &mut rcc);
    let adc = adc12_common.claim_and_configure(
        peripherals.ADC2,
        hal::adc::config::AdcConfig::default(),
        &mut delay,
    );

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let gpiob = peripherals.GPIOB.split(&mut rcc);
    let gpioc = peripherals.GPIOC.split(&mut rcc);

    let left_x = gpioa.pa0.into_analog();
    let left_y = gpioa.pa1.into_analog();
    let right_x = gpioa.pa4.into_analog();
    let right_y = gpioc.pc2.into_analog();
    let left_trigger = gpioc.pc1.into_analog();
    let right_trigger = gpioc.pc0.into_analog();

    let a = gpiob.pb5.into_pull_up_input();
    let b = gpiob.pb4.into_pull_up_input();
    let x = gpiob.pb10.into_pull_up_input();
    let y = gpioa.pa8.into_pull_up_input();
    let start = gpioa.pa9.into_pull_up_input();
    let back = gpioa.pa10.into_pull_up_input();
    let left_bumper = gpiob.pb6.into_pull_up_input();
    let right_bumper = gpiob.pb7.into_pull_up_input();

    let usb_dm = gpioa.pa11.into_alternate(); // D-
    let usb_dp = gpioa.pa12.into_alternate(); // D+

    let usb = Peripheral {
        usb: peripherals.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

    let usb_bus = UsbBus::new(usb);

    let mut controls = Controls::new(
        adc,
        left_x,
        left_y,
        right_x,
        right_y,
        left_trigger,
        right_trigger,
        a,
        b,
        x,
        y,
        start,
        back,
        left_bumper,
        right_bumper,
    );
    let mut xinput = XInput::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(VENDOR_ID, PRODUCT_ID))
        .device_class(DEVICE_CLASS)
        .device_sub_class(DEVICE_SUB_CLASS)
        .device_protocol(DEVICE_PROTOCOL)
        .usb_rev(UsbRev::Usb200)
        .max_packet_size_0(MAX_PACKET_SIZE_0)
        .unwrap()
        .device_release(DEVICE_RELEASE)
        .supports_remote_wakeup(true)
        .max_power(MAX_POWER_MA)
        .unwrap()
        .strings(&[StringDescriptors::default()
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(SERIAL_NUMBER)])
        .unwrap()
        .build();

    loop {
        usb_dev.poll(&mut [&mut xinput]);

        if last_send.elapsed() >= send_period_cycles {
            let state = controls.read();
            let packet = xinput::package(&state);
            let _ = xinput.send(&packet);
            last_send = mono.now();
        }
    }
}
