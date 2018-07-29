pub mod fc0013;
pub mod r820t;

#[allow(non_snake_case)]
pub struct Device {
    pub NAME: &'static str,
    pub I2C_ADDR: u8,
    pub CHECK_ADDR: u8,
    pub CHECK_VAL: u8
}

