use super::{Tuner, TunerInfo};
use usb::RtlSdrDeviceHandle;

pub const TUNER_ID: &str = "r820t";
pub const TUNER_NAME: &str = "Rafael Micro R820T";

const I2C_ADDR: u8 = 0x34;
const CHECK_ADDR: u8 = 0x00;
const CHECK_VAL: u8 = 0x69;
const IF_FREQ: u32 = 3570000;
const REG_SHADOW_START: u8 = 5;
const NUM_REGS: usize = 30;
const NUM_IMR: usize = 5;
const IMR_TRIAL: u8 = 9;

enum Chip {
    R820T,
    R620D,
    R828D,
    R828,
    R828S,
    R820C,
}

enum TunerType {
    Radio,
    AnalogTv,
    DigitalTv,
}

enum XtalCapValue {
    XtalLowCap30P,
    XtalLowCap20P,
    XtalLowCap10P,
    XtalLowCap0P,
    XtalHighCap0P,
}

pub struct R820T {
    pub device: TunerInfo,
    regs: [u8; NUM_REGS],
    buf: [u8; NUM_REGS + 1],
    xtal_cal_sel: XtalCapValue,
    pll: u16, // kHz
    int_freq: u32,
    fil_cal_code: u8,
    input: u8,
    has_lock: i16,  // bool?
    init_done: i16, // bool?
    delsys: u32,
    tuner_type: TunerType,
    bw: u32, // MHz
}

pub struct FreqRange {
    freq: u32,
    open_d: u8,
    rf_mux_ploy: u8,
    tf_c: u8,
    xtal_cap20p: u8,
    xtal_cap10p: u8,
    xtal_cap0p: u8,
}

pub struct Config {
    i2c_addr: u8,
    xtal: u32,
    chip: Chip,
    max_i2c_msg_len: u16,
    use_predetect: i16, // bool?
}

pub const TUNER_INFO: TunerInfo = TunerInfo {
    id: TUNER_ID,
    name: TUNER_NAME,
    i2c_addr: I2C_ADDR,
    check_addr: CHECK_ADDR,
    check_val: CHECK_VAL,
    // gains: vec![
    //     0, 9, 14, 27, 37, 77, 87, 125, 144, 157, 166, 197, 207, 229, 254, 280, 297, 328, 338, 364,
    //     372, 386, 402, 421, 434, 439, 445, 480, 496,
    // ],
};

// starts from REG_SHADOW_START
const INITIAL_REGS: [u8; NUM_REGS] = vec![
    0x83, 0x32, 0x75, 0xc0, 0x40, 0xd6, 0x6c, 0xf5, 0x63, 0x75, 0x68, 0x6c, 0x83, 0x80, 0x00, 0x0f,
    0x00, 0xc0, 0x30, 0x48, 0xcc, 0x60, 0x00, 0x54, 0xae, 0x4a, 0xc0,
];

const FREQ_RANGES: [(u32, u8, u8, u8, u8, u8, u8); 21] = [
    (000, 0x08, 0x02, 0xdf, 0x02, 0x01, 0x00),
    (050, 0x08, 0x02, 0xbe, 0x02, 0x01, 0x00),
    (055, 0x08, 0x02, 0x8b, 0x02, 0x01, 0x00),
    (060, 0x08, 0x02, 0x7b, 0x02, 0x01, 0x00),
    (065, 0x08, 0x02, 0x69, 0x02, 0x01, 0x00),
    (070, 0x08, 0x02, 0x58, 0x02, 0x01, 0x00),
    (075, 0x00, 0x02, 0x44, 0x02, 0x01, 0x00),
    (080, 0x00, 0x02, 0x44, 0x02, 0x01, 0x00),
    (090, 0x00, 0x02, 0x34, 0x01, 0x01, 0x00),
    (100, 0x00, 0x02, 0x34, 0x01, 0x01, 0x00),
    (110, 0x00, 0x02, 0x24, 0x01, 0x01, 0x00),
    (120, 0x00, 0x02, 0x24, 0x01, 0x01, 0x00),
    (140, 0x00, 0x02, 0x14, 0x01, 0x01, 0x00),
    (180, 0x00, 0x02, 0x13, 0x00, 0x00, 0x00),
    (220, 0x00, 0x02, 0x13, 0x00, 0x00, 0x00),
    (250, 0x00, 0x02, 0x11, 0x00, 0x00, 0x00),
    (280, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00),
    (310, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00),
    (450, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00),
    (588, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00),
    (650, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00),
];

const XTAL_CAPS: [(u8, XtalCapValue); 5] = [
    (0x0b, XtalCapValue::XtalLowCap30P),
    (0x02, XtalCapValue::XtalLowCap20P),
    (0x01, XtalCapValue::XtalLowCap10P),
    (0x00, XtalCapValue::XtalLowCap0P),
    (0x10, XtalCapValue::XtalHighCap0P),
];

impl R820T {
    pub fn new(handle: &RtlSdrDeviceHandle) -> R820T {
        let tuner = R820T { device: TUNER_INFO };
        tuner.init(handle);
        tuner
    }
}

impl Drop for R820T {
    fn drop(&mut self) {
        self.exit();
    }
}

impl Tuner for R820T {
    fn init(&self, handle: &RtlSdrDeviceHandle) {
        // disable Zero-IF mode
        handle.demod_write_reg(1, 0xb1, 0x1a, 1);

        // only enable In-phase ADC input
        handle.demod_write_reg(0, 0x08, 0x4d, 1);

        // the R82XX use 3.57 MHz IF for the DVB-T 6 MHz mode, and
        // 4.57 MHz for the 8 MHz mode
        handle.set_if_freq(IF_FREQ);

        // enable spectrum inversion
        handle.demod_write_reg(1, 0x15, 0x01, 1);
    }

    fn exit(&self) {}

    fn set_freq(&self, _freq: u32) {
        unimplemented!()
    }

    fn set_bandwidth(&self, bw: u16, rate: u32, handle: &RtlSdrDeviceHandle) {
        let mut rc: u16;
        let mut real_bw: u16 = 0;
        let mut reg_0a: u8;
        let mut reg_0b: u8;

        if bw > 7000000 {}
    }

    fn set_gain(&self, _gain: u32) {
        unimplemented!()
    }

    fn set_if_gain(&self, _if_gain: u32) {
        unimplemented!()
    }

    fn set_gain_mode(&self, _mode: bool) {
        unimplemented!()
    }

    fn display(&self) -> &str {
        self.device.name
    }
}
