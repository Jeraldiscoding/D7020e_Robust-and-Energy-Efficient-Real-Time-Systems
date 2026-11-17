//! examples/rtt_timing.rs

// #![feature(fn_align)] // in orderer to align functions, (feature not yet stabilized)
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::{asm, peripheral::DWT};
// use panic_halt as _; // You need to enable `panic-halt` in the Cargo.toml file
use nrf52840_hal as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[rtic::app(device = nrf52840_hal::pac)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // E.1) Check the instruction cache before and after enabling it.
        let icachecnf = &cx.device.NVMC.icachecnf;
        rprintln!("cacheen status: {}", icachecnf.read().cacheen().is_enabled());
        icachecnf.modify(|_r, w| w.cacheen().enabled());
        rprintln!("cacheen status: {}", icachecnf.read().cacheen().is_enabled());

        rprintln!("start timed_loop");
        let (start, end) = timed_loop();
        rprintln!(
            "start {}, end {}, diff {}",
            start,
            end,
            end.wrapping_sub(start)
        );

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#[inline(never)] // Forbid inlining.

//#[repr(align(64))]
// #[link_section = ".data.timed_loop"] // for placing the function in RAM
fn timed_loop() -> (u32, u32) {
    //             // fill out number cycles obtained below when enabling nop:s
    //             // 0 nop  [number cycles]
    // asm::nop(); // 1 nop, [number cycles]
    // asm::nop(); // 2 nop, [number cycles]
    // asm::nop(); // 3 nop, [number cycles]
    // asm::nop(); // 4 nop, [number cycles]

    // asm::nop(); // 5 nop, [number cycles]
    // asm::nop(); // 6 nop, [number cycles]
    // asm::nop(); // 7 nop, [number cycles]
    // asm::nop(); // 8 nop, [number cycles]

    // asm::nop(); // 9 nop, [number cycles]
    // asm::nop(); //10 nop, [number cycles]
    // asm::nop(); //11 nop, [number cycles]
    // asm::nop(); //12 nop, [number cycles]

    // asm::nop(); //13 nop, [number cycles]
    // asm::nop(); //14 nop, [number cycles]
    // asm::nop(); //15 nop, [number cycles]
    // asm::nop(); //16 nop, [number cycles]
    let start = DWT::cycle_count();

    for _ in 0..10000 {
        asm::nop();
    }
    let end = DWT::cycle_count();
    (start, end)
}
