//! examples/rtt_timing_lab3_per.rs

// #![feature(fn_align)] // in orderer to align functions, (feature not yet stabilized)
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
// use panic_halt as _; // You need to enable `panic-halt` in the Cargo.toml file
use nrf52840_hal as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[rtic::app(device = nrf52840_hal::pac, dispatchers = [TIMER1])]
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

        // E.1)
        // let icachecnf = &cx.device.NVMC.icachecnf; // <- un-comment me
        // code to print the status of the instruction cache enable status
        // code to enable the instruction code
        // code to print status after enabling

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        let _ = measure::spawn(9);

        #[allow(unreachable_code)]
        loop {
            continue;
        }
    }

    #[task()]
    fn measure(_cx: measure::Context, n: u8) {
        rprintln!("start timed_loop");
        let (start, end) = timed_loop(n);
        rprintln!(
            "start {}, end {}, diff {}",
            start,
            end,
            end.wrapping_sub(start)
        );
    }
}

// recursive
pub fn sum_recursive(n: u8) -> u32 {
    match n {
        0 => 0,
        _ => n as u32 + sum_recursive(n - 1),
    }
}

// iterative
pub fn sum_iterative(n: u8) -> u32 {
    let mut sum = 0;
    for v in 0..=n {
        sum += v as u32
    }
    sum
}

// mathematical formula
pub fn sum_formula(n: u8) -> u32 {
    let n: u32 = n as u32;
    n * (n + 1) / 2
}

#[inline(never)] // or some of the below tricks

// #[repr(align(64))]
// #[link_section = ".data.timed_loop"] // for placing the function in RAM
fn timed_loop(n: u8) -> (u32, u32) {
    let start = DWT::cycle_count();
    // uncomment one at a time

    let r = sum_recursive(n);
    // let r = sum_iterative(n);
    // let r = sum_formula(n);
    let end = DWT::cycle_count();
    rprintln!("r {}", r);
    (start, end)
}
