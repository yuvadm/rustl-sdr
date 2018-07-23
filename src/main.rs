extern crate libusb;

fn main() {
    println!("Running");
    let ctx = libusb::Context::new().unwrap();
    for mut dev in ctx.devices().unwrap().iter() {
        let desc = dev.device_descriptor().unwrap();
        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            dev.bus_number(),
            dev.address(),
            desc.vendor_id(),
            desc.product_id());
    }
}
