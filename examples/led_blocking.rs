#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{
    display_pins,
    hal::{gpio::p0::Parts as P0Parts, prelude::*, Timer},
};

use microbit::led;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::pac::Peripherals::take() {
        let mut timer = Timer::new(p.TIMER0);
        let p0parts = P0Parts::new(p.GPIO);

        // Display
        let pins = display_pins!(p0parts);
        let mut leds = led::Display::new(pins);

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_R = [
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_u = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_s = [
            [0, 0, 0, 0, 0],
            [0, 0, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_t = [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        loop {
            leds.display(&mut timer, letter_I, 1000);
            leds.display(&mut timer, heart, 1000);
            leds.display(&mut timer, letter_R, 1000);
            leds.display(&mut timer, letter_u, 1000);
            leds.display(&mut timer, letter_s, 1000);
            leds.display(&mut timer, letter_t, 1000);
            leds.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
