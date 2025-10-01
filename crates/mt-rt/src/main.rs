#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use nrf52840_hal as hal;
use hal::pac;
use hal::gpio::p0::Parts;
use hal::gpio::Level;
use embedded_hal::digital::{OutputPin};

use hal::Delay;
use embedded_hal::delay::DelayNs;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = Parts::new(p.P0);

    let mut led = port0.p0_15.into_push_pull_output(Level::Low);

    let core_p = pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core_p.SYST); 

    loop {
        led.set_high().ok();
        delay.delay_ns(1_000_000_000); 
        led.set_low().ok();
        delay.delay_ns(1_000_000_000);
    }
}
