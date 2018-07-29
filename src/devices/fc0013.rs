use super::Device;

pub const FC0013: Device = Device {
    name: "Fitipower FC0013",
    i2c_addr: 0xc6,
    check_addr: 0x00,
    check_val: 0xa3
};
