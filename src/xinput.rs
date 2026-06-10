use usb_device::class_prelude::*;

pub struct XInput<'a, B: UsbBus> {
    if0: InterfaceNumber,
    if1: InterfaceNumber,
    if2: InterfaceNumber,
    if3: InterfaceNumber,

    security_string: StringIndex,

    ep_in:   EndpointIn<'a, B>,
    ep_out:  EndpointOut<'a, B>,

    ep2_in:  EndpointIn<'a, B>,
    ep2_out: EndpointOut<'a, B>,

    ep3_in:  EndpointIn<'a, B>,
    ep3_out: EndpointOut<'a, B>,

    ep4_in:  EndpointIn<'a, B>,
}

impl<'a, B: UsbBus> XInput<'a, B> {
    pub fn new(alloc: &'a UsbBusAllocator<B>) -> Self {
        Self {
            if0: alloc.interface(),
            if1: alloc.interface(),
            if2: alloc.interface(),
            if3: alloc.interface(),

            security_string: alloc.string(),

            ep_in:   alloc.alloc(Some(0x81.into()), EndpointType::Interrupt, 32,  4).unwrap(),
            ep_out:  alloc.alloc(Some(0x01.into()), EndpointType::Interrupt, 32,  8).unwrap(),

            ep2_in:  alloc.alloc(Some(0x82.into()), EndpointType::Interrupt, 32,  2).unwrap(),
            ep2_out: alloc.alloc(Some(0x02.into()), EndpointType::Interrupt, 32,  4).unwrap(),

            ep3_in:  alloc.alloc(Some(0x83.into()), EndpointType::Interrupt, 32, 64).unwrap(),
            ep3_out: alloc.alloc(Some(0x03.into()), EndpointType::Interrupt, 32, 16).unwrap(),

            ep4_in:  alloc.alloc(Some(0x84.into()), EndpointType::Interrupt, 32, 16).unwrap(),
        }
    }
}

impl<B: UsbBus> UsbClass<B> for XInput<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter,) -> usb_device::Result<()> {
        writer.interface(self.if0, 0xff, 0x5d, 0x01)?;
        writer.write(
            0x21,
            &[
                0x00, 0x01, 0x01, 0x25,
                0x81, 0x14,
                0x00, 0x00, 0x00, 0x00, 0x13,
                0x01, 0x08,
                0x00, 0x00,
            ],
        )?;
        writer.endpoint(&self.ep_in)?;
        writer.endpoint(&self.ep_out)?;

        writer.interface(self.if1, 0xff, 0x5d, 0x03)?;
        writer.write(
            0x21,
            &[
                0x00, 0x01, 0x01, 0x01,
                0x82, 0x40, 0x01,
                0x02, 0x20, 0x16,
                0x83, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x16,
                0x03, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        )?;
        writer.endpoint(&self.ep2_in)?;
        writer.endpoint(&self.ep2_out)?;
        writer.endpoint(&self.ep3_in)?;
        writer.endpoint(&self.ep3_out)?;

        writer.interface(self.if2, 0xff, 0x5d, 0x02)?;
        writer.write(0x21, &[0x00, 0x01, 0x01, 0x22, 0x84, 0x07, 0x00])?;
        writer.endpoint(&self.ep4_in)?;

        writer.interface_alt(self.if3, 0, 0xff, 0xfd, 0x13, Some(self.security_string))?;
        writer.write(0x41, &[0x00, 0x01, 0x01, 0x03])?;

        Ok(())
    }

    fn get_string(&self, index: StringIndex, _lang_id: LangID) -> Option<&str> {
        if index == self.security_string {
            Some(
                "Xbox Security Method 3, Version 1.00, © 2005 Microsoft Corporation. All rights reserved.",
            )
        } else {
            None
        }
    }
}
