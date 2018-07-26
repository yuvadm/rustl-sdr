extern crate libusb;

mod lib;

use lib::RtlSdr;

fn main() {
    println!("Running");
    let ctx = libusb::Context::new().unwrap();
    let mut rtlsdr = RtlSdr::new(&ctx);
    rtlsdr.init();
}
