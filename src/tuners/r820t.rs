use super::{Tuner, TunerInfo};
use usb::RtlSdrDeviceHandle;

const R82XX_IF_FREQ: u32 = 3570000;

pub struct R820T {
    pub device: TunerInfo,
    // pub handle: &'a RtlSdrDeviceHandle,
}

pub const TUNER_ID: &str = "r820t";

pub const TUNER_INFO: TunerInfo = TunerInfo {
    id: TUNER_ID,
    name: "Rafael Micro R820T",
    i2c_addr: 0x34,
    check_addr: 0x00,
    check_val: 0x69,
    // gains: vec![
    //     0, 9, 14, 27, 37, 77, 87, 125, 144, 157, 166, 197, 207, 229, 254, 280, 297, 328, 338, 364,
    //     372, 386, 402, 421, 434, 439, 445, 480, 496,
    // ],
};

impl R820T {
    pub fn new(handle: &RtlSdrDeviceHandle) -> R820T {
        let tuner = R820T { device: TUNER_INFO };
        tuner.init(handle);
        tuner
    }
}

impl Drop for R820T {
    fn drop(&mut self) {
        self.exit();
    }
}

impl Tuner for R820T {
    fn init(&self, handle: &RtlSdrDeviceHandle) {
        // disable Zero-IF mode
        handle.demod_write_reg(1, 0xb1, 0x1a, 1);

        // only enable In-phase ADC input
        handle.demod_write_reg(0, 0x08, 0x4d, 1);

        // the R82XX use 3.57 MHz IF for the DVB-T 6 MHz mode, and
        // 4.57 MHz for the 8 MHz mode
        handle.set_if_freq(R82XX_IF_FREQ);

        // enable spectrum inversion
        handle.demod_write_reg(1, 0x15, 0x01, 1);
    }

    fn exit(&self) {}

    fn set_freq(&self, _freq: u32) {
        unimplemented!()
    }

    fn set_bw(&self, _bw: u32, _handle: &RtlSdrDeviceHandle) {
        unimplemented!()
    }

    fn set_gain(&self, _gain: u32) {
        unimplemented!()
    }

    fn set_if_gain(&self, _if_gain: u32) {
        unimplemented!()
    }

    fn set_gain_mode(&self, _mode: bool) {
        unimplemented!()
    }

    fn display(&self) -> &str {
        self.device.name
    }
}
