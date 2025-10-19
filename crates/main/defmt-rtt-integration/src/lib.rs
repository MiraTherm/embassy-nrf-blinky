#![no_main]
#![no_std]

use defmt_rtt as _;
use embassy_nrf as _;
use panic_probe as _;


#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    semihosting::process::exit(0);
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    semihosting::process::exit(1);
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn testing_possible() {
        assert!(true)
    }
}