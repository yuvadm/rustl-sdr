# RusTL-SDR

A pure Rust implementation of the [RTL-SDR](https://github.com/osmocom/rtl-sdr/) driver, for shits and giggles.

This is mostly an exercise in writing low-level driver code.

Unusable, and highly unlikely to ever become usable. But it's fun.

## Usage

Install the crate in your `Cargo.toml`:

```toml
[dependencies]
rustl-sdr = "0.1"
```

Use in your code:

```rust
extern crate rustl_sdr;

fn foo() {
    rtlsdr = rustl_sdr::RtlSdr::new();
    rtlsdr.do_stuff()
}
```

## Dev

The usual:

```bash
$ cargo build
$ cargo test
```

Since there isn't (?) any good USB device mocking setup, for tests to pass an RTL-SDR device must be connected.

## License

[GPLv3](LICENSE)
