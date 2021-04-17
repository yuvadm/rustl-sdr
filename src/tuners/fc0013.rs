use super::{Tuner, TunerInfo};
use usb::RtlSdrDeviceHandle;

pub struct FC0013<'a> {
    pub device: TunerInfo,
    pub handle: &'a RtlSdrDeviceHandle,
}

pub const TUNER_ID: &str = "fc0013";

pub const TUNER_INFO: TunerInfo = TunerInfo {
    id: TUNER_ID,
    name: "Fitipower FC0013",
    i2c_addr: 0xc6,
    check_addr: 0x00,
    check_val: 0xa3,
};

impl<'a> FC0013<'a> {
    pub fn new(handle: &'a RtlSdrDeviceHandle) -> FC0013<'a> {
        FC0013 {
            device: TUNER_INFO,
            handle: handle,
        }
    }
}

impl<'a> Tuner for FC0013<'a> {
    fn init(&self) {
        println!("Init {}", self.device.name);
    }

    fn exit(&self) {
        unimplemented!()
    }

    fn set_freq(&self, _freq: u32) {
        unimplemented!()
    }

    fn set_bw(&self, _bw: u32) {
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
