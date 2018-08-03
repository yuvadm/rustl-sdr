extern crate libusb;

mod devices;
mod usb;

use devices::*;
use usb::Usb;

const INTERFACE_ID: u8 = 0;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM")
];

pub struct RtlSdr<'a> {
    ctx: &'a libusb::Context,
    iface_id: u8
}

impl<'a> RtlSdr<'a> {
    
    pub fn new(ctx: &'a libusb::Context) -> RtlSdr<'a> {
        RtlSdr {
            ctx,
            iface_id: INTERFACE_ID
        }
    }

    pub fn open(&mut self) {
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

                    {
                        let usb = Usb::new(&handle);

                        usb.test_write();
                        // reset device if write didn't succeed

                        usb.init_baseband();
                        usb.set_i2c_repeater( true);

                        let d = r820t::R820T::new(&usb);
                        let _found = match usb.i2c_read_reg(d.device.i2c_addr, d.device.check_addr) {
                            Ok(reg) => {
                                if reg == d.device.check_val {
                                    println!("Found {} tuner\n", d.device.name);
                                    true
                                }
                                else {
                                    false
                                }
                            },
                            Err(_) => false
                        };

                        d.init();

                        usb.deinit_baseband();
                    }

                    if has_kernel_driver {
                        handle.attach_kernel_driver(self.iface_id).ok();
                    }
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
        rtlsdr.open();
    }
}
