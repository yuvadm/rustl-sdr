use super::{DeviceInfo, Device};

#[allow(dead_code)]
pub struct FC0013 {
    pub device: DeviceInfo
}

#[allow(dead_code)]
pub const DEVICE_INFO: DeviceInfo = DeviceInfo {
    id: "fc0013",
    name: "Fitipower FC0013",
    i2c_addr: 0xc6,
    check_addr: 0x00,
    check_val: 0xa3
};

#[allow(dead_code)]
impl FC0013 {
    pub fn new() -> FC0013 {
        FC0013 {
            device: DEVICE_INFO
        }
    }
}

impl Device for FC0013 {
    fn init(&self) {
        println!("Init {}", self.device.name);
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
