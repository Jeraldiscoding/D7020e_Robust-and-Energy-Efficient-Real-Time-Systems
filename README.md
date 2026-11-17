# RTIC on the nRF52840DK board

## Resources

- [52840](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fstruct_nrf52%2Fstruct%2Fnrf52840.html), you may also download the Product Specification (v1.7) as a pdf.

- [Table of processor instructions](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en), for the Cortex M4 architecture. You may also download the whole Cortex-M4 Technical reference manual.

- [ARM and Thumb-2 Instruction Set Quick Reference Card](https://developer.arm.com/documentation/qrc0001/m), pdf only.

- [Cortex-M4 Technical Reference Manual r0p0](https://developer.arm.com/documentation/ddi0439/b/Data-Watchpoint-and-Trace-Unit/DWT-Programmers-Model)

---

## Note for Mac Users

Before you proceed, you may need Xcode Command Line Tools installed. This is a set of general software development tools provided by Apple, so good idea to have them in any case if you're planning to do software development in the future.
You can install these by running
```shell 
xcode-select --install
``` 
and following the instructions.
## Rust

We assume Rust to be installed using [rustup](https://www.rust-lang.org/tools/install).

Additionally you need to install the `thumbv7em-none-eabi` target and the `llvm-tools-preview`.

```shell
rustup target add thumbv7em-none-eabi
rustup component add llvm-tools-preview
```

### Cargo Subcommands

We assume the following Cargo sub-commands (tools) are in place:

- [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)
- [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat)

```shell
cargo install cargo-binutils cargo-bloat
cargo install probe-rs-tools --locked
```

You may find other Cargo sub-commands useful as well:

- `cargo-edit`, allows you to manage dependencies from the command line.
- `cargo-expand`, to view/generate (Rust source) files after macro expansion.
- `nrf-recover`, unlocks your nRF52 device using CMSIS-DAP or JLink probe. (Alternatively you may use the `nrfjprog` command discussed below.)
- `cargo-update`, allows you to update all cargo installed utilities by running:
  `cargo install-update -a`

---

## Nordic utilities
Tools for working with the Nordic range of MCUs.

### Linux
Under Arch you can install them through `aur`:

```shell
yay nrf5x-command-line-tools
```
### MacOS
You can find the tools available [here](https://www.nordicsemi.com/Products/Development-tools/nRF-Command-Line-Tools/Download). Note that you need both of the tools bundled with the download, so both J-LINK and the Nordic utilities.
### General
To unlock the 52840DK (allowing you to program it over SWD) using `probe-rs` you run:

```shell
probe-rs erase --chip=nrf52840_xxAA --allow-erase-all
```

You can alternatively do the same using the `nrf5x-command-line-tools`:

```shell
nrfjprog --recover
```

This operation might take some time (it erases the complete flash memory and flash protection bits).

Notice, the nRF 52840DK comes with the flash protected, so you must run either of the above to unlock the device.

---

## Vscode

Vscode allows to install plugins extending its functionality. For the labs you will need:
- `rust-analyzer` and the
- `probe-rs debugger` plugins.

You may also install the 
- `dependi`, plugin to get `toml` versioning information, 
- `even-better-toml`, to get `toml` file formatting, and
- `Arm Assembly`, to get syntax highlighting and formatting for assembly code.

Another great plugin is `Code Spell Checker`, to ensure both your code, comments and other files are reasonably spelled (there is no grammar checking).

Hint: you can setup a `profile` in vscode to isolate the configuration if you already have vscode setup for other languages.

---

## Application Setup

This crate is contains a ready made configuration for compiling, running/tracing Rust RTIC on the `nRF52840DK`.

- `Cargo.toml` specifies all dependencies.
- `.cargo/config.toml` specifies further build options. By default `cargo run` will use `probe-rs run` as specified:

  ```toml
  runner = "probe-rs run --chip nRF52840_xxAA"
  ```

- `memory.x` specifies the memory layout for the target. The 52840 has a 1Mb onboard flash (starting at address 0x0000_0000), and 256k of continuous RAM (starting at address 0x2000_0000).

  ```shell
  {
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    /* TODO Adjust these memory regions to match your device memory layout */
    /* These values correspond to the nRF 52840 */
    FLASH : ORIGIN = 0x00000000, LENGTH = 1M
    RAM : ORIGIN = 0x20000000, LENGTH = 256K
  }
  ```

## Exercises

- `examples/main.rs`

  Developing embedded applications in Rust is made simle by the RTIC framework. In this exercise you will familiarize yourself with the basics `init` and `idle`, and see how you can trace the output to a terminal using `cargo run`.

  You will also learn about `panic`s and how they can be traced.

- `examples/hello_world.rs`

  Here you learn how to use `vscode` based tracing and debugging.

- `examples/rtt_timing.rs`

  Here you will learn about (almost) cycle accurate timing measurements:

  - Using instrumentation code (which introduces bloat and overhead).

  - Code generation optimization.

  - Code inspection, `objdump`.

  - Code trimming, RTIC is "A Zero-Cost Abstraction for Memory Safe Concurrency".

- `examples/rtt_stack.rs`

Each exercise has a corresponding `README_<file_name>.md`, with detailed information on expected learning outcomes and instructions how to carry out the lab.

For each exercise, remember to use git commits frequently so if you mess something up you can always revert.

---

## A note on embedded programming and debugging

### Nordic 52840DK

The Nordic 52840DK is an entry level development kit for embedded programming based on the ARM Cortex M4 architecture. It features an on-board programmer (jlink) allowing applications to be both programmed onto the MCU (flashed), debugged and traced.

In these examples we use RTT (Real-Time Transfer) for instrumentation/tracing. It is very convenient as we don't need to setup some extra communication link between the host (PC) and the target (MCU), instead tracing is going over the already present debug interface.

### Probe

[probe.rs](https://probe.rs) is a set of libraries and utilities aiming to facilitate embedded programming in general and embedded Rust in particular. In this lab you will try some different alternatives based on probe.

- `probe-rs` tool, leverages the `probe-rs` library to flash and trace application (`probe-rs run`).
- `cargo embed` is another `probe-rs` base utility similar to the `probe-rs` tool, offering more features and flexibility.
- `probe-rs debug` is a `vscode` plugin using `probe` tooling for integrated debugging.

### Overall debug scenario

The `probe-rs run` is configured in the `.cargo/config.toml` (as the "runner" for this project). It will:

- First establish a connection to the target.

- Then periodically poll a predefined memory region on the target for tracing output and convey this to the host terminal. This technique is known as RTT (Real-Time trace). 
- Listen to messages from the onboard programmer, e.g., status such as breakpoints hit, or errors occurred.

### RTT Pros and Cons

RTT is very easy/convenient and in many cases less intrusive to the embedded application than tracing over some (blocking) serial link. It is also relatively fast (the majority of overhead on the target side will be to format the data into characters).

The cons is that it requires a host connected initially, so you cannot easily tap-into an already running system in an ad-hoc manner. For the purpose of development debugging/tracing this is not a big issue, but for maintenance its a no go.

Another cons is that the host actively probes the target, so you cannot easily trace systems going in and out of low power states (where the debug unit might be powered down).

Another cons is that the active polling of memory introduces bus load (the MCU and the debug unit cannot access the memory simultaneously, introducing wait states in case of congestion). This is not a big deal in practice but it may affect cycle accurate measurements.

Another drawback is the memory management/queue handling, which by default introduces critical sections, thus unsuitable to real-time tracing. This can be solved by splitting the RTT transfer into exclusively accessed channels. This requires resource management on the application side. RTIC can elegantly solve this problem.

### Deferred formatting

Using `probe` you can use `defmt` to defer formatting to the host side to reduce overhead on the target side, we don't need it for our purpose but you might find it useful to developing other applications.

### Trouble shooting

If you get problems connecting to the device, run in terminal:

```shell
nrfjprog --recover
```

---

## Learning outcomes

- First exposure to embedded programming in Rust.

- Tracing and panic handling.

- You have observed the effect of optimization (going from very very inefficient to very very efficient)

- You have seen that RTIC applications are easy to trace using RTT

  If more details are required, you may use a "stop and inspect" debugger, e.g., the  `probe-rs-debugger` (from within vscode).

- You have seen how the embedded Rust ecosystem is designed with `cortex-m` dealing with the generic functionality and embedded HAL abstracting hardware from the end user.

- You have confirmed that RTIC is extremely light weight (zero-cost in release build).

  Applications can be be less than 1k flash. Ideal for small systems, like sensors, actuators and IoT devices in general.

- You have setup timing measurements which are:

  - (Almost) cycle accurate (0.000 000 0x s at 100MHz).

  - Consistent (down to a few clock cycles).

  - Predictable.

- You now master common tooling to inspect generated code.

- You are now acquainted with inline assemble and the use of libraries to wrap these into safe abstractions.

- You are now familiar with "stack overflow". While Rust guarantees memory safety and defined behavior, stack behavior goes outside the language guarantees (overrunning the heap leads to Undefined Behavior (UB)). You now master the state-of-practice, using "stack-painting" to get a snapshot of tainted memory locations which allows you to measure slack.  

Already at this point you have been exposed to many of the challenges with embedded programming in general, and how Rust RTIC helps you address these.

