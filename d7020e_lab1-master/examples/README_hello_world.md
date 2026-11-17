# README for hello_world.rs

Simple example just to get you going with Rust on embedded devices.

In this case you will use the `probe-rs` debugger in `vscode` to run your program. (See `README.md` regarding plugin extensions.)

Open the `examples/hello_world.rs` file, it needs to be the active window in `vscode`, then press `CTRL-D` this should open up a panel to the left.

In the dropdown, select `probe-rs Debug`, and press the play button (or press F5).

If all goes well it will compile your example and run it, and you will have the RTT trace in the `vscode` terminal.

---

## Exercises:

### A.1) Tracing your application.

Your program should have panicked. Paste the output from the terminal window.

[ Your answer here ]

### A.2) Tracing the call stack on a crash.

In the left pane you have a `CALL STACK` dropdown, it indicates that your program is still `Running`

At this point we have reached the `rtt_panic`. In this case, it was a deliberate `panic`, (we called `lets_crash` from `idle`), but in the typical case the `panic` handler is called when the running program attempts to violate the soundness rules of Rust. Instead of continuing with potentially harmful `undefined behavior` the program ends up in the `panic` handler and nothing bad will happen (the program is `safe`).

While it is `safe` to stop, it is rarely what we want the program to do, so let's do some forensic post-mortem analysis.

Press the "pause" button (or press F6).

This allows us to inspect the call stack.

You should see something like:

```text
panic       lib.rs:68:9

lets_crash  hello_world.rs:45:5
idle        hello_world.rs:38:9
main        hello_world.rs:13:1
```

What you see here are the different stack frames. (Recall from D0013E, function calls and stack frames.)

Click the `lets_crash` frame and see what line was causing the panic.

In the left panel you have `Registers, Static` and `Variables`. The argument `i` should be listed under `Variables`.

What was the value of `i`.

[ Your answer here ]

Now let's dig deeper from where was `lets_crashed` called?

You should find this on the call stack, below `lets_crash`.

Where did the call come from?

[ Your answer here ]

You can now click the corresponding frame, just to see that the call chain makes sense.

### A.3) Optimization

Rust is good at optimizing code, but it can be hard to debug since the structure of the code is altered in the process.

Now repeat the same process as in A.3 but choosing the `probe-rs Release` profile instead. You need to terminate the current debug session by pressing the red square (or press SHIFT-F5).


### A.4) Breakpoints

To wrap up, now set a breakpoint at line 28 (the `rprintln`), you click to the left of the line and a red ball should appear. Set breakpoints are also shown in the left panel.

Now run the program again using the `probe-rs Debug` profile.

What is the value reported for `i`, when the program stops at the breakpoint.

[ Your answer here ]

You can press play again (F5), what is the new value of `i`.

[ Your answer here ]

Now, terminate the debug session and try the same in `probe-rs Release`.

What is the value reported for `i`.

[ Your answer here ]

You can press play again (F5), what is the new value of `i`.

[ Your answer here ]

What do you think happened?

[ Your answer here ]

... your guess is as good as mine, ... its hard also for the debugger to figure out ...

Later in the lab we will have a go at inspecting the generated binary (but for an even simpler example).

---

Intended learning outcomes:

Debugging is essential to embedded development, using `vscode` you can:

- View `rtt` traces in the `vscode` terminal
- Do crash/panic forensics inspecting the stack.
- Breakpoints are great to stop and inspect intermittent program states.
- `rtt` based tracing is great for observing effects of a running program.
- Debugging of optimized code may be difficult, its hard for the debugger to extract information as the compiler restructures the program.
