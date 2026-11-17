# README for rtt_timing.rs

In order to assess the timing properties of a system we need to estimate the execution time of individual tasks.

To this end, the ARM provides a cycle counter (a wrapping 32 bit timer) as part of the `DWT` (Data Watchpoint and Trace) unit. We can obtain its current value through the `cortex-m` interface, where `DWT::cycle_count()` reads the current value.

---

## Exercises:

### A.1) What is the cycle count for the loop?

```shell
cargo run --example rtt_timing
```

```text
diff 1,440,211
```

### A.2) How many cycles per iteration?

1,440,211 ÷ 10,000 = **144.0211 cycles per iteration**.

### A.3) Why do we need a wrapping subtraction?

The 32-bit cycle counter wraps to zero after its maximum value, so `end` can
be smaller than `start`. `wrapping_sub` gives the elapsed cycles when the
measurement is shorter than one full counter period.

---

Now we try a release (optimized build, see `Cargo.toml` for build options).

### B.1) What is the cycle count for the loop?

```shell
cargo run --example rtt_timing --release
```

```text
start 3249574718, end 3249614723, diff 40005
```

### B.2) How many cycles per iteration?

40,005 ÷ 10,000 = **4.0005 cycles per iteration**.

What is the speedup (A/B)?

1,440,211 ÷ 40,005 ≈ **36.00× speedup over Debug**.

Why do you think it differs that much?

The Debug disassembly shows calls to the range iterator's `next()` and the
`nop()` wrapper on each pass. Release runs the same 10,000 iterations with
just three instructions in the loop body: `subs`, `nop`, and `bne`.

Now lets have a closer look at the generated assembly. For reference look at the assembly instruction set: [Table of processor instructions](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en)

```shell
cargo objdump --example rtt_timing --release -- --disassemble > rtt_timing.objdump
```

Open the file in your editor and search for the `timed_loop` function implementation.

### B.3) Assembly for `timed_loop`

Release disassembly from the baseline build before enabling the cache in E.1.

```asm
00000302 <_RNvCs1g1KSFAriwu_10rtt_timing10timed_loop>:
    302: f241 0104     movw r1, #0x1004  ;; Put 0x1004 in the lower 16 bits of r1.
    306: f242 7210     movw r2, #0x2710  ;; Set the loop counter to 10,000.
    30a: f2ce 0100     movt r1, #0xe000  ;; Fill the upper 16 bits. r1 is now 0xE0001004,
                                       ;; the address of the DWT cycle counter.
    30e: 6808          ldr  r0, [r1]    ;; Read the counter into r0 (start).

    310: 3a01          subs r2, #0x1    ;; Decrement the loop counter and update the zero flag.
    312: bf00          nop             ;; Execute one no-operation instruction.
    314: d1fc          bne  0x310      ;; Repeat from 0x310 if the counter is still nonzero.

    316: b580          push {r7, lr}   ;; Save r7 and the return address on the stack.
    318: 466f          mov  r7, sp     ;; Set r7 to the current stack pointer.
    31a: 6809          ldr  r1, [r1]    ;; Read the same counter into r1 (end).
    31c: bd80          pop  {r7, pc}   ;; Restore r7 and return, with start in r0 and end in r1.
```

Compare the `timed_loop` Rust code with the generated assembly. Make comments in the assembly (pasted above) explaining the source code translation into the generated assembly. In the comments you should state the purpose/usage of the `r0`, `r1` and `r2` registers, how the loop counter is initialized and compared, how the address to the `DWT::cycle_count` register is created, how the result is returned, etc.

Locate the loop body, and verify that it makes sense
based on the information from the technical documentation:

In your generated doc (`cargo doc --open`), search for `DWT::cycle_count()`, and paste its implementation.

```rust
pub fn cycle_count() -> u32 {
    // NOTE(unsafe) atomic read with no side effects
    unsafe { (*Self::PTR).cyccnt.read() }
}
```

Explain in your own words how this happens to read the right location in memory.

Hint:
The cortex-m core peripherals are found in the `cortex_m::peripheral::Peripherals` struct.

- Look at the implementation of `Peripherals` and check the implementation of `DWT` (and in particular what is `PTR`).
- Look for the declaration of `dwt::RegisterBlock` and locate the field `cyccnt` in the structure.
- Look for the declaration of `dwt::RegisterBlock` and locate the associated method `cycle_count()`.

Relate the offset of `cyccnt` to the DWT register table in the
[Arm Cortex-M4 Technical Reference Manual, section 9.3](https://documentation-service.arm.com/static/5f19da2a20b7cf4bc524d99a).

Now you should be able to understand and explain the implementation of `cycle_count()`.

The function follows `Self::PTR` to the DWT register block at `0xE0001000`.
`cyccnt` comes after the four-byte `ctrl` register, so it sits at
`0xE0001000 + 0x4 = 0xE0001004`. The volatile read returns the current count
as a `u32`. Both `ldr` instructions read that address.

### B.5) Latency per instruction in the loop.

Fill in the table below with cycle latencies for the instructions in the loop.

[Arm Cortex-M4 Technical Reference Manual, Table 3-1](https://documentation-service.arm.com/static/5f19da2a20b7cf4bc524d99a)

| Instruction | Cycles with zero memory wait states |
| --- | --- |
| `subs r2, #0x1` | 1 |
| `nop` | 1 |
| `bne 0x310` | `1 + P` when taken, 1 when falling through |

`P` is the cost of refilling the instruction pipeline, from 1 to 3 cycles.

### B.5) Estimated execution time in cycles.

Calculate the estimated loop count for executing the loop (it should iterate 10000 times right).

The branch is taken 9,999 times and falls through once. With `P = 1`, the
best-case loop estimate is `9,999 × 4 + 3 = 39,999 cycles`. With `P = 3`, the
estimate is `9,999 × 6 + 3 = 59,997 cycles` before adding flash wait states.

The Release measurement of 40,005 cycles is 6 above the best-case loop estimate.
The measured interval also includes the `push` and `mov` after the loop.

### B.6) Estimated execution time in cycles, including wait states.

Estimate the timing under the worksheet's assumptions of one extra cycle per
instruction fetch and `P = 3`.

`59,997 + 3 × 10,000 = 89,997 cycles`. This is a simplified estimate that
charges one separate stall to each of the three instructions per iteration.
It exceeds the measured 40,005 cycles. The measurement alone does not identify
the actual flash stall count or refill cost.

### C) General way to access Memory Mapped IO (MMIO).

First you need to generate the documentation:

```shell
cargo doc --open
```

This will document your crate and its dependencies, and open the docs in your default browser. Now, you can search the documentation by pressing `S` Search for `DWT` (the cycle counter is part of the Data Watchpoint and Trace peripheral).

You will get something like:

```rust
cortex_m::peripheral::dwt       Data Watchpoint and Trace unit
cortex_m::peripheral::DWT       Data Watchpoint and Trace unit
nrf52840_hal::pac::DWT          Data Watchpoint and Trace unit
nrf52840_pac::DWT               Data Watchpoint and Trace unit
...
```

The upper ones stem from the generic `cortex_m` crate, that provides interfaces for the `core` peripherals. (The `nrf` crate is merely re-exporting these for convenient access.)

Under the `cortex_m::peripheral::dwt` module you will find two structs: `Comparator`, and `RegisterBlock` (using the `Comparator struct`).

The `RegisterBlock` captures the hardware layout of the registers
and allow you to `read`, `write` and `modify` the underlying hardware, through `RW` (read/modify/write), `RO` (read), `WO` (write).

Now let take a close look at `cortex_m::peripheral::dwt::RegisterBlock`. We find that most of the registers are RW (Read-Write) but some are RO (Read-Only) (`pcsr` and `lsr`), and even WO (Write Only) (`lar`).

The structure was implemented by manually inspecting the data sheets from ARM and finding the right abstraction `RO/RW/WO` according to the specification.

(Through conditional compilation flags the correct abstraction is generated dependent on version of the architecture M0/M0+/M3/M4/M7 etc.)

### C.1 RW interface

We can further inspect the API by clicking on `RW`. We see that `write` and `modify` are marked `unsafe`. Explain in your own words why these are `unsafe`:

Writing a hardware register can change how the device behaves. The generic
`RW` wrapper cannot check whether a write is safe for that register and the
rest of the program. `modify` is also unsafe because it writes the result back.

The `RO` and `WO` types expose the operations their registers support.

Essentially, the user should never need to access the hardware at this level.

Instead `cortex_m::peripheral::DWT` is provided as user facing abstraction. It is a zero-sized struct that de-references into a `RegisterBlock` at the corresponding address.

### C.2 Finding the peripheral address

What address does `cortex_m::peripheral::DWT` de-reference to?
Hint, you need to view/follow the (source) code for `DWT::deref`, until you find the constant value of the pointer.

It dereferences to the `RegisterBlock` at `0xE0001000`. The implementation
returns `&*Self::PTR`.

### C.3 Methods vs. associated functions

Why does `enable_cycle_counter` require `&mut self`, while the associated
function `cycle_count` takes no receiver?

`enable_cycle_counter` changes the DWT control register, so the API requires
an exclusive borrow of the DWT handle. `cycle_count()` reads a fixed hardware
register without changing it, so it takes no receiver.

There are only a few different architectures in the ARM Cortex series, which makes it possible to develop and maintain hand written implementations in the `cortex-m` crate.

---

### D) PACs and HALs

There are dozens of manufacturers, each with a plethora of different chips (e.g., ST has 17 families , each with dozens of slightly different configurations [stm32-32-bit-arm-cortex-mcus](https://www.st.com/en/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus.html). Similarly there are many different chips from Nordic.

The plethora of chips makes it impractical to develop and maintain `RegisterBlock` implementations manually. Instead the tool `svd2rust` has been developed that reads vendor specifications, System View Description (SVD) files in xml format, and produces the corresponding PAC (Peripheral Access Crate).

Although it is possible to use the PAC files directly in user code, it is often better to provide hardware access at higher abstraction level.

To this end [embedded_hal](https://github.com/rust-embedded/embedded-hal) defines a set of APIs (traits).

The idea is the user code is written on top of the HAL traits (not the PAC), thus to relieve the user from hardware specifics. This offers a separation of concerns where the business logic goes in the user code, and the hardware specifics goes into the HAL trait implementation (which can be developed and maintained separately from the user code).

This also facilitates writing portable and re-usable code in the embedded space, in stark contrast to industrial practice where each brand defines its own HAL.

### D.1) Trait based HAL advantages

What advantages/opportunities can you foresee with this approach?

Code written against a HAL trait can work with different chips that implement
the same operations. Tests can use a fake implementation.

### D.2) Trait based HAL disadvantages

What disadvantages/obstacles can you foresee with this approach?

A common interface can leave out features specific to a chip, and each
supported chip needs an implementation. Hardware differences can still affect
the timing and performance of portable code.

---

## E) Enabling the instruction cache.

In the documentation for the nRF 52840, lookup the `ICACHECNF` register, you should find something like this:

```
ICACHECNF

Address offset: 0x540

I-Code cache configuration register.
```

The `CACHEEN` bit is disabled at reset. In [rtt_timing.rs](rtt_timing.rs),
`init` prints the bit, sets it with `modify(|_r, w| w.cacheen().enabled())`,
and reads it again. Both arguments to `modify` start with the current
register value. The closure changes only `CACHEEN`.
[Nordic nRF52840 NVMC register reference](https://docs.nordicsemi.com/r/bundle/ps_nrf52840/page/nvmc.html)

### E.1) Cache status and timing

With all numbered extra NOPs still commented out, run both commands on
the board. Press Ctrl+C after capturing the output from each run.

```sh
cargo run --example rtt_timing
cargo run --example rtt_timing --release
```

**Debug output**

```text
init
23:07:24.517: cacheen status: false
23:07:24.517: cacheen status: true
23:07:24.517: start timed_loop
23:07:24.517: start 3353009532, end 3354229736, diff 1220204
23:07:24.517: idle
```

**Release output**

```text
init
23:07:35.542: cacheen status: false
23:07:35.542: cacheen status: true
23:07:35.542: start timed_loop
23:07:35.542: start 3969401728, end 3969441733, diff 40005
23:07:35.542: idle
```

Both zero-extra-NOP counts were reproduced on the board in separate runs.

Commit and push the cache change and results after completing section E.

### E.2) Best case estimation vs. measured number of clock cycles

Now compare the measured cycle count, to the best case estimation (B5 above), what do you find?

The zero-extra-NOP Release run measured **40,005 cycles**, 6 cycles above
the B.5 best-case loop estimate of 39,999. It matched the earlier Release
measurement with the cache disabled.

The Release sweep added one NOP at a time before the first counter read.
The cache status changed from `false` to `true` in all 17 runs.

| Extra NOPs | Loop start (`subs`) | Measured cycles |
| ---: | ---: | ---: |
| 0 | `0x2fa` | 40,005 |
| 1 | `0x2fc` | 40,005 |
| 2 | `0x2fe` | 40,005 |
| 3 | `0x300` | 40,005 |
| 4 | `0x302` | 40,005 |
| 5 | `0x304` | 70,002 |
| 6 | `0x306` | 70,002 |
| 7 | `0x308` | 40,005 |
| 8 | `0x30a` | 40,005 |
| 9 | `0x30c` | 40,005 |
| 10 | `0x30e` | 40,005 |
| 11 | `0x310` | 40,005 |
| 12 | `0x312` | 40,005 |
| 13 | `0x314` | 70,002 |
| 14 | `0x316` | 70,002 |
| 15 | `0x318` | 40,005 |
| 16 | `0x31a` | 40,005 |

### E.3) Best case vs. worst case memory layout

Restore each measured setting in turn and use this command to print its
`timed_loop` disassembly. Record any ties.

```sh
cargo objdump --example rtt_timing --release -- --disassemble |
  sed -n '/<[^>]*timed_loop>:/,/^$/p'
```

**Best measured case: 40,005 cycles.** Settings 0–4, 7–12, and 15–16 tied.
The disassembly below is from the zero-extra-NOP setting.

```asm
000002ec <_RNvCs1g1KSFAriwu_10rtt_timing10timed_loop>:
     2ec: f241 0104    	movw	r1, #0x1004
     2f0: f242 7210    	movw	r2, #0x2710
     2f4: f2ce 0100    	movt	r1, #0xe000
     2f8: 6808         	ldr	r0, [r1]
     2fa: 3a01         	subs	r2, #0x1
     2fc: bf00         	nop
     2fe: d1fc         	bne	0x2fa <_RNvCs1g1KSFAriwu_10rtt_timing10timed_loop+0xe> @ imm = #-0x8
     300: b580         	push	{r7, lr}
     302: 466f         	mov	r7, sp
     304: 6809         	ldr	r1, [r1]
     306: bd80         	pop	{r7, pc}
```

**Worst measured case: 70,002 cycles.** Settings 5, 6, 13, and 14 tied.
The disassembly below is from the five-extra-NOP setting.

```asm
000002ec <_RNvCs1g1KSFAriwu_10rtt_timing10timed_loop>:
     2ec: f241 0104    	movw	r1, #0x1004
     2f0: bf00         	nop
     2f2: bf00         	nop
     2f4: bf00         	nop
     2f6: bf00         	nop
     2f8: bf00         	nop
     2fa: f242 7210    	movw	r2, #0x2710
     2fe: f2ce 0100    	movt	r1, #0xe000
     302: 6808         	ldr	r0, [r1]
     304: 3a01         	subs	r2, #0x1
     306: bf00         	nop
     308: d1fc         	bne	0x304 <_RNvCs1g1KSFAriwu_10rtt_timing10timed_loop+0x18> @ imm = #-0x8
     30a: b580         	push	{r7, lr}
     30c: 466f         	mov	r7, sp
     30e: 6809         	ldr	r1, [r1]
     310: bd80         	pop	{r7, pc}
```

Reruns of 0 and 5 extra NOPs reproduced both counts. The source and board
were restored to the zero-extra-NOP setting after the sweep.

### E.4) Memory layout analysis

Explain in your own words why the performance differs based on the memory layout for the loop, and the documentation for the cache operation. (The documentation is quite sparse in the product data sheet though.)

The measured loop uses the same three instructions in both builds. With
five extra NOPs before timing starts, the branch target moves from `0x2fa`
to `0x304` and the count rises by 29,997 cycles. All slow cases in this
sweep have branch targets 4 or 6 bytes past a 16-byte boundary.

Arm lists branch target alignment as a factor in pipeline refill time.
Nordic documents that cache hits have zero wait states and that misses fetch
from flash. The timings show a repeatable placement effect. Cache profiling
would be needed to determine whether cache misses contributed to it.
[Arm timing, section 3.3.1](https://documentation-service.arm.com/static/5f19da2a20b7cf4bc524d99a) ·
[Nordic nRF52840 NVMC cache](https://docs.nordicsemi.com/r/bundle/ps_nrf52840/page/nvmc.html)

---

### Optional exercise, align attribute

The optional experiments below were left unrun. The original worksheet
examples are reference material.

So, what we observed here is that memory layout can be very critical to the performance of code. Moving code around with adding `asm::nop` is not very stable, since it all depends on where the compiler (and linker) chose to place the code. To that end there are several alternatives:

- You may use the `align` attribute to the function, ensuring it will be mapped to an address being a multiple of the chosen alignment (this corresponds to an address evenly dividable by the alignment). Unfortunately this attribute is not yet stabilized so you would need a nightly compiler.
- Copy your `rtt_timing.rs` to `rtt_timing_align.rs`, and add that to your repository. In the newly created file:
- Un-comment the `#![feature(fn_align)]` attribute.
- Un-comment the `#[repr(align(64))]` attribute.
- Select the nightly toolchain, e.g. by:

  - `rustup override set nightly`
  - `rustup target add thumbv7em-none-eabi`
  - `rustup component add llvm-tools-preview`

Now, comment out the added `asm::nop()` and the program should execute with an optimal fit to the cache.

We now know that the `timed_loop` will be aligned to an even 64 bytes in memory, and we can reverse engineer the behavior of the cache and pipeline fill for _this specific_ chip.

Measure the time when having 0,..16 nop:s enabled. You should be able to see a pattern emerging.

[ At what interval do you spot a repetition? ]

---

### Optional exercise, align by linker script

Another alternative is to use the linker script (`memory.x`) to add a section in flash with a given alignment, ([memory-layout](https://docs.rust-embedded.org/embedonomicon/memory-layout.html)), and tell Rust to place the function in the given section, see [link-section](https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute). This is somewhat complicated, but should work.

If you want to experiment with this, I recommend you create a git branch, as you will fiddle with the linker script etc., and its easier to get back to the original setting if kept separate.

### Optional exercise, RAM function

- A better alternative is to force the function to be placed in RAM, since the RAM accesses should have 0 wait states. In order to do so:

  - Copy your `rtt_timing.rs` to `rtt_timing_ram.rs`, add that to your repo and work from there.
  - Uncomment the `#[link_section = ".data.timed_loop"]`.
  - Make an objdump and verify that the code now contains a trampoline:

    ```
    0000019c <__Thumbv7ABSLongThunk_timed_loop>:
     19c: f240 0c01    	movw	r12, #1
     1a0: f2c2 0c00    	movt	r12, #8192
     1a4: 4760         	bx	r12
    ```

    and the actual `timed_loop` code at address 0x2000_0000 (which is the start of the RAM area):

    ```
    20000000 <timed_loop>:
    20000000: f241 0104    	movw	r1, #4100
    20000004: f242 7210    	movw	r2, #10000
    20000008: f2ce 0100    	movt	r1, #57344
    2000000c: 6808         	ldr	r0, [r1]
    2000000e: 3a01         	subs	r2, #1
    20000010: bf00         	nop
    20000012: d1fc         	bne	0x2000000e <timed_loop+0xe> @ imm = #-8
    20000014: 6809         	ldr	r1, [r1]
    20000016: 4770         	bx	lr
    ```

    When the `elf` binary is flashed, this code is relocated to a flash address, and copied to RAM before the `init` task is executed.

    Now, according to the data sheet, we should enjoy the full potential of the Cortex-M4, with 0 wait state memory...... right ....

    Wrong.... unfortunately it does not help that the memory itself is 0 wait state, you have to access the memory over an internal bus.

    We put the code in RAM at address 0x2000_0000 and up. This access goes over the DCODE bus. For some weird reason it seems that this bus (DCODE) infers wait states.

    When running the code you will likely get some 80_000 ish clock cycles (that is worse than with the flash memory WITHOUT cache).

    How can we fix this?

    Well, looking at the data sheet section 4.2 Memory you will see that there is another bus (ICODE) going to the Code RAM. As it turns out the code and data ram refers to the same 256kByte memory, just mapped to different addresses. Thus address 0x2000_0000 accessed over the DCODE bus amounts to the same RAM address as address 0x0080_0000 accessed over the ICODE bus.

    We now have the function (`timed_loop`) in RAM at address 0x2000_0000, how can we then call it at address 0x0080_0000 instead?

    One solution (though ugly) is to cast the address:

    `let timed_loop_fn = unsafe { core::mem::transmute::<_, fn() -> (u32, u32)>(timed_loop as usize + 0x0080_0000 - 0x2000_0000) };`

    Now instead of calling `timed_loop()` (which is gives us 80_000 cycles), we can call `time_loop_fn()` (which gives us 40_000 cycles).

    In other words, calling the function over the "wrong" bus, gives as a 100% overhead. This is unfortunately not clear from reading the data sheet.

    With this information at hand you should be able to reconstruct both the run from RAM over the DCODE bus, and the ICODE bus, to verify the 100% overhead.

    Create two files `rtt_timing_dcode.rs` with the DCODE version, and `rtt_timing_icode.rs` with the ICODE version, and add them to your repository.

---

### F) Size of `timed_loop`

`cargo bloat -n 100 --example rtt_timing` reports **64 bytes** for
`timed_loop` in Debug. With `--release`, it reports **28 bytes**. The
Release assembly in B.3 includes a `push` and `pop` outside the loop.

### F.1) Size including trace output.

```shell
cargo size --example rtt_timing
```

```text
   text    data     bss     dec     hex  filename
  19016       0    1096   20112    4e90  rtt_timing
```

Now try a release build, what is the total size?

```sh
cargo size --example rtt_timing --release
```

```text
   text    data     bss     dec     hex  filename
   7296       0    1092    8388    20c4  rtt_timing
```

These sizes are for the traced build with the E.1 code and zero extra
NOPs. Flash occupied by the reported sections (`text + data`) is
**19,016 bytes in Debug** and **7,296 bytes in Release**. The `bss` column
counts zero-initialised RAM.

### F.2) Size, how low can you go?

Your assignment now is to get this down, by identifying the memory hogs. You can remove the tracing, replace the panic
handler by `panic_halt`, but besides that it should still measure time.

Make a copy of the `rtt_timing.rs` and name it `rtt_timing_opt.rs`.

You can use `_start/_end` in the caller to avoid erroring out.
It should be possible to get well below 1k of flash. Check that `timed_loop` has not been optimized out, and that its called (you will find the call in `rtt_timing_opt::app::rtic_ext::main:`).

Hint: make an objdump as done before, to verify that the call to `timed_loop` is not optimized away.

```shell
cargo size --example rtt_timing_opt --release
```

The [small example](rtt_timing_opt.rs) uses `panic_halt` and removes RTT
formatting and output. It keeps both counter reads and computes the elapsed
cycles. `black_box` keeps the computed value in this build.

```text
   text    data     bss     dec     hex  filename
    468       0       4     472     1d8  rtt_timing_opt
```

That is **468 bytes of flash** (`text + data`). The Release disassembly
shows the call at `0x186` and the two counter reads inside the function:

```asm
    186: f000 f806    bl   0x196 <_RNvCskF4i5ckJFOL_14rtt_timing_opt10timed_loop>
    18a: 1a08         subs r0, r1, r0

00000196 <_RNvCskF4i5ckJFOL_14rtt_timing_opt10timed_loop>:
    ...
    1a2: 6808         ldr  r0, [r1]
    1a4: 3a01         subs r2, #0x1
    1a6: bf00         nop
    1a8: d1fc         bne  0x1a4
    ...
    1ae: 6809         ldr  r1, [r1]
```

Don't forget to add and commit the `rtt_timing_opt.rs` to your repository.

---

### G) Summary

- The hardware counter measured a 36× speedup for the Release loop. The
  disassembly shows the simpler loop control.
- The DWT register layout explains how Rust reads a hardware counter
  through a memory address.
- With the cache enabled, changing the loop's address produced counts of
  40,005 or 70,002 cycles across the 17 tested layouts.
- The minimal build reports 468 bytes of flash and retains the timing code.
