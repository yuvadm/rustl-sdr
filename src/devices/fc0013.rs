use super::{DeviceInfo, Actions};

pub struct FC0013 {
    device: DeviceInfo
}

pub const DEVICE_INFO: DeviceInfo = DeviceInfo {
    id: "fc0013",
    name: "Fitipower FC0013",
    i2c_addr: 0xc6,
    check_addr: 0x00,
    check_val: 0xa3
};

impl FC0013 {
    fn new() -> FC0013 {
        FC0013 {
            device: DEVICE_INFO
        }
    }
}

impl Actions for FC0013 {
    fn init(&self) {
        println!("Init {}", self.device.name);
    }
}
