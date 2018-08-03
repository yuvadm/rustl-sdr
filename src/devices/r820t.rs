use super::{DeviceInfo, Device};
use usb::Usb;

const R82XX_IF_FREQ: u32 = 3570000;

pub struct R820T {
    pub device: DeviceInfo
}

pub const DEVICE_INFO: DeviceInfo = DeviceInfo {
    id: "r820t",
    name: "Rafael Micro R820T",
    i2c_addr: 0x34,
    check_addr: 0x00,
    check_val: 0x69
};

impl R820T {
    pub fn new() -> R820T {
        R820T {
            device: DEVICE_INFO
        }
    }
}

impl Device for R820T {
    fn init(&self, usb: &Usb) {
        // disable Zero-IF mode
        usb.demod_write_reg(1, 0xb1, 0x1a, 1);

        // only enable In-phase ADC input
        usb.demod_write_reg(0, 0x08, 0x4d, 1);

        // the R82XX use 3.57 MHz IF for the DVB-T 6 MHz mode, and
        // 4.57 MHz for the 8 MHz mode
        usb.set_if_freq(R82XX_IF_FREQ);

        // enable spectrum inversion
        usb.demod_write_reg(1, 0x15, 0x01, 1);
    }
}
