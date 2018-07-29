use super::Device;

pub const R820T: Device = Device {
    name: "Rafael Micro R820T",
    i2c_addr: 0x34,
    check_addr: 0x00,
    check_val: 0x69
};
