#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use embassy_nrf as _;

use embassy_executor::Spawner;
use embassy_time::{
    Timer, 
    Duration
};
use embassy_nrf::{
    gpio::{
        Level, 
        Output,
        OutputDrive
    }, 
    bind_interrupts,
    config::Config
};

bind_interrupts!(struct Irqs {
    // Add interrupts here if needed
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_nrf::init(config);
    let output_config = OutputDrive::Standard;
    let led: Output<'static> = Output::new(p.P0_15, Level::Low, output_config);
    spawner.spawn(blinker(led)).unwrap();
}

#[embassy_executor::task]
async fn blinker(mut led: Output<'static>) {
    loop {
        led.set_high(); 
        defmt::info!("LED ON");
        Timer::after(Duration::from_secs(1)).await;
        led.set_low();
        defmt::info!("LED OFF");
        Timer::after(Duration::from_secs(1)).await;
    }
}
