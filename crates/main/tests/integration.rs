#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use embassy_nrf as _;

#[cfg(test)]
#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn integration_testing_possible() {
        assert!(true)
    }
}