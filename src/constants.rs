use stm32g4xx_hal::gpio::{
    Analog, Input, PA0, PA1, PA10, PA4, PA8, PA9, PB10, PB4, PB5, PB6, PB7, PC0, PC1, PC2,
};

// Pin mapping. These aliases make `Controls::new(...)` require the expected pins.
pub type LeftXPin = PA0<Analog>;
pub type LeftYPin = PA1<Analog>;
pub type RightXPin = PA4<Analog>;
pub type RightYPin = PC2<Analog>;
pub type LeftTriggerPin = PC1<Analog>;
pub type RightTriggerPin = PC0<Analog>;

pub type APin = PB5<Input>;
pub type BPin = PB4<Input>;
pub type XPin = PB10<Input>;
pub type YPin = PA8<Input>;
pub type StartPin = PA9<Input>;
pub type BackPin = PA10<Input>;
pub type LeftBumperPin = PB6<Input>;
pub type RightBumperPin = PB7<Input>;

// USB Device IDs
pub const VENDOR_ID: u16 = 0x045e;
pub const PRODUCT_ID: u16 = 0x028e;

// USB Device Class / SubClass / Protocol
pub const DEVICE_CLASS: u8 = 0xff;
pub const DEVICE_SUB_CLASS: u8 = 0xff;
pub const DEVICE_PROTOCOL: u8 = 0xff;

// USB Device Configuration
pub const MAX_PACKET_SIZE_0: u8 = 8;
pub const DEVICE_RELEASE: u16 = 0x0114;
pub const MAX_POWER_MA: usize = 500;

// USB Strings
pub const MANUFACTURER: &str = "©Microsoft Corporation";
pub const PRODUCT: &str = "Controller";
pub const SERIAL_NUMBER: &str = "00000001";

// Security String
pub const SECURITY_STRING: &str =
    "Xbox Security Method 3, Version 1.00, © 2005 Microsoft Corporation. All rights reserved.";

// Endpoint Addresses
pub const EP1_IN_ADDR: u8 = 0x81;
pub const EP1_OUT_ADDR: u8 = 0x01;
pub const EP2_IN_ADDR: u8 = 0x82;
pub const EP2_OUT_ADDR: u8 = 0x02;
pub const EP3_IN_ADDR: u8 = 0x83;
pub const EP3_OUT_ADDR: u8 = 0x03;
pub const EP4_IN_ADDR: u8 = 0x84;

// Endpoint Configuration
pub const EP_PACKET_SIZE: u16 = 32;
pub const EP1_IN_POLL_INTERVAL: u8 = 4;
pub const EP1_OUT_POLL_INTERVAL: u8 = 8;
pub const EP2_IN_POLL_INTERVAL: u8 = 2;
pub const EP2_OUT_POLL_INTERVAL: u8 = 4;
pub const EP3_IN_POLL_INTERVAL: u8 = 64;
pub const EP3_OUT_POLL_INTERVAL: u8 = 16;
pub const EP4_IN_POLL_INTERVAL: u8 = 16;

// Interface Class / SubClass / Protocol
pub const IF0_CLASS: u8 = 0xff;
pub const IF0_SUB_CLASS: u8 = 0x5d;
pub const IF0_PROTOCOL: u8 = 0x01;
pub const IF1_CLASS: u8 = 0xff;
pub const IF1_SUB_CLASS: u8 = 0x5d;
pub const IF1_PROTOCOL: u8 = 0x03;
pub const IF2_CLASS: u8 = 0xff;
pub const IF2_SUB_CLASS: u8 = 0x5d;
pub const IF2_PROTOCOL: u8 = 0x02;
pub const IF3_CLASS: u8 = 0xff;
pub const IF3_SUB_CLASS: u8 = 0xfd;
pub const IF3_PROTOCOL: u8 = 0x13;
pub const IF3_ALT_SETTING: u8 = 0;

// Descriptor Types
pub const DESC_TYPE_XBOX: u8 = 0x21;
pub const DESC_TYPE_SECURITY: u8 = 0x41;

// XInput packet timing/sizes
pub const SEND_PERIOD_MS: u32 = 4;
pub const XINPUT_REPORT_SIZE: usize = 20;
pub const XINPUT_OUT_PACKET_SIZE: usize = 32;

// XInput button bit masks
pub const START: u8 = 1 << 4;
pub const BACK: u8 = 1 << 5;

pub const LEFT_BUMPER: u8 = 1 << 0;
pub const RIGHT_BUMPER: u8 = 1 << 1;
pub const A: u8 = 1 << 4;
pub const B: u8 = 1 << 5;
pub const X: u8 = 1 << 6;
pub const Y: u8 = 1 << 7;

// Controller config
pub const ADC_MIN: u16 = 0;
pub const ADC_CENTER: u16 = 2048;
pub const ADC_MAX: u16 = 4095;

pub const LEFT_X_INVERT: bool = false;
pub const LEFT_Y_INVERT: bool = true;
pub const RIGHT_X_INVERT: bool = false;
pub const RIGHT_Y_INVERT: bool = true;

// Descriptor Data
pub const IF0_DESCRIPTOR: &[u8] = &[
    0x00, 0x01, 0x01, 0x25, 0x81, 0x14, 0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x08, 0x00, 0x00,
];
pub const IF1_DESCRIPTOR: &[u8] = &[
    0x00, 0x01, 0x01, 0x01, 0x82, 0x40, 0x01, 0x02, 0x20, 0x16, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x16, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const IF2_DESCRIPTOR: &[u8] = &[0x00, 0x01, 0x01, 0x22, 0x84, 0x07, 0x00];
pub const IF3_DESCRIPTOR: &[u8] = &[0x00, 0x01, 0x01, 0x03];
