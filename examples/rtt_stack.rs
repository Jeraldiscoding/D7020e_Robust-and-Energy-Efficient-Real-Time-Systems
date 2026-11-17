//! examples/rtt_stack.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use lab1::*;
use panic_halt as _;

use nrf52840_hal as _;
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
        rprintln!("init");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        bkpt!();
        let (outer_fn_sp, inner_fn_sp) = outer_fn();
        let idle_sp = read_sp!();
        // bkpt!();
        let lowest_sp = find_lowest();
        print_stack_info(idle_sp, outer_fn_sp, inner_fn_sp, lowest_sp);
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#[inline(never)] // Forbid inlining.
fn outer_fn() -> (u32, u32) {
    bkpt!();
    let outer_fn_sp = read_sp!();
    let inner_fn_sp = inner_fn();
    (outer_fn_sp, inner_fn_sp)
}

// #[inline(always)]
#[inline(never)] // Forbid inlining.
fn inner_fn() -> u32 {
    bkpt_nr!(1);
    let stack_alloc = [7u8; 0x100]; // byte aligned (size of u8 is one byte)
    core::hint::black_box(stack_alloc); // prevent optimization
    read_sp!()
}

#[inline(never)] // Forbid inlining
fn print_stack_info(idle_sp: u32, outer_fn_sp: u32, inner_fn_sp: u32, lowest_sp: u32) {
    rprintln!("idle_sp {:x}", idle_sp);
    rprintln!("outer_fn_sp {:x}", outer_fn_sp);
    rprintln!("inner_fn_sp {:x}", inner_fn_sp);
    rprintln!("end_sp {:x}", lowest_sp);
}
