extern crate libusb;

use std::time::Duration;
use libusb::{Direction, RequestType, Recipient};

const BLOCK_DEMODB: u16 = 0;
const BLOCK_USBB: u16 = 1;
const BLOCK_SYSB: u16 = 2;
const BLOCK_TUNB: u16 = 3;

const ADDR_USB_SYSCTL: u16 = 0x2000;
const ADDR_USB_CTRL: u16 = 0x2010;
const ADDR_USB_STAT: u16 = 0x2014;
const ADDR_USB_EPA_CFG: u16 = 0x2144;
const ADDR_USB_EPA_CTL: u16 = 0x2148;
const ADDR_USB_EPA_MAXPKT: u16 = 0x2158;
const ADDR_USB_EPA_MAXPKT_2: u16 = 0x215a;
const ADDR_USB_EPA_FIFO_CFG: u16 = 0x2160;

const ADDR_SYS_DEMOD_CTL: u16 = 0x3000;
const ADDR_SYS_DEMOD_CTL_1: u16 = 0x300b;

const INTERFACE_ID: u8 = 0;

const CTRL_TIMEOUT: Duration = Duration::from_millis(300);
const KNOWN_DEVICES: [(u16, u16, &str); 2] = [
    (0x0bda, 0x2832, "Generic RTL2832U"),
    (0x0bda, 0x2838, "Generic RTL2832U OEM")
];

const FIR_LENGTH: usize = 16;
const FIR_DEFAULT: [i16; FIR_LENGTH] = [
	-54, -36, -41, -40, -32, -14, 14, 53,	// 8 bit signed
	101, 156, 215, 273, 327, 372, 404, 421	// 12 bit signed
];

pub struct RtlSdr<'a> {
    ctx: &'a libusb::Context,
    dev: Option<libusb::DeviceHandle<'a>>,
    iface_id: u8,
    fir: [i16; FIR_LENGTH]
}

impl<'a> RtlSdr<'a> {
    
    pub fn new(ctx: &'a libusb::Context) -> RtlSdr<'a> {
        RtlSdr {
            ctx,
            dev: None,
            iface_id: INTERFACE_ID,
            fir: FIR_DEFAULT
        }
    }

    fn write_reg(&self, handle: &libusb::DeviceHandle, block: u16, addr: u16, val: u16, len: u8) -> usize {
        let type_vendor_out = libusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let mut data: [u8; 2] = [0, 0];
        let index: u16 = (block << 8) | 0x10;

        // switching endianness???
        data[0] = if len == 1 {
            (val & 0xff) as u8
        } else {
            (val >> 8) as u8
        };
        data[1] = (val & 0xff) as u8;

        match handle.write_control(type_vendor_out, 0, addr, index, &data, CTRL_TIMEOUT) {
            Ok(n) => n,
            Err(_) => 0
        }
    }

    fn demod_read_reg(&self, handle: &libusb::DeviceHandle, page: u8, addr: u16, len: u8) -> u16 {
        let type_vendor_in = libusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        let mut data: [u8; 2] = [0, 0];
        let index: u16 = page.into();
        let addr = (addr << 8) | 0x20;
        let res = handle.write_control(type_vendor_in, 0, addr, index, &data, CTRL_TIMEOUT);
        let reg: u16 = ((data[1] as u16) << 8) | (data[0] as u16);
        return reg;
    }


    fn demod_write_reg(&self, handle: &libusb::DeviceHandle, page: u8, addr: u16, val: u16, len: u8) -> u16 {
        let type_vendor_out = libusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let mut data: [u8; 2] = [0, 0];
        let index: u16 = (0x10 | page).into();
        let addr = (addr << 8) | 0x20;

        data[0] = if len == 1 {
            (val & 0xff) as u8
        } else {
            (val >> 8) as u8
        };
        data[1] = (val & 0xff) as u8;

        let res = handle.write_control(type_vendor_out, 0, addr, index, &data, CTRL_TIMEOUT);
        self.demod_read_reg(handle, 0x0a, 0x01, 1)
    }

    fn set_fir(&self, handle: &libusb::DeviceHandle) {
        let fir: [i16; 20];

        /* format: int8_t[8] */
        for i in 0..8 {
            let val = FIR_DEFAULT[i];
            if val < -128 || val > 127 {
                return;
            }
            fir[i] = val;
        }

        /* format: int12_t[8] */
        for idx in 0..4 {
            let i = idx * 2;
            let val0 = FIR_DEFAULT[8+i];
            let val1 = FIR_DEFAULT[8+i+1];
            if val0 < -2048 || val0 > 2047 || val1 < -2048 || val1 > 2047 {
                return;
            }
            fir[8+i*3/2] = val0 >> 4;
            fir[8+i*3/2+1] = (val0 << 4) | ((val1 >> 8) & 0x0f);
            fir[8+i*3/2+2] = val1;
        }

        for i in 0..20 {
            self.demod_write_reg(handle, 1, 0x1c + i, fir[i], 1);
        }
    }

    fn init_baseband(&self, handle: &libusb::DeviceHandle) {
        // init USB
        self.write_reg(&handle, BLOCK_USBB, ADDR_USB_SYSCTL, 0x09, 1);
        self.write_reg(&handle, BLOCK_USBB, ADDR_USB_EPA_MAXPKT, 0x0002, 2);
        self.write_reg(&handle, BLOCK_USBB, ADDR_USB_EPA_CTL, 0x1002, 2);

        // power on demod
        self.write_reg(&handle, BLOCK_SYSB, ADDR_SYS_DEMOD_CTL_1, 0x22, 1);
        self.write_reg(&handle, BLOCK_SYSB, ADDR_SYS_DEMOD_CTL, 0xe8, 1);

        // reset demod (bit 3, soft_rst)
        self.demod_write_reg(&handle, 1, 0x01, 0x14, 1);
        self.demod_write_reg(&handle, 1, 0x01, 0x10, 1);

        // disable spectrum inversion and adjacent channel rejection
        self.demod_write_reg(&handle, 1, 0x15, 0x00, 1);
        self.demod_write_reg(&handle, 1, 0x16, 0x0000, 2);

        // clear both DDC shift and IF frequency registers
        for i in 0..6 {
            self.demod_write_reg(&handle, 1, 0x16 + i, 0x00, 1);
        }

        self.set_fir(&handle);

        // enable SDR mode, disable DAGC (bit 5)
        self.demod_write_reg(&handle, 0, 0x19, 0x05, 1);

        // init FSM state-holding register
        self.demod_write_reg(&handle, 1, 0x93, 0xf0, 1);
        self.demod_write_reg(&handle, 1, 0x94, 0x0f, 1);

        // disable AGC (en_dagc, bit 0) (this seems to have no effect)
        self.demod_write_reg(&handle, 1, 0x11, 0x00, 1);

        // disable RF and IF AGC loop
        self.demod_write_reg(&handle, 1, 0x04, 0x00, 1);

        // disable PID filter (enable_PID = 0)
        self.demod_write_reg(&handle, 0, 0x61, 0x60, 1);

        // opt_adc_iq = 0, default ADC_I/ADC_Q datapath
        self.demod_write_reg(&handle, 0, 0x06, 0x80, 1);

        // Enable Zero-IF mode (en_bbin bit), DC cancellation (en_dc_est),
        // IQ estimation/compensation (en_iq_comp, en_iq_est)
        self.demod_write_reg(&handle, 1, 0xb1, 0x1b, 1);

        // disable 4.096 MHz clock output on pin TP_CK0
        self.demod_write_reg(&handle, 0, 0x0d, 0x83, 1);
    }

    pub fn init(&mut self) {
        for mut dev in self.ctx.devices().unwrap().iter() {
            let desc = dev.device_descriptor().unwrap();
            let vid = desc.vendor_id();
            let pid = desc.product_id();

            for kd in KNOWN_DEVICES.iter() {
                if kd.0 == vid && kd.1 == pid {
                    let mut handle = dev.open().unwrap();

                    let has_kernel_driver = match handle.kernel_driver_active(self.iface_id) {
                            Ok(true) => {
                                handle.detach_kernel_driver(self.iface_id).ok();
                                true
                            },
                            _ => false
                    };

                    let _iface = handle.claim_interface(self.iface_id).unwrap();

                    let res = self.write_reg(&handle, BLOCK_USBB, ADDR_USB_SYSCTL, 0x09, 1);
                    // reset device is write didn't succeed

                    self.init_baseband(&handle);

                    if has_kernel_driver {
                        handle.attach_kernel_driver(self.iface_id).ok();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let ctx = libusb::Context::new().unwrap();
        let mut rtlsdr = RtlSdr::new(&ctx);
        assert!(rtlsdr.dev.is_none());
        rtlsdr.init();
        assert!(rtlsdr.dev.is_some());
    }
}
