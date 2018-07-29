pub mod fc0013;
pub mod r820t;

pub struct DeviceInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub i2c_addr: u8,
    pub check_addr: u8,
    pub check_val: u8
}

pub trait Actions {
    fn init(&self);
}
