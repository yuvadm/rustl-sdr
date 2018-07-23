extern crate libusb;

mod lib;

use lib::RtlSdr;

fn main() {
    println!("Running");
    let mut rtlsdr = RtlSdr::new();
    rtlsdr.init();
}
