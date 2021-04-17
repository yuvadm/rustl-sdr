# RusTL-SDR

A pure Rust implementation of the RTL-SDR driver, for shits and giggles.

This is mostly an exercise in writing low-level driver code.

Unusable, and highly unlikely to ever become usable. But it's fun.

For real alternatives check out the original [librtlsdr](https://github.com/osmocom/rtl-sdr/) implementation or the high-level [rtlsdr_mt](https://github.com/kchmck/rtlsdr_mt.rs) Rust bindings.

## Usage

Install the crate in your `Cargo.toml`:

```toml
[dependencies]
rustl-sdr = "0.2"
```

Use in your code:

```rust
extern crate rustl_sdr;

fn foo() {
    rtlsdr = rustl_sdr::RtlSdr::new(&ctx);
    rtlsdr.init();
    rtlsdr.do_stuff();
}
```

## Dev

The usual:

```bash
$ cargo build
$ cargo test -- --nocapture
```

Since there isn't (?) any good USB device mocking setup, for tests to pass an RTL-SDR device must be connected.

## Design

### Overview

RusTL-SDR is very similar to the original rtl-sdr driver. It uses libusb, via the [rusb](https://github.com/a1ien/rusb/) bindings, as the main interface to issue commands to the rtl-sdr USB dongle.

### Lifecycle

Devices generally go through the following lifecycle:

1. Get a libusb context/handle, and find a compatible and supported RTL-SDR device
2. Initialize the device baseband
3. Probe the device for known tuners via the I2C interface
4. Run any special initialization required for the detected tuner
5. Interact with the device, usually this is where samples are read
6. Deinitialize the tuner
7. Deinitialize the baseband
8. Close the USB handle

## License

[GPLv3](LICENSE)
