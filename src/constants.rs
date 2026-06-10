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
pub const MAX_POWER_MA: u16 = 500;

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

// Descriptor Data
pub const IF0_DESCRIPTOR: &[u8] = &[
    0x00, 0x01, 0x01, 0x25, 0x81, 0x14, 0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x08, 0x00, 0x00,
];
pub const IF1_DESCRIPTOR: &[u8] = &[
    0x00, 0x01, 0x01, 0x01, 0x82, 0x40, 0x01, 0x02, 0x20, 0x16, 0x83, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x16, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const IF2_DESCRIPTOR: &[u8] = &[0x00, 0x01, 0x01, 0x22, 0x84, 0x07, 0x00];
pub const IF3_DESCRIPTOR: &[u8] = &[0x00, 0x01, 0x01, 0x03];
