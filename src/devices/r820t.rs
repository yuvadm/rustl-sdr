use super::{DeviceInfo, Device};
use usb::Usb;

const R82XX_IF_FREQ: u32 = 3570000;

pub struct R820T<'a> {
    pub device: DeviceInfo,
    pub usb: &'a Usb<'a>
}

pub const DEVICE_INFO: DeviceInfo = DeviceInfo {
    id: "r820t",
    name: "Rafael Micro R820T",
    i2c_addr: 0x34,
    check_addr: 0x00,
    check_val: 0x69
};

impl<'a> R820T<'a> {
    pub fn new(usb: &'a Usb) -> R820T<'a> {
        R820T {
            device: DEVICE_INFO,
            usb
        }
    }
}

impl<'a> Device for R820T<'a> {
    fn init(&self) {
        // disable Zero-IF mode
        self.usb.demod_write_reg(1, 0xb1, 0x1a, 1);

        // only enable In-phase ADC input
        self.usb.demod_write_reg(0, 0x08, 0x4d, 1);

        // the R82XX use 3.57 MHz IF for the DVB-T 6 MHz mode, and
        // 4.57 MHz for the 8 MHz mode
        self.usb.set_if_freq(R82XX_IF_FREQ);

        // enable spectrum inversion
        self.usb.demod_write_reg(1, 0x15, 0x01, 1);
    }

    fn exit(&self){
        unimplemented!()
    }

    fn set_freq(&self, _freq: u32){
        unimplemented!()
    }

    fn set_bw(&self, _bw: u32){
        unimplemented!()
    }

    fn set_gain(&self, _gain: u32){
        unimplemented!()
    }

    fn set_if_gain(&self, _if_gain: u32){
        unimplemented!()
    }

    fn set_gain_mode(&self, _mode: bool){
        unimplemented!()
    }

}
