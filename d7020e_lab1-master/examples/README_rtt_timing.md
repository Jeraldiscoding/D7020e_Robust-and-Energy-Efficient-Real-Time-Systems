# README for rtt_timing.rs

In order to assess the timing properties of a system we need to estimate the execution time of individual tasks.

To this end, the ARM provides a cycle counter (a wrapping 32 bit timer) as part of the `DWT` (Data Watchpoint and Trace) unit. We can obtain its current value through the `cortex-m` interface, where `DWT::cycle_count()` reads the current value.

---

## Exercises:

### A.1) What is the cycle count for the loop?

```shell
cargo run --example rtt_timing
```

[ Your answer here ]

### A.2) How many cycles per iteration?

[ Your answer here ]

### A.3) Why do we need a wrapping subtraction?

[ Your answer here ]

---

Now we try a release (optimized build, see `Cargo.toml` for build options).

### B.1) What is the cycle count for the loop?

```shell
cargo run --example rtt_timing --release
```

[ Your answer here ]

### B.2) How many cycles per iteration?

[ Your answer here ]

What is the speedup (A/B)?

[ Your answer here ]

Why do you think it differs that much?

[ Your answer here ]

---

Now lets have a closer look at the generated assembly. For reference look at the assembly instruction set: [Table of processor instructions](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en)

```shell
cargo objdump --example rtt_timing --release -- --disassemble > rtt_timing.objdump
```

Open the file in your editor and search for the `timed_loop` function implementation.

### B.3) Assembly for `timed_loop`

[ Paste Assembly for function `timed_loop` here ]

Compare the `timed_loop` Rust code with the generated assembly. Make comments in the assembly (pasted above) explaining the source code translation into the generated assembly. In the comments you should state the purpose/usage of the `r0`, `r1` and `r2` registers, how the loop counter is initialized and compared, how the address to the `DWT::cycle_count` register is created, how the result is returned, etc.

Locate the loop body, and verify that it makes sense
based on the information from the technical documentation:

In your generated doc (`cargo doc --open`), search for `DWT::cycle_count()`, and paste its implementation.

[ Paste code for `DWT::cycle_count()` here ]

Explain in your own words how this happens to read the right location in memory.

Hint:
The cortex-m core peripherals are found in the `cortex_m::peripheral::Peripherals` struct.

- Look at the implementation of `Peripherals` and check the implementation of `DWT` (and in particular what is `PTR`).

- Look for the declaration of `dwt::RegisterBlock` and locate the field `cyccnt` in the structure.

- Look for the declaration of `dwt::RegisterBlock` and locate the associated method `cycle_count()`.

Relate the offset of the `cyccnt` to the documentation [Cortex-M4 Technical Reference Manual r0p0](https://developer.arm.com/documentation/ddi0439/b/Data-Watchpoint-and-Trace-Unit/DWT-Programmers-Model).

Now you should be able to understand and explain the implementation of `cycle_count()`.

[ Your own explanation of `cycle_count()` here ]

### B.5) Latency per instruction in the loop.

Fill in the table below with cycle latencies for the instructions in the loop.

[Table of processor instructions](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en)

| Instruction   | Cycle Count |
| ------------- | ----------- |
| subs r2, #0x1 |             |
| nop           |             |
| bne ... #-0x8 |             |

### B.5) Estimated execution time in cycles.

Calculate the estimated loop count for executing the loop (it should iterate 10000 times right).

[ Your estimated number of cycles here ]

Compare that to the measured values.

You will see that your calculated value does not correspond to the measured value.

### B.6) Estimated execution time in cycles, including wait states.

With the default settings the flash memory (for program storage) requires a "wait state" for accessing the flash memory. This will introduce one clock cycle latency for each instruction fetch.

Take the flash latency into account and calculate a new estimated value.

You may assume `P` to be 3 (the worst case).

[ Your answer here ]

Hint, This estimated value should correspond (roughly) to the measured value.

Later in this lab, we will see how we can enable the instruction cache, and reach the theoretical maximum performance of the M4.

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

[ Your answer here ] 

This API prevents the user from accidental UB (e.g., reading to a write only register), and/or attacks from safe code. (Using *unsafe* code you can `break` any abstraction.)

Essentially, the user should never need to access the hardware at this level.

Instead `cortex_m::peripheral::DWT` is provided as user facing abstraction. It is a zero-sized struct that de-references into a `RegisterBlock` at the corresponding address.

### C.2 Finding the peripheral address

What address does `cortex_m::peripheral::DWT` de-reference to?
Hint, you need to view/follow the (source) code for `DWT::deref`, until you find the constant value of the pointer.

[ Your answer here ]

While it is in general unsafe to operate directly on the raw register block, a set of functions/methods on the `cortex_m::peripheral::DWT` struct.

### C.3 Methods vs. associated functions

Why do you think `enable_cycle_counter` method requires an `&mut self`, while the associated function `cycle_counter` does not?

[ Your answer here ]

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

[ Your answer here ]

### D.2) Trait based HAL disadvantages

What disadvantages/obstacles can you foresee with this approach?

[ Your answer here ]

---

## E) Enabling the instruction cache.

In the documentation for the nRF 52840, lookup the `ICACHECNF` register, you should find something like this:

```
ICACHECNF

Address offset: 0x540

I-Code cache configuration register.
```

The `CACHEEN` bit controls the instruction cache (0-disable/1-enable).

Your task here is to enable the instruction cache.

Now that you are familiar with the documentation, lookup the `cacheen` in your crate's documentation (`S cacheen` should get you there). You will find both a reader and writer interface (accessible both through the `hal` and directly in the `pac`, use the `pac` one).

Figure out how to `read` the cache bit and print its value in init.
Hint 1: uncomment the commented line:

```rust
// let icachecnf = &cx.device.NVMC.icachecnf;
```

Hint 2, use the `read()` to get the register value, use `cacheen()` to get the field value, use `is_enabled()` to inspect the field value.

Run the application and confirm the you get a `false` value printed.

Now, use the `modify(|_r, w| ...)` API to change the field value. (Here `r` will be the reset default, and `w` the current value. Set the `cacheen()` bit by `enabled()` based on the current value passed in `w`.

Finally, add code to print the `cacheen` status again (to confirm that it has been correctly set).

Now run the application again.

You should have a trace similar to this (if not, go back and fix your code).

```
cacheen status: false
cacheen status: true
```

### E.1)

Paste your program output for a debug/dev build:

[ Program output here ]

Paste your program output for a release build:

[ Program output here ]

Commit the changed source code `rtt_timing.rs` and push it to your git repo.

### E.2) Best case estimation vs. measured number of clock cycles

Now compare the measured cycle count, to the best case estimation (B5 above), what do you find?

[ Your answer here ] 

Now, with a bit of luck the loop fits perfectly in the cache and you should not have any wait states triggered. However, the cache and pipeline fill is not very advanced/smart, so it depends largely on the address alignment. Try "moving" the loop by un-commenting one of the `asm::nop()` instructions. Re-run your application to see if the cycle count changed. Iterate, this (un-comment another) and try to find the "best" and "worst" case (lowest/highest number of cycles).

### E.3) Best case vs. worst case memory layout

Now, for the best(worst) case, make an objdump and paste the `timed_loop`, below.

[ Best case objdump ]

[ Worst case objdump ]

Compare the memory layout for the tight loop, (the address of the instructions). Confer to the nRF documentation for the I-cache.

### E.4) Memory layout analysis

Explain in your own words why the performance differs based on the memory layout for the loop, and the documentation for the cache operation. (The documentation is quite sparse in the product data sheet though.)

[ Your answer here ]

---

### Optional exercise, align attribute

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

### F) Finally some statics

For embedded it is often/typically crucial to reduce overhead,

- code footprint (flash memory)
- memory footprint (sram memory)
- execution OH (speed/power consumption)

We have already looked execution overhead and can conclude
Rust to do an excellent job. The RAM requirement for the application amounts only to the buffers (allocated for RTT), and the stack for the functions (which is in this case 0 for the `timed_loop`).
(Look at the code, there is no pushing/popping, no local stack.)

Let's have a look at the code footprint (flash).

```shell
cargo bloat -n 100 --example rtt_timing --release
...
0.0%   0.4%    24B  rtt_timing rtt_timing::timed_loop
...
```

That is, the timed loop takes only 24 bytes of memory!

What is the size for `timed_loop` in a debug build?

[ Your answer here ]

### F.1) Size including trace output.

```shell
cargo size --example rtt_timing
```

[ Your output here ]

Now try a release build, what is the total size?

[ Your output here ]

The overall code size should be around than 7kByte of flash, so a fair amount smaller than your debug/dev build.

However, imagine we want to run the code on a really tiny device, like Apollo11 guidance computer (having a total of some 64kByte of manually woven copper-wire "ROM"), we would want to squeeze the code to its minimum.

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

[ Your answer here ]

Don't forget to add and commit the `rtt_timing_opt.rs` to your repository.

---

### G) Summary:

What are the learning outcomes.

- You have confirmed that RTIC is extremely light weight (zero-cost in release build).
  Applications can be be less than 1k flash. Ideal for IoT, or Apollo11!.

- You have seen that RTIC applications are easy to trace using RTT
  If more details are required, you may use a stop and inspect debugger (like `probe-rs-debug` or `gdb`).

- You have confirmed that Rust generates:
  really, really, bad code in debug build (beware!).
  really, really, really good code in release build!

- You have setup timing measurements which are
  (Almost) cycle accurate (0.000 000 0x s at 100MHz).
  Consistent (down to clock cycle).
  Predictable (you eventually got what you expected right?)

- Dealing with hardware puts another dimension to coding. In order to achieve the maximum performance you need to understand the underlying hardware and all its subtle details.

- Unfortunately, neither ARM nor Nordic is disclosing their designs, making it hard/impossible to know exactly how the code will execute.

  To that end, in the optional assignments have shown how you can reverse engineer the behavior of the pipeline fill/cache, ta get a stable/predictable behavior.

  The advent of the RISC-V architecture allows for complete open source MCU implementations. Hopefully companies understand/appreciate the benefit and shares their designs (instead of just preying on the open source designs for their own profit).

  This would allow You as a programmer to better predict the behavior of your code, and thus allow for better software to be built. Moreover, tools for SW analysis will benefit from open source hardware to improve the precision of code analysis.
