use super::Device;

pub const R820T: Device = Device {
    NAME: "Rafael Micro R820T",
    I2C_ADDR: 0x34,
    CHECK_ADDR: 0x00,
    CHECK_VAL: 0x69
};
