//! examples/hello_world.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use nrf52840_hal as _;
// use panic_halt as _; // You need to enable `panic-halt` in the Cargo.toml file
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
    fn init(_cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        for i in 0..10 {
            rprintln!("hello_world #{}", i); // <- put a breakpoint here
        }

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        some_function();
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#[inline(never)]
fn some_function() {
    rprintln!("in some_function");
    lets_crash(42);
}

#[inline(never)]
fn lets_crash(i: u32) {
    rprintln!("in lets_crash");
    panic!("let's crash with {}", i);
}
