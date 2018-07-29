use super::Device;

pub const FC0013: Device = Device {
    NAME: "Fitipower FC0013",
    I2C_ADDR: 0xc6,
    CHECK_ADDR: 0x00,
    CHECK_VAL: 0xa3
};
