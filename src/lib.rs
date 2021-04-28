extern crate log;
extern crate pretty_env_logger;
extern crate rusb;

mod tuners;
mod usb;

use log::{error, info, trace, warn};
use std::convert::TryInto;
use tuners::*;
use usb::RtlSdrDeviceHandle;

const INTERFACE_ID: u8 = 0;

const DEF_RTL_XTAL_FREQ: u32 = 28800000;
// const MIN_RTL_XTAL_FREQ: u32 = DEF_RTL_XTAL_FREQ - 1000;
// const MAX_RTL_XTAL_FREQ: u32 = DEF_RTL_XTAL_FREQ + 1000;

const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM"),
];

pub struct RtlSdr {
    handle: RtlSdrDeviceHandle,
    tuner: Option<Box<dyn Tuner>>,
}

impl RtlSdr {
    pub fn new() -> RtlSdr {
        pretty_env_logger::init();
        let handle = Self::open_device().unwrap();
        let tuner_id = Self::search_tuner(&handle).unwrap();
        let tuner = Self::init_tuner(tuner_id, &handle);

        RtlSdr { handle, tuner }
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

                    return Some(handle);
                }
            }
        }
        None
    }

    fn search_tuner(handle: &RtlSdrDeviceHandle) -> Option<&str> {
        for tuner_info in KNOWN_TUNERS.iter() {
            let regval = handle.i2c_read_reg(tuner_info.i2c_addr, tuner_info.check_addr);
            trace!(
                "Probing tuner {} at I2C address {:#02x} and checking address {:#02x}",
                tuner_info.name,
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
                Err(e) => {
                    warn!("Reading failed with {}, continuing", e);
                }
            };
        }
        None
    }

    pub fn init_tuner(tuner_id: &str, handle: &RtlSdrDeviceHandle) -> Option<Box<dyn Tuner>> {
        let tuner: Option<Box<dyn Tuner>> = match tuner_id {
            // r820t::TUNER_ID => Some(Tuners::R820T(r820t::R820T::new(&self))),
            // fc0013::TUNER_ID => Some(Tuners::FC0013(fc0013::FC0013::new(&self))),
            r820t::TUNER_ID => Some(Box::new(r820t::R820T::new(handle))),
            fc0013::TUNER_ID => Some(Box::new(fc0013::FC0013::new(handle))),
            _ => {
                error!("Could not find any valid tuner.");
                None
            }
        };
        info!("Found tuner r820t");
        tuner
    }

    pub fn set_freq(&self) {}

    pub fn set_sample_rate(&mut self, samp_rate: u32) {
        let real_rsamp_ratio: u32;

        // check if the rate is supported by the resampler
        if (samp_rate <= 225000)
            || (samp_rate > 3200000)
            || ((samp_rate > 300000) && (samp_rate <= 900000))
        {
            error!("Invalid sample rate: {} Hz", samp_rate);
        }

        let mut rsamp_ratio: u32 = (DEF_RTL_XTAL_FREQ * (2 ^ 22)) / samp_rate;
        rsamp_ratio &= 0x0ffffffc;

        real_rsamp_ratio = rsamp_ratio | ((rsamp_ratio & 0x08000000) << 1);
        let real_rate: f64 = ((DEF_RTL_XTAL_FREQ * (2 ^ 22)) / real_rsamp_ratio).into();
        info!("Exact sample rate is: {} Hz", real_rate);

        self.handle.set_i2c_repeater(true);
        self.tuner
            .as_mut()
            .unwrap()
            .set_bandwidth(real_rate as u32, &self.handle);
        self.handle.set_i2c_repeater(false);

        let mut tmp: u16 = (rsamp_ratio >> 16).try_into().unwrap();
        self.handle.demod_write_reg(1, 0x9f, tmp, 2);
        tmp = (rsamp_ratio & 0xffff).try_into().unwrap();
        self.handle.demod_write_reg(1, 0xa1, tmp, 2);
        // self.set_sample_freq_corr();

        // reset demod (bit 3, soft_rst)
        self.handle.demod_write_reg(1, 0x01, 0x14, 1);
        self.handle.demod_write_reg(1, 0x01, 0x10, 1);

        // self.set_offset_tuning();
    }

    pub fn set_if_freq(&self, freq: u32) {
        self.handle.set_if_freq(freq);
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
