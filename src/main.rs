extern crate libusb;

mod lib;

use lib::RtlSdr;

fn main() {
    println!("Running");
    let rtlsdr = RtlSdr::new();
    rtlsdr.find();
}
