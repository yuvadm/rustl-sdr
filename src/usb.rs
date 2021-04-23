extern crate log;
extern crate pretty_env_logger;
extern crate rusb;

use log::{error, info, trace, warn};
use rusb::{DeviceHandle, Direction, GlobalContext, Recipient, RequestType};
use std::convert::TryInto;
use std::time::Duration;
use tuners::*;

// const BLOCK_DEMODB: u16 = 0;
const BLOCK_USBB: u16 = 1;
const BLOCK_SYSB: u16 = 2;
// const BLOCK_TUNB: u16 = 3;
const BLOCK_IICB: u8 = 6;

const ADDR_USB_SYSCTL: u16 = 0x2000;
// const ADDR_USB_CTRL: u16 = 0x2010;
// const ADDR_USB_STAT: u16 = 0x2014;
// const ADDR_USB_EPA_CFG: u16 = 0x2144;
const ADDR_USB_EPA_CTL: u16 = 0x2148;
const ADDR_USB_EPA_MAXPKT: u16 = 0x2158;
// const ADDR_USB_EPA_MAXPKT_2: u16 = 0x215a;
// const ADDR_USB_EPA_FIFO_CFG: u16 = 0x2160;

const ADDR_SYS_DEMOD_CTL: u16 = 0x3000;
const ADDR_SYS_DEMOD_CTL_1: u16 = 0x300b;

const FIR_LENGTH: usize = 20;
const FIR_DEFAULT: [u8; FIR_LENGTH] = [
    0xca, 0xdc, 0xd7, 0xd8, 0xe0, 0xf2, 0x0e, 0x35, 0x06, 0x50, 0x9c, 0x0d, 0x71, 0x11, 0x14, 0x71,
    0x74, 0x19, 0x41, 0xa5,
];

const DEF_RTL_XTAL_FREQ: u32 = 28800000;
// const MIN_RTL_XTAL_FREQ: u32 = DEF_RTL_XTAL_FREQ - 1000;
// const MAX_RTL_XTAL_FREQ: u32 = DEF_RTL_XTAL_FREQ + 1000;

const CTRL_TIMEOUT: Duration = Duration::from_millis(300);

pub struct RtlSdrDeviceHandle {
    handle: DeviceHandle<GlobalContext>,
    // tuner: Option<r820t::R820T>, // when we go abstract: Option<Box<dyn Tuner>>,
    iface_id: u8,
    kernel_driver_active: bool,
}

/// A wrapper around libusb's DeviceHandle that implements
/// various rtl-sdr specific methods
impl RtlSdrDeviceHandle {
    pub fn new(handle: DeviceHandle<GlobalContext>, iface_id: u8) -> RtlSdrDeviceHandle {
        let mut handle = RtlSdrDeviceHandle {
            handle,
            // tuner: None,
            iface_id,
            kernel_driver_active: false,
        };

        handle.detach_kernel_driver();
        handle
    }

    pub fn init_tuner(self) {
        let tuner_id: &str = match self.search_tuner() {
            Some(tid) => {
                trace!("Found tuner ID {}", tid);
                tid
            }
            None => "",
        };

        let tuner: Option<r820t::R820T> = match tuner_id {
            r820t::TUNER_ID => Some(r820t::R820T::new(self)),
            // fc0013::TUNER_ID => Some(Box::new(fc0013::FC0013::new(&self))),
            _ => {
                error!("Could not find any valid tuner.");
                None
            }
        };

        // self.tuner = tuner;

        info!("Found tuner r820t");
    }

    /// Probe all known tuners at their I2C addresses
    /// and search for expected return values
    fn search_tuner(&self) -> Option<&str> {
        for tuner_info in KNOWN_TUNERS.iter() {
            let regval = self.i2c_read_reg(tuner_info.i2c_addr, tuner_info.check_addr);
            trace!(
                "Probing tuner {} at I2C address {:#02x} and checking address {:#02x}",
                tuner_info.name,
                tuner_info.i2c_addr,
                tuner_info.check_addr,
            );
            match regval {
                Ok(val) => {
                    trace!(
                        "Expecting value {:#02x}, got value {:#02x}",
                        tuner_info.check_val,
                        val
                    );
                    if val == tuner_info.check_val {
                        return Some(tuner_info.id);
                    }
                }
                Err(e) => {
                    warn!("Reading failed with {}, continuing", e);
                }
            };
        }
        None
    }

    pub fn detach_kernel_driver(&mut self) {
        let active = match self.handle.kernel_driver_active(self.iface_id) {
            Ok(true) => {
                self.handle.detach_kernel_driver(self.iface_id).ok();
                true
            }
            _ => false,
        };
        self.kernel_driver_active = active;
    }

    pub fn attach_kernel_driver(&mut self) {
        if self.kernel_driver_active {
            self.handle.attach_kernel_driver(self.iface_id).ok();
        }
    }

    pub fn claim_interface(&mut self) {
        self.handle.claim_interface(self.iface_id).unwrap()
    }

    pub fn write_reg(&self, block: u16, addr: u16, val: u16, len: u8) -> usize {
        let type_vendor_out =
            rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let mut data: [u8; 2] = [0, 0];
        let index: u16 = (block << 8) | 0x10;

        // switching endianness???
        data[0] = if len == 1 {
            (val & 0xff) as u8
        } else {
            (val >> 8) as u8
        };
        data[1] = (val & 0xff) as u8;

        match self
            .handle
            .write_control(type_vendor_out, 0, addr, index, &data, CTRL_TIMEOUT)
        {
            Ok(n) => n,
            Err(_) => 0,
        }
    }

    pub fn demod_read_reg(&self, page: u8, addr: u16, _len: u8) -> u16 {
        let type_vendor_in =
            rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        let data: [u8; 2] = [0, 0];
        let index: u16 = page.into();
        let addr = (addr << 8) | 0x20;
        let _res = self
            .handle
            .write_control(type_vendor_in, 0, addr, index, &data, CTRL_TIMEOUT);
        let reg: u16 = ((data[1] as u16) << 8) | (data[0] as u16);
        reg
    }

    pub fn demod_write_reg(&self, page: u8, addr: u16, val: u16, len: u8) -> u16 {
        let type_vendor_out =
            rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let mut data: [u8; 2] = [0, 0];
        let index: u16 = (0x10 | page).into();
        let addr = (addr << 8) | 0x20;

        data[0] = if len == 1 {
            (val & 0xff) as u8
        } else {
            (val >> 8) as u8
        };
        data[1] = (val & 0xff) as u8;

        let _res = self
            .handle
            .write_control(type_vendor_out, 0, addr, index, &data, CTRL_TIMEOUT);
        self.demod_read_reg(0x0a, 0x01, 1)
    }

    pub fn read_array(&self, block: u8, addr: u16, arr: &mut [u8]) -> usize {
        let type_vendor_in =
            rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        let index: u16 = (block as u16) << 8;
        self.handle
            .read_control(type_vendor_in, 0, addr, index, arr, CTRL_TIMEOUT)
            .unwrap()
    }

    pub fn i2c_read(&self, i2c_addr: u8, buf: &mut [u8]) -> usize {
        self.read_array(BLOCK_IICB, i2c_addr as u16, buf)
    }

    pub fn write_array(&self, block: u8, addr: u16, arr: &[u8]) -> Result<usize, rusb::Error> {
        let type_vendor_out =
            rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let index: u16 = ((block as u16) << 8) | 0x10;
        self.handle
            .write_control(type_vendor_out, 0, addr, index, arr, CTRL_TIMEOUT)
    }

    pub fn i2c_read_reg(&self, i2c_addr: u8, reg: u8) -> Result<u8, &str> {
        let addr: u16 = i2c_addr.into();
        let reg: [u8; 1] = [reg];
        let mut data: [u8; 1] = [0];

        match self.write_array(BLOCK_IICB, addr, &reg) {
            Ok(_res) => {
                self.read_array(BLOCK_IICB, addr, &mut data);
                Ok(data[0])
            }
            Err(_) => Err("I2C read error"),
        }
    }

    pub fn i2c_write(&self, i2c_addr: u8, buf: &[u8]) {
        self.write_array(BLOCK_IICB, i2c_addr as u16, buf);
    }

    pub fn i2c_write_reg(&self, i2c_addr: u8, reg: u8, val: u8) {
        let data: [u8; 2] = [reg, val];
        self.write_array(BLOCK_IICB, i2c_addr as u16, &data);
    }

    pub fn set_i2c_repeater(&self, on: bool) {
        let val = match on {
            true => 0x18,
            false => 0x10,
        };
        self.demod_write_reg(1, 0x01, val, 1);
    }

    pub fn test_write(&self) {
        self.write_reg(BLOCK_USBB, ADDR_USB_SYSCTL, 0x09, 1);
    }

    pub fn init_baseband(&self) {
        // init USB
        self.write_reg(BLOCK_USBB, ADDR_USB_SYSCTL, 0x09, 1);
        self.write_reg(BLOCK_USBB, ADDR_USB_EPA_MAXPKT, 0x0002, 2);
        self.write_reg(BLOCK_USBB, ADDR_USB_EPA_CTL, 0x1002, 2);

        // power on demod
        self.write_reg(BLOCK_SYSB, ADDR_SYS_DEMOD_CTL_1, 0x22, 1);
        self.write_reg(BLOCK_SYSB, ADDR_SYS_DEMOD_CTL, 0xe8, 1);

        // reset demod (bit 3, soft_rst)
        self.demod_write_reg(1, 0x01, 0x14, 1);
        self.demod_write_reg(1, 0x01, 0x10, 1);

        // disable spectrum inversion and adjacent channel rejection
        self.demod_write_reg(1, 0x15, 0x00, 1);
        self.demod_write_reg(1, 0x16, 0x0000, 2);

        // clear both DDC shift and IF frequency registers
        for i in 0..6 {
            self.demod_write_reg(1, 0x16 + i, 0x00, 1);
        }

        // set the FIR coefficients
        for i in 0..FIR_LENGTH {
            self.demod_write_reg(1, (0x1c + i) as u16, FIR_DEFAULT[i].into(), 1);
        }

        // enable SDR mode, disable DAGC (bit 5)
        self.demod_write_reg(0, 0x19, 0x05, 1);

        // init FSM state-holding register
        self.demod_write_reg(1, 0x93, 0xf0, 1);
        self.demod_write_reg(1, 0x94, 0x0f, 1);

        // disable AGC (en_dagc, bit 0) (this seems to have no effect)
        self.demod_write_reg(1, 0x11, 0x00, 1);

        // disable RF and IF AGC loop
        self.demod_write_reg(1, 0x04, 0x00, 1);

        // disable PID filter (enable_PID = 0)
        self.demod_write_reg(0, 0x61, 0x60, 1);

        // opt_adc_iq = 0, default ADC_I/ADC_Q datapath
        self.demod_write_reg(0, 0x06, 0x80, 1);

        // Enable Zero-IF mode (en_bbin bit), DC cancellation (en_dc_est),
        // IQ estimation/compensation (en_iq_comp, en_iq_est)
        self.demod_write_reg(1, 0xb1, 0x1b, 1);

        // disable 4.096 MHz clock output on pin TP_CK0
        self.demod_write_reg(0, 0x0d, 0x83, 1);
    }

    pub fn set_if_freq(&self, freq: u32) {
        let rtl_xtal: u32 = DEF_RTL_XTAL_FREQ; // need to apply PPM correction
        let base = 1u32 << 22;
        let if_freq: i32 = (freq as f64 * base as f64 / rtl_xtal as f64 * -1f64) as i32;

        let tmp = ((if_freq >> 16) as u16) & 0x3f;
        self.demod_write_reg(1, 0x19, tmp, 1);
        let tmp = ((if_freq >> 8) as u16) & 0xff;
        self.demod_write_reg(1, 0x1a, tmp, 1);
        let tmp = if_freq as u16 & 0xff;
        self.demod_write_reg(1, 0x1b, tmp, 1);
    }

    pub fn set_sample_rate(&self, samp_rate: u32) {
        let real_rsamp_ratio: u32;

        // check if the rate is supported by the resampler
        if (samp_rate <= 225000)
            || (samp_rate > 3200000)
            || ((samp_rate > 300000) && (samp_rate <= 900000))
        {
            error!("Invalid sample rate: {} Hz", samp_rate);
        }

        let mut rsamp_ratio: u32 = (DEF_RTL_XTAL_FREQ * (2 ^ 22)) / samp_rate;
        rsamp_ratio &= 0x0ffffffc;

        real_rsamp_ratio = rsamp_ratio | ((rsamp_ratio & 0x08000000) << 1);
        let real_rate: f64 = ((DEF_RTL_XTAL_FREQ * (2 ^ 22)) / real_rsamp_ratio).into();
        info!("Exact sample rate is: {} Hz", real_rate);

        self.set_i2c_repeater(true);
        // self.tuner.as_ref().unwrap().set_bw(real_rate as u32, &self);
        self.set_i2c_repeater(false);

        let mut tmp: u16 = (rsamp_ratio >> 16).try_into().unwrap();
        self.demod_write_reg(1, 0x9f, tmp, 2);
        tmp = (rsamp_ratio & 0xffff).try_into().unwrap();
        self.demod_write_reg(1, 0xa1, tmp, 2);
        // self.set_sample_freq_corr();

        // reset demod (bit 3, soft_rst)
        self.demod_write_reg(1, 0x01, 0x14, 1);
        self.demod_write_reg(1, 0x01, 0x10, 1);

        // self.set_offset_tuning();
    }

    pub fn reset_buffer(&self) {
        self.write_reg(BLOCK_USBB, ADDR_USB_EPA_CTL, 0x1002, 2);
        self.write_reg(BLOCK_USBB, ADDR_USB_EPA_CTL, 0x0000, 2);
    }

    pub fn deinit_baseband(&self) {
        // power off demod and ADCs
        self.write_reg(BLOCK_SYSB, ADDR_SYS_DEMOD_CTL, 0x20, 1);
    }
}

// impl Drop for RtlSdrDeviceHandle {
//     fn drop(self) {
//         self.deinit_baseband();
//         self.attach_kernel_driver();
//     }
// }
