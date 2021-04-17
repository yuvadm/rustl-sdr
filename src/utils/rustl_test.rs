pub fn main() {
    println!("Starting rustl_test");
    let mut r = rustl_sdr::RtlSdr::new();
    r.open();
}
