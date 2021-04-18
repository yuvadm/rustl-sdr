extern crate log;
extern crate pretty_env_logger;
extern crate rusb;

mod tuners;
mod usb;

use log::{info, trace};
use usb::RtlSdrDeviceHandle;

const INTERFACE_ID: u8 = 0;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM"),
];

pub struct RtlSdr {
    handle: RtlSdrDeviceHandle,
}

impl RtlSdr {
    pub fn new() -> RtlSdr {
        pretty_env_logger::init();
        let handle = Self::open_device().unwrap();
        RtlSdr { handle: handle }
    }

    pub fn open_device() -> Option<RtlSdrDeviceHandle> {
        for dev in rusb::devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            trace!("Found USB device {{{:04x}:{:04x}}}", vid, pid);

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    info!("Found RTL-SDR device {} {{{:04x}:{:04x}}}", kd.2, vid, pid);
                    let usb_handle = dev.open().unwrap();
                    let mut handle = RtlSdrDeviceHandle::new(usb_handle, INTERFACE_ID);
                    // let kernel_driver_attached = handle.detach_kernel_driver();

                    let _iface = handle.claim_interface();

                    handle.test_write();
                    // reset device if write didn't succeed

                    handle.init_baseband();
                    handle.set_i2c_repeater(true);

                    handle.init_tuner();

                    return Some(handle);
                }
            }
        }
        return None;
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

    pub fn read_sync(&self, len: usize) -> Vec<u8> {
        return Vec::with_capacity(len);
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
