#![no_std]
#[macro_export]
macro_rules! bkpt_nr {
    ( $nr:expr ) => {
        unsafe {
            core::arch::asm!(
                "bkpt #{nr}",
                nr = const $nr,
                options(nomem, nostack, preserves_flags),
            );
        }
    };
}

#[macro_export]
macro_rules! bkpt {
    ( ) => {{
        lab1::bkpt_nr!(0);
    }};
}

// read the stack pointer value
#[macro_export]
macro_rules! read_sp {
    ( ) => {
        {
            #[allow(unused_assignments)]
            let mut sp_val: u32 = 0;

            unsafe {
                core::arch::asm!(
                    "mov {sp_val}, sp",
                    sp_val = out(reg) sp_val,
                    options(nomem, nostack)
                );
                sp_val
            }
        }
    }
}

#[inline(never)]
pub fn find_lowest() -> u32 {
    let mut addr = read_sp!();

    while unsafe { core::ptr::read_volatile(addr as *const u32) } != 0xcccc_cccc {
        addr -= 4;
    }
    addr + 4
}
