pub struct Dongle {
    vendor: u16,
    product: u16,
    name: &'static str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dongle() {
        let _dongle = Dongle {
            vendor: 0x1234,
            product: 0x5678,
            name: "Test"
        };
        assert!(_dongle.name == "Test");
    }
}
