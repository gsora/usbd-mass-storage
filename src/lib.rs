use usb_device::{
    class_prelude::{InterfaceNumber, UsbBus, UsbBusAllocator, UsbClass},
    control,
    endpoint::{EndpointIn, EndpointOut},
};

use log::debug;

const BULK_ONLY_MASS_STORAGE_RESET: u8 = 0xff;
const GET_MAX_LUN: u8 = 0xfe;

pub struct UMSClass<'a, B: UsbBus> {
    if_num: InterfaceNumber,
    /// Low-latency OUT buffer
    out_ep: EndpointOut<'a, B>,
    /// Low-latency IN buffer
    in_ep: EndpointIn<'a, B>,
}

impl<B: UsbBus> UMSClass<'_, B> {
    pub fn new<'a>(alloc: &'a UsbBusAllocator<B>) -> UMSClass<'a, B> {
        let out_ep = alloc.bulk(64);
        let in_ep = alloc.bulk(64);

        UMSClass {
            if_num: alloc.interface(),
            out_ep,
            in_ep,
        }
    }
}

impl<B: UsbBus> UsbClass<B> for UMSClass<'_, B> {
    fn get_configuration_descriptors(
        &self,
        writer: &mut usb_device::descriptor::DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface(
            self.if_num,
            0x8,  // mass storage class
            0x6,  // SCSI
            0x50, // bulk-only
        )?;

        let _ = writer;
        Ok(())
    }

    fn get_bos_descriptors(
        &self,
        writer: &mut usb_device::descriptor::BosWriter,
    ) -> usb_device::Result<()> {
        let _ = writer;
        Ok(())
    }

    fn get_string(
        &self,
        index: usb_device::class_prelude::StringIndex,
        lang_id: u16,
    ) -> Option<&str> {
        let _ = (index, lang_id);
        None
    }

    fn reset(&mut self) {}

    fn poll(&mut self) {}

    fn control_out(&mut self, xfer: usb_device::class_prelude::ControlOut<B>) {
        let _ = xfer;
    }

    fn control_in(&mut self, xfer: usb_device::class_prelude::ControlIn<B>) {
        let req = xfer.request();

        let if_num = u8::from(self.if_num) as u16;

        // Bail out if its not relevant to our interface.
        if !(req.recipient == control::Recipient::Interface
            && req.index == if_num)
        {
            return;
        }

        debug!("control_in for interface {}, request number {}", if_num, req.request);

        match req.request {
            BULK_ONLY_MASS_STORAGE_RESET => {
                debug!("request BULK_ONLY_MASS_STORAGE_RESET");
                xfer.accept_with(&[]).ok();
            }
            GET_MAX_LUN => {
                debug!("request GET_MAX_LUN");
                xfer.accept_with(&[0x00]).ok();
            },
            _ => {},
        }
    }

    fn endpoint_setup(&mut self, addr: usb_device::endpoint::EndpointAddress) {
        let _ = addr;
    }

    fn endpoint_out(&mut self, addr: usb_device::endpoint::EndpointAddress) {
        let _ = addr;
    }

    fn endpoint_in_complete(&mut self, addr: usb_device::endpoint::EndpointAddress) {
        let _ = addr;
    }
}
