pub mod fc0013;
pub mod r820t;

pub struct TunerInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub i2c_addr: u8,
    pub check_addr: u8,
    pub check_val: u8,
}

pub trait Tuner {
    fn init(&self);
    fn exit(&self);
    fn set_freq(&self, freq: u32);
    fn set_bw(&self, bw: u32);
    fn set_gain(&self, gain: u32);
    fn set_if_gain(&self, if_gain: u32);
    fn set_gain_mode(&self, mode: bool);
    fn display(&self) -> &str;
}
