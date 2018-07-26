extern crate libusb;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM")
];

pub struct RtlSdr<'a> {
    ctx: &'a libusb::Context,
    dev: Option<libusb::DeviceHandle<'a>>
}

impl<'a> RtlSdr<'a> {
    
    pub fn new(ctx: &'a libusb::Context) -> RtlSdr<'a> {
        RtlSdr {
            ctx,
            dev: None
        }
    }

    pub fn init(&mut self) {
        self.dev = self.find_device();
    }

    pub fn find_device(&self) -> Option<libusb::DeviceHandle<'a>> {
        for mut dev in self.ctx.devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    return Some(dev.open().unwrap())
                }
            }
        }
        None
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
