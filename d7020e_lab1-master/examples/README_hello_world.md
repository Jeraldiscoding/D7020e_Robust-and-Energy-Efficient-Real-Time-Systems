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

```text
hello_world #0
21:54:46.296: hello_world #1
21:54:46.296: hello_world #2
21:54:46.296: hello_world #3
21:54:46.296: hello_world #4
21:54:46.296: hello_world #5
21:54:46.296: hello_world #6
21:54:46.296: hello_world #7
21:54:46.296: hello_world #8
21:54:46.296: hello_world #9
21:54:46.296: idle
21:54:46.296: in some_function
21:54:46.296: in lets_crash
21:54:46.296: panicked at examples/hello_world.rs:52:5:
21:54:46.296: let's crash with 42
```

### A.2) Tracing the call stack on a crash.

In the left pane you have a `CALL STACK` dropdown, it indicates that your program is still `Running`

The program deliberately panics in `lets_crash` and enters the RTT panic
handler. Pause the debugger to see which function calls led to the panic.

Press the "pause" button (or press F6).

This allows us to inspect the call stack.

You should see something like:

```text
... panic handler frames ...
lets_crash     hello_world.rs:52:5
some_function  hello_world.rs:46:5
idle           hello_world.rs:37:9
main           hello_world.rs:13:1
```

What you see here are the different stack frames. (Recall from D0013E, function calls and stack frames.)

Click the `lets_crash` frame and see what line was causing the panic.

In the left panel you have `Registers, Static` and `Variables`. The argument `i` should be listed under `Variables`.

What was the value of `i`.

42

Which frame below `lets_crash` shows where it was called from?

The caller was `some_function` at line 46, which executes
`lets_crash(42);`. The panic occurs inside `lets_crash` at line 52.

You can now click the corresponding frame, just to see that the call chain makes sense.

### A.3) Optimization

Rust is good at optimizing code, but it can be hard to debug since the structure of the code is altered in the process.

Repeat the crash inspection from A.2 using the `probe-rs Release` profile.
Stop the current session first by pressing the red square (or Shift+F5).

### A.4) Breakpoints

To wrap up, now set a breakpoint at line 28 (the `rprintln`), you click to the left of the line and a red ball should appear. Set breakpoints are also shown in the left panel.

Now run the program again using the `probe-rs Debug` profile.

What is the value reported for `i`, when the program stops at the breakpoint.

```
0
```

You can press play again (F5), what is the new value of `i`.

```
1
```

Now, terminate the debug session and try the same in `probe-rs Release`.

What is the value reported for `i`.

```
< <value optimized away by compiler, out of scope, or dropped> >
```

You can press play again (F5), what is the new value of `i`.

```
no second stop at line 28 observed
```

What do you think happened?

In Release, the debugger stopped at line 28 but couldn't show the loop
counter `i`. Compiler optimisations can make a variable unavailable to the
debugger. After F5, no second stop at line 28 was observed. The manual pause
showed `i = 42` in `lets_crash`, where `i` is the function argument.

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
