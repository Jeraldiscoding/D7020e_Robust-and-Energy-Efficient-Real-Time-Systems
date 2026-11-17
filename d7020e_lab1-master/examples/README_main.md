# Readme for main.rs

A small warmup example to get you going with RTIC and embedded Rust.

---

## Expected learning outcomes

- Compile and run a simple RTIC application on the target.

- Tracing to terminal.

- Panic handling in Rust RTIC.

---

## Exercises

### A) A Simple Trace

You can run the application by `cargo run`, press Ctrl-C to stop it.

```shell
cargo run --example main
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabi/debug/examples/main`
      Erasing ✔ [00:00:00] [###########################################################] 16.00 KiB/16.00 KiB @ 30.23 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [###########################################################] 16.00 KiB/16.00 KiB @ 25.58 KiB/s (eta 0s )    Finished in 1.183s
...
^C⏎

```

What is the output?

```text
init
21:14:35.049: idle
```

Explain in your own words what the program does.

The program prints `init`, then prints `idle` once. The empty `loop {}` keeps running without doing anything. Since the print statement is before the loop, it is not repeated.

### B) Panic tracing

Rust is designed for reliability, with the aim to deliver memory safety and defined behavior at all times.

Recoverable errors (in libraries and user code) should use the `Result<T,E>` type, while unrecoverable errors should `panic`.

Let's introduce a non-recoverable `panic` (uncomment line 34).

```shell
cargo run --example main
```

What is the output?

```text
init
21:19:09.612: idle
21:19:09.612: panicked at examples/main.rs:34:9:
21:19:09.612: my custom panic message
```

### C) Panic halt

Tracing is nice during development, but requires a debugger attached and a host listening. For a deployed product, other `panic` behavior should be adopted (e.g. storing to flash, for later post-mortem debugging) or just reset:ing the device. In this example we chose just to `halt`.

Enable `panic_halt` (uncomment line 10, and comment out line 9).

```shell
cargo run --example main
```

What is the output?

```text
init
21:22:23.173: idle
```

Explain in your own words why the output changed.

We switched the panic handler from `panic_rtt_target` to `panic_halt`. The program still reaches the same `panic!` call, but `panic_halt` immediately loops forever without printing panic information. In contrast, `panic_rtt_target` prints the panic location and message through RTT before looping forever. The `init` and `idle` messages still appear because the program prints them before the panic, and ordinary RTT tracing is still enabled.

### D) Find the source:

Figure out how to find the source of `panic_halt`, and look at the implementation.

- `cargo doc --open`
- `crates.io` (you can also find/view published crates from the crates site)

Source: `panic-halt` version `1.0.0`, `src/lib.rs`.

```rust
use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

Look at the source code and explain in your own words what it does.

The #[panic_handler] attribute makes Rust call this function when a panic occurs. It ignores the panic information and enters an empty infinite loop without printing anything. The ! return type means the function never returns

---

## Pushing to git

Once done with this part you should:

- `git add .`, this will add the changed files to git.

- `git commit -m "some text describing the commit"`, commit the changes.

- `git push`, to push the commit upstream (to the Vesuvio gitlab server).
