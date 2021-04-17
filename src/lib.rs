extern crate log;
extern crate pretty_env_logger;
extern crate rusb;

mod tuners;
mod usb;

use log::{error, info, trace};
use tuners::*;
use usb::RtlSdrDeviceHandle;

const INTERFACE_ID: u8 = 0;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM"),
];

const KNOWN_TUNERS: [TunerInfo; 2] = [r820t::TUNER_INFO, fc0013::TUNER_INFO];

pub struct RtlSdr {
    // ctx: rusb::Context,
    // device: UsbDevice,
    iface_id: u8,
}

impl RtlSdr {
    pub fn new() -> RtlSdr {
        pretty_env_logger::init();
        RtlSdr {
            // ctx: rusb::Context::new().unwrap(),
            iface_id: INTERFACE_ID,
        }
    }

    /// Open a new RTL-SDR device
    pub fn open(&mut self) {
        for dev in rusb::devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            trace!("Found USB device with vid {} and pid {}", vid, pid);

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    info!("Found {} {{{:04x}:{:04x}}}", kd.2, vid, pid);
                    let usb_handle = dev.open().unwrap();
                    let mut handle = RtlSdrDeviceHandle::new(usb_handle, self.iface_id);
                    // let kernel_driver_attached = handle.detach_kernel_driver();

                    let _iface = handle.claim_interface();

                    handle.test_write();
                    // reset device if write didn't succeed

                    handle.init_baseband();
                    handle.set_i2c_repeater(true);

                    {
                        let tuner: Box<dyn Tuner> = match self.search_tuner(&handle) {
                            Some(tuner) => match tuner {
                                r820t::TUNER_ID => Box::new(r820t::R820T::new(&handle)),
                                fc0013::TUNER_ID => Box::new(fc0013::FC0013::new(&handle)),
                                _ => {
                                    error!("Could not find any valid tuner, aborting.");
                                    return;
                                }
                            },
                            None => {
                                error!("Could not find any valid tuner, aborting.");
                                return;
                            }
                        };

                        tuner.init();
                    }
                }
            }
        }
        error!("Could not find any RTL-SDR device, aborting.")
    }

    /// Probe all known tuners at their I2C addresses
    /// and search for expected return values
    fn search_tuner(&mut self, handle: &RtlSdrDeviceHandle) -> Option<&str> {
        for tuner_info in KNOWN_TUNERS.iter() {
            match handle.i2c_read_reg(tuner_info.i2c_addr, tuner_info.check_addr) {
                Ok(val) => {
                    if val == tuner_info.check_val {
                        return Some(tuner_info.name);
                    }
                }
                Err(_) => {}
            };
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
        let found = rtlsdr.open();
        assert_eq!(found, true);
    }
}
