pub fn main() {
    println!("Starting rustl_fm");
    let mut r = rustl_sdr::RtlSdr::new();
    r.open();
}
