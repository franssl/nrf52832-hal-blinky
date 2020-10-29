#![no_main]
#![no_std]

extern crate panic_halt;

use embedded_hal::digital::v2::OutputPin;
use nrf52832_hal as hal;
use nrf52832_hal::gpio::Level;

#[cortex_m_rt::entry]
fn main() -> ! {
    if let Some(p) = hal::pac::Peripherals::take() {
        let port0 = hal::gpio::p0::Parts::new(p.P0);
        let mut timer = hal::Timer::new(p.TIMER0);

        let mut led = port0.p0_18.into_push_pull_output(Level::Low);

        loop {
            led.set_low().unwrap();
            timer.delay(1000_u32); // 1 second
            led.set_high().unwrap();
            timer.delay(1000_u32);
        }
    }

    loop {
        continue;
    }
}