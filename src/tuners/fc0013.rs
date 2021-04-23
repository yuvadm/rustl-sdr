use super::{Tuner, TunerInfo};
use usb::RtlSdrDeviceHandle;

pub struct FC0013 {
    pub device: TunerInfo,
    // pub handle: &'a RtlSdrDeviceHandle,
}

pub const TUNER_ID: &str = "fc0013";

pub const TUNER_INFO: TunerInfo = TunerInfo {
    id: TUNER_ID,
    name: "Fitipower FC0013",
    i2c_addr: 0xc6,
    check_addr: 0x00,
    check_val: 0xa3,
    // gains: vec![
    //     -99, -73, -65, -63, -60, -58, -54, 58, 61, 63, 65, 67, 68, 70, 71, 179, 181, 182, 184, 186,
    //     188, 191, 197,
    // ],
};

impl FC0013 {
    pub fn new(handle: &RtlSdrDeviceHandle) -> FC0013 {
        let tuner = FC0013 { device: TUNER_INFO };
        tuner.init(handle);
        tuner
    }
}

impl Tuner for FC0013 {
    fn init(&self, _handle: &RtlSdrDeviceHandle) {
        unimplemented!()
    }

    fn exit(&self) {
        unimplemented!()
    }

    fn set_freq(&self, _freq: u32) {
        unimplemented!()
    }

    fn set_bandwidth(&self, _bw: u32, _handle: &RtlSdrDeviceHandle) {
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
