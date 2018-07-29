use super::{DeviceInfo, Actions};

pub struct R820T {
    device: DeviceInfo
}

pub const DEVICE_INFO: DeviceInfo = DeviceInfo {
    id: "r820t",
    name: "Rafael Micro R820T",
    i2c_addr: 0x34,
    check_addr: 0x00,
    check_val: 0x69
};

impl R820T {
    fn new() -> R820T {
        R820T {
            device: DEVICE_INFO
        }
    }
}

impl Actions for R820T {
    fn init(&self) {
        println!("Init {}", self.device.name);
    }
}
