const DEFAULT_SAMPLE_RATE: u32 = 2048000;
const DEFAULT_BUF_LENGTH: usize = 16 * 16384;

pub fn main() {
    println!("Starting rustl_test");
    let r = rustl_sdr::RtlSdr::new();
    // r.set_sample_rate(DEFAULT_SAMPLE_RATE);
    // r.set_test_mode(true);
    // r.reset_buffer();
    println!("Reading buffers in sync mode");
    loop {
        let samples = r.read_sync(DEFAULT_BUF_LENGTH);
        println!(
            "Expected {} samples and got {}",
            DEFAULT_BUF_LENGTH,
            samples.len()
        )
    }
}
