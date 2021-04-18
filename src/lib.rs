extern crate log;
extern crate pretty_env_logger;
extern crate rusb;

mod tuners;
mod usb;

use log::{error, info, trace, warn};
use tuners::*;
use usb::RtlSdrDeviceHandle;

const INTERFACE_ID: u8 = 0;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM"),
];

const KNOWN_TUNERS: [TunerInfo; 2] = [r820t::TUNER_INFO, fc0013::TUNER_INFO];

pub struct RtlSdr {
    handle: RtlSdrDeviceHandle,
    tuner: Box<dyn Tuner>,
}

impl RtlSdr {
    pub fn new() -> RtlSdr {
        pretty_env_logger::init();

        let handle = Self::open_device().unwrap();
        let tuner = Self::open_tuner(&handle).unwrap();

        RtlSdr {
            handle: handle,
            tuner: tuner,
        }
    }

    pub fn open_device() -> Option<RtlSdrDeviceHandle> {
        for dev in rusb::devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            trace!("Found USB device with vid {} and pid {}", vid, pid);

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    info!("Found device {} {{{:04x}:{:04x}}}", kd.2, vid, pid);
                    let usb_handle = dev.open().unwrap();
                    let mut handle = RtlSdrDeviceHandle::new(usb_handle, INTERFACE_ID);
                    // let kernel_driver_attached = handle.detach_kernel_driver();

                    let _iface = handle.claim_interface();

                    handle.test_write();
                    // reset device if write didn't succeed

                    handle.init_baseband();
                    handle.set_i2c_repeater(true);

                    return Some(handle);
                }
            }
        }
        return None;
    }

    fn open_tuner(handle: &RtlSdrDeviceHandle) -> Option<Box<dyn Tuner>> {
        let tuner_id = match Self::search_tuner(&handle) {
            Some(tid) => {
                info!("Got tuner ID {}", tid);
                tid
            }
            None => {
                error!("Could not find a value tuner, aborting.");
                return None;
            }
        };

        let tuner: Option<Box<dyn Tuner>> = match tuner_id {
            r820t::TUNER_ID => Some(Box::new(r820t::R820T::new(&handle))),
            fc0013::TUNER_ID => Some(Box::new(fc0013::FC0013::new(&handle))),
            _ => {
                error!("Could not find any valid tuner, aborting.");
                return None;
            }
        };

        // info!("Found tuner {}", self.tuner.unwrap().display());
        return tuner;
    }

    /// Probe all known tuners at their I2C addresses
    /// and search for expected return values
    fn search_tuner(handle: &RtlSdrDeviceHandle) -> Option<&str> {
        for tuner_info in KNOWN_TUNERS.iter() {
            let regval = handle.i2c_read_reg(tuner_info.i2c_addr, tuner_info.check_addr);
            trace!(
                "Probing I2C address {:#02x} checking address {:#02x}",
                tuner_info.i2c_addr,
                tuner_info.check_addr,
            );
            match regval {
                Ok(val) => {
                    trace!(
                        "Expecting value {:#02x}, got value {:#02x}",
                        tuner_info.check_val,
                        val
                    );
                    if val == tuner_info.check_val {
                        return Some(tuner_info.id);
                    }
                }
                Err(_) => {
                    warn!("Reading failed, continuing");
                }
            };
        }
        None
    }

    pub fn set_sample_rate(&self, samp_rate: u32) {
        self.handle.set_sample_rate(samp_rate);
    }

    pub fn set_test_mode(&self, on: bool) {
        let val = match on {
            true => 0x03,
            false => 0x05,
        };
        self.handle.demod_write_reg(0, 0x19, val, 1);
    }

    pub fn reset_buffer(&self) {
        self.handle.reset_buffer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let _rtlsdr = RtlSdr::new();
    }
}
