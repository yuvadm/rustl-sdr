extern crate libusb;

pub struct Dongle {
    vendor: u16,
    product: u16,
    name: &'static str
}

pub struct RtlSdr {
    usb_ctx: libusb::Context
}

impl RtlSdr {
    pub fn new() -> RtlSdr {
        let usb_ctx = libusb::Context::new().unwrap();
        RtlSdr {
            usb_ctx
        }
    }

    pub fn find(&self) {
        for mut dev in self.usb_ctx.devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
                dev.bus_number(),
                dev.address(),
                desc.vendor_id(),
                desc.product_id());
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dongle() {
        let _dongle = Dongle {
            vendor: 0x1234,
            product: 0x5678,
            name: "Test"
        };
        assert!(_dongle.name == "Test");
    }
}
