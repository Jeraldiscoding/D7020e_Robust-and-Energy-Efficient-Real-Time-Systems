//! Minimal cycle counter example for Lab 1, section F.2.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use core::hint::black_box;
use cortex_m::{asm, peripheral::DWT};
use nrf52840_hal as _;
use panic_halt as _;

#[rtic::app(device = nrf52840_hal::pac)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        let (start, end) = timed_loop();
        black_box(end.wrapping_sub(start));

        (Shared {}, Local {}, init::Monotonics())
    }
}

#[inline(never)]
fn timed_loop() -> (u32, u32) {
    let start = DWT::cycle_count();
    for _ in 0..10_000 {
        asm::nop();
    }
    let end = DWT::cycle_count();
    (start, end)
}
