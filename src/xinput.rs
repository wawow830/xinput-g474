use usb_device::class_prelude::*;

use crate::{constants::*, controls::State};

pub struct Packet {
    pub bytes: [u8; XINPUT_REPORT_SIZE],
}

pub fn package(state: &State) -> Packet {
    let mut bytes = [0; XINPUT_REPORT_SIZE];

    bytes[0] = 0x00;
    bytes[1] = XINPUT_REPORT_SIZE as u8;
    bytes[2] = state.buttons_lo;
    bytes[3] = state.buttons_hi;
    bytes[4] = state.left_trigger;
    bytes[5] = state.right_trigger;
    bytes[6..8].copy_from_slice(&state.left_x.to_le_bytes());
    bytes[8..10].copy_from_slice(&state.left_y.to_le_bytes());
    bytes[10..12].copy_from_slice(&state.right_x.to_le_bytes());
    bytes[12..14].copy_from_slice(&state.right_y.to_le_bytes());
    bytes[14] = 0x00;
    bytes[15] = 0x00;
    bytes[16] = 0x00;
    bytes[17] = 0x00;
    bytes[18] = 0x00;
    bytes[19] = 0x00;

    Packet { bytes }
}

pub struct XInput<'a, B: UsbBus> {
    if0: InterfaceNumber,
    if1: InterfaceNumber,
    if2: InterfaceNumber,
    if3: InterfaceNumber,

    security_string: StringIndex,

    ep_in: EndpointIn<'a, B>,
    ep_out: EndpointOut<'a, B>,

    ep2_in: EndpointIn<'a, B>,
    ep2_out: EndpointOut<'a, B>,

    ep3_in: EndpointIn<'a, B>,
    ep3_out: EndpointOut<'a, B>,

    ep4_in: EndpointIn<'a, B>,
}

impl<'a, B: UsbBus> XInput<'a, B> {
    pub fn new(alloc: &'a UsbBusAllocator<B>) -> Self {
        Self {
            if0: alloc.interface(),
            if1: alloc.interface(),
            if2: alloc.interface(),
            if3: alloc.interface(),

            security_string: alloc.string(),

            ep_in: alloc
                .alloc(
                    Some(EP1_IN_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP1_IN_POLL_INTERVAL,
                )
                .unwrap(),
            ep_out: alloc
                .alloc(
                    Some(EP1_OUT_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP1_OUT_POLL_INTERVAL,
                )
                .unwrap(),

            ep2_in: alloc
                .alloc(
                    Some(EP2_IN_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP2_IN_POLL_INTERVAL,
                )
                .unwrap(),
            ep2_out: alloc
                .alloc(
                    Some(EP2_OUT_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP2_OUT_POLL_INTERVAL,
                )
                .unwrap(),

            ep3_in: alloc
                .alloc(
                    Some(EP3_IN_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP3_IN_POLL_INTERVAL,
                )
                .unwrap(),
            ep3_out: alloc
                .alloc(
                    Some(EP3_OUT_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP3_OUT_POLL_INTERVAL,
                )
                .unwrap(),

            ep4_in: alloc
                .alloc(
                    Some(EP4_IN_ADDR.into()),
                    EndpointType::Interrupt,
                    EP_PACKET_SIZE,
                    EP4_IN_POLL_INTERVAL,
                )
                .unwrap(),
        }
    }

    pub fn send(&mut self, packet: &Packet) -> usb_device::Result<()> {
        match self.ep_in.write(&packet.bytes) {
            Ok(_) | Err(UsbError::WouldBlock) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl<B: UsbBus> UsbClass<B> for XInput<'_, B> {
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface(self.if0, IF0_CLASS, IF0_SUB_CLASS, IF0_PROTOCOL)?;
        writer.write(DESC_TYPE_XBOX, IF0_DESCRIPTOR)?;
        writer.endpoint(&self.ep_in)?;
        writer.endpoint(&self.ep_out)?;

        writer.interface(self.if1, IF1_CLASS, IF1_SUB_CLASS, IF1_PROTOCOL)?;
        writer.write(DESC_TYPE_XBOX, IF1_DESCRIPTOR)?;
        writer.endpoint(&self.ep2_in)?;
        writer.endpoint(&self.ep2_out)?;
        writer.endpoint(&self.ep3_in)?;
        writer.endpoint(&self.ep3_out)?;

        writer.interface(self.if2, IF2_CLASS, IF2_SUB_CLASS, IF2_PROTOCOL)?;
        writer.write(DESC_TYPE_XBOX, IF2_DESCRIPTOR)?;
        writer.endpoint(&self.ep4_in)?;

        writer.interface_alt(
            self.if3,
            IF3_ALT_SETTING,
            IF3_CLASS,
            IF3_SUB_CLASS,
            IF3_PROTOCOL,
            Some(self.security_string),
        )?;
        writer.write(DESC_TYPE_SECURITY, IF3_DESCRIPTOR)?;

        Ok(())
    }

    fn get_string(&self, index: StringIndex, _lang_id: LangID) -> Option<&str> {
        if index == self.security_string {
            Some(SECURITY_STRING)
        } else {
            None
        }
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        let mut buf = [0; XINPUT_OUT_PACKET_SIZE];

        if addr == self.ep_out.address() {
            let _ = self.ep_out.read(&mut buf);
        } else if addr == self.ep2_out.address() {
            let _ = self.ep2_out.read(&mut buf);
        } else if addr == self.ep3_out.address() {
            let _ = self.ep3_out.read(&mut buf);
        }
    }
}
