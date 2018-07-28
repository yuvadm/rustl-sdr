extern crate libusb;

use std::time::Duration;
use libusb::{Direction, RequestType, Recipient};

const BLOCK_USBB: u16 = 1;
const ADDR_USB_SYSCTL: u16 = 0x2000;

const INTERFACE_ID: u8 = 0;
const CTRL_TIMEOUT: Duration = Duration::from_millis(300);
const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM")
];

pub struct RtlSdr<'a> {
    ctx: &'a libusb::Context,
    dev: Option<libusb::DeviceHandle<'a>>,
    iface_id: u8,
}

impl<'a> RtlSdr<'a> {
    
    pub fn new(ctx: &'a libusb::Context) -> RtlSdr<'a> {
        RtlSdr {
            ctx,
            dev: None,
            iface_id: INTERFACE_ID,
        }
    }

    pub fn init(&mut self) {
        self.find_device();
    }

    fn write_reg(&self, handle: &libusb::DeviceHandle, block: u16, addr: u16, val: u8, _len: u8) -> usize {
        let vendor_out = libusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let mut data: [u8; 2] = [0, 0];
        let index: u16 = (block << 8) | 0x10;

        data[0] = val;
        data[1] = val;

        match handle.write_control(vendor_out, 0, addr, index, &data, CTRL_TIMEOUT) {
            Ok(n) => n,
            Err(_) => 0
        }
    }

    pub fn find_device(&mut self) {
        for mut dev in self.ctx.devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    let mut handle = dev.open().unwrap();

                    let has_kernel_driver = match handle.kernel_driver_active(self.iface_id) {
                            Ok(true) => {
                                handle.detach_kernel_driver(self.iface_id).ok();
                                true
                            },
                            _ => false
                    };

                    let _iface = handle.claim_interface(self.iface_id).unwrap();

                    let res = self.write_reg(&handle, BLOCK_USBB, ADDR_USB_SYSCTL, 0x09, 1);
                    println!("Got {}", res);

                    if has_kernel_driver {
                        handle.attach_kernel_driver(self.iface_id).ok();
                    }

                    self.dev = Some(handle);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let ctx = libusb::Context::new().unwrap();
        let mut rtlsdr = RtlSdr::new(&ctx);
        assert!(rtlsdr.dev.is_none());
        rtlsdr.init();
        assert!(rtlsdr.dev.is_some());
    }
}
