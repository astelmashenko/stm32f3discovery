#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(t6: &tim6::RegisterBlock, ms: u16) {
    for _ in 0..4 * ms as u32 {
        aux9::nop()
    }
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();

    let mut leds = leds.into_array();

    // TODO initialize TIM6

    let ms = 1000;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
