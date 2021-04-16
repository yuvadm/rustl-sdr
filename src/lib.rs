extern crate rusb;

mod devices;
mod usb;

use devices::*;
use usb::RtlSdrDeviceHandle;

const INTERFACE_ID: u8 = 0;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM"),
];

pub struct RtlSdr {
    // ctx: rusb::Context,
    // device: UsbDevice,
    iface_id: u8,
}

impl RtlSdr {
    pub fn new() -> RtlSdr {
        RtlSdr {
            // ctx: rusb::Context::new().unwrap(),
            iface_id: INTERFACE_ID,
        }
    }

    pub fn open(&mut self) {
        for dev in rusb::devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    let usb_handle = dev.open().unwrap();
                    let mut handle = RtlSdrDeviceHandle::new(usb_handle, self.iface_id);
                    let kernel_driver_attached = handle.detach_kernel_driver();

                    let _iface = handle.claim_interface();

                    handle.test_write();
                    // reset device if write didn't succeed

                    handle.init_baseband();
                    handle.set_i2c_repeater(true);

                    let d = r820t::R820T::new(handle);
                    // let _found = match handle.i2c_read_reg(d.device.i2c_addr, d.device.check_addr) {
                    //     Ok(reg) => {
                    //         if reg == d.device.check_val {
                    //             println!("Found {} tuner\n", d.device.name);
                    //             true
                    //         } else {
                    //             false
                    //         }
                    //     }
                    //     Err(_) => false,
                    // };

                    d.init();

                    // handle.deinit_baseband();

                    if kernel_driver_attached {
                        //     handle.attach_kernel_driver();
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
        let mut rtlsdr = RtlSdr::new();
        let x = rtlsdr.open();
        assert_eq!(x, 1);
    }
}
