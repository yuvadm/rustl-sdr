extern crate libusb;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM")
];

struct Usb {
    ctx: libusb::Context,
    desc: Option<libusb::DeviceDescriptor>
    // dev: Option<libusb::Device>
}

pub struct RtlSdr {
    usb: Usb
}

impl RtlSdr {
    
    pub fn new() -> RtlSdr {
        RtlSdr {
            usb: Usb {
                ctx: libusb::Context::new().unwrap(),
                desc: None
            }
        }
    }

    pub fn init(&mut self) {
        self.usb.desc = self.find_device();
    }

    pub fn find_device(&self) -> Option<libusb::DeviceDescriptor> {
        for mut dev in self.usb.ctx.devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    return Some(desc)
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
        let mut rtlsdr = RtlSdr::new();
        assert!(rtlsdr.usb.desc.is_none());
        rtlsdr.init();
        assert!(rtlsdr.usb.desc.is_some());
    }
}
