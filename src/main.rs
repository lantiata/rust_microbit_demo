#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // panic handler
use nrf52833_hal as hal;
use embedded_hal::digital::OutputPin;
use hal::gpio::{Level};
use hal::pac::Peripherals;
use hal::Delay;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use rtt_target::{rprintln, rtt_init_print};
#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, micro:bit v2!");    
    let p = Peripherals::take().unwrap();

    // Röviden inicializáljuk a GPIO portokat
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    // micro:bit v2 LED 0 = P0.13 (példa)
    let mut led = port0.p0_28.into_push_pull_output(Level::Low);
    let mut led2 = port0.p0_21.into_push_pull_output(Level::Low);
    led2.set_high().unwrap();
    // Vegyük a Cortex-M perifériumokat
    let core_peripherals = cortex_m::Peripherals::take().unwrap();

    // Delay létrehozása SysTick alapján
    let mut delay = Delay::new(core_peripherals.SYST);
    let mut count = 0;
    loop {
        led.set_high().unwrap();
        delay.delay_ms(500_u32);
        rprintln!("Hello, micro:bit loop! Count: {}", count);  
        led.set_low().unwrap();
        delay.delay_ms(500_u32);
        count += 1;
    }
}