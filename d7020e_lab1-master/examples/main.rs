//! src/main.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use nrf52840_hal as _;
// use panic_rtt_target as _;
use panic_halt as _; // You need to enable `panic-halt` in the Cargo.toml file

#[rtic::app(device = nrf52840_hal::pac)]
mod app {
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");

        panic!("my custom panic message");
        #[allow(unreachable_code)]
        #[allow(clippy::empty_loop)]
        loop {}
    }
}
