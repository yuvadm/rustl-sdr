const _DEFAULT_SAMPLE_RATE: u32 = 2048000;
const DEFAULT_BUF_LENGTH: usize = 16 * 16384;

pub fn main() {
    println!("Starting rustl_test");
    let _buffer: [u8; DEFAULT_BUF_LENGTH];
    let mut r = rustl_sdr::RtlSdr::new();
    r.open();
}
