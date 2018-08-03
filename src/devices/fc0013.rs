use super::{DeviceInfo, Device};
use usb::Usb;

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
    fn init(&self, _usb: &Usb) {
        println!("Init {}", self.device.name);
    }
}
