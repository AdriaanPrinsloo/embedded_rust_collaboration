#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

use pg::delay;
use pg::led::{LEDS};

#[no_mangle]
pub fn main() -> ! {
    loop {
        let delay = 200;
        let lights_one = LEDS.iter();
        let mut lights_two = LEDS.iter();
        lights_two.next();

        let combined = lights_one.zip(lights_two);

        for (light_one, light_two) in combined {
            light_two.on();
            delay::ms(delay);
            light_one.off();
            delay::ms(delay);
        }

        let last_light = &LEDS[7];
        let first_light = &LEDS[0];
        first_light.on();
        delay::ms(delay);
        last_light.off();
        delay::ms(delay);
    }
}
