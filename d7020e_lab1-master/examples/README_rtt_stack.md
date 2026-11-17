# Readme for rtt_stack.rs

Stack memory handling is outside of scope of the Rust language, thus correctness is ultimately up to the developer. 

Task in the RTIC executable model translates to Rust functions, accessing data through statically allocated Resources (referring either to the heap, or to some memory mapped peripheral). Task may also allocate values on the current stack (local to the running context).

In effect these run-time stack allocations causes the stack to grow (and shrink once the function returns). Thus, the stack may eventually "overflow", overwriting heap-allocated data, or accessing memory of bounds for the region. 

The latter is typically caught by the hardware which throws an exception. In RTIC we can associate a task to the hard fault. At that point we don't have any stack space left, thus we cannot do much else than "die" (e.g., reset:ing the target). At any rate, "memory safety" can be maintained. 

The former is more problematic. By default the stack is located at the end of the RAM memory region while the heap starts from the beginning. As the overflowing stack is referring to a valid address in the RAM memory region, the hardware will not protect us, and thus the application can run into Undefined Behavior (UB) without any chance for us (developers) to know or notice. 

While this is a potentially disastrous corner case of RTIC, each pointer dereferencing in C/C++ is up to you as a developer to prove sound/correct.

If we can fix this problem for Rust, we take a leap-step towards complete memory safety. The trivial solution is to flip the stack and heap, thus allowing the hardware to spot stack overflow. Emil Fresk (long time co-developer of RTIC), is currently pushing for changing the default behavior to the "flipped" order. The reason for heap, stack default order is simplicity and legacy (C/C++) which traditionally use this order. 

However, using Rust/RTIC all allocations are Sized, thus we can determine (at compile time) the required heap-size. We need linking to be carried out in two steps, first determining the heap-size, then assigning start/end addresses such that heap is allocated at the end of the RAM region, and the stack start position is set to the bottom of the heap. 

The problem with this trivial solution is that robustness and reliability is traded for memory safety (we can still run into a stack overflow at run-time, which instead of silently running into UB now will be caught by the hardware leading to an un-recoverable a hard fault).

What we really want is a compile time guarantee that the system will never run out of stack memory. (With that at hand we would not even need flipping heap stack order.) In this lab you will learn best practices in the field. Later we will look into state-of-the art and beyond.

## Learning outcomes

- Basic understanding of the "stack-overflow" problem, and its impact to UB as discussed above.
- Understanding of stack-allocation code generation and optimizations done by the Rust toolchain.
- Creating a Rust library with useful functions to inspect stack. 
  - inline assembly macro to read the stack-pointer value.
  - inline assembly macro to generate custom breakpoint instructions.
- Stack-painting and its use to snap-shotting stack accesses.

## Exercises

### A) Tracing the stack using the debugger.

Run the program in release mode in vscode.

It should now hit the breakpoint at line 35.

Expand the `Registers` view and locate the R13/SP.

What is the current SP value?

[ Paste the SP value here ]

Explain in you own words why this is reasonable. Hint, relating to the content of the `memory.x` file, and the discussion in the first section of this file.

[ Your answer here ]

Now continue to the next breakpoint. Should be in the `outer_fn`.

What is now the current SP value?

[ Paste the SP value here ]

Explain in your own words why this value is reasonable. Hint, think in terms of local variables. Notice, the complete frame is constructed already when we hit the first line in the function body.

[ Your answer here ]

Now continue to next breakpoint, should be in the `inner_fn`. 

What is the current value of the SP?

[ Paste the SP value here ]

Argue in your own words why this is reasonable. Hint, look at the size of the local array.

[ Your answer here ]

Rust macros allows you to extend the syntax of the Rust language, in this case by an inline assembly "bkpt #<imm>" instruction. During compilation the macro is expanded, and the argument `1` in `bkpt_nr(1)` is injected *into* the assembly instruction (we are actually generating code "in place" at the call site).

Now, hover the `bkpt_nr!` and press F12 (Go To Definition). (You should end up with the definition of `bkpt_nr` in the `src/lib.rs` file.) 

[ Paste the macro here ]

Now, lookup the Rust documentation [Inline Assembly](https://doc.rust-lang.org/reference/inline-assembly.html), and the cortex-m assembly documentation [Table of processor instructions](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en).

The `options` flags inform the compiler of the side effects the specific operation(s) has to the state of the processor. This information (contract) is later used as assumptions by the compiler backend, in order to optimize code generation (and is thus *crucial* for both performance and correctness of the generated code.)

Explain in your own words, the reasoning behind `options(nomem, nostack, preserves_flags)`, for this particular operation/instruction.

[ Your explanation here ]

Now comment out line 59.

What error do you get and why do you think this should be an error?

[ Your answer here ]

Let us now comment out line 4 (`#![deny(warnings)]`).

If you hover `stack_alloc` (line 58), what is the warning reported?

[ Your answer here ]

Follow the hint/suggestion to get rid of the warning.

Now, re-run the experiment:

What is the value of SP when we hit line 57. 

[ Paste the value of SP here ]

Now, compare this value to the previous experiment? Explain in your own words, what you think the compiler has done.

[ Your answer here ]

!!! IMPORTANT !!!
Now, revert the changes made, (uncomment lines 4 and 59, and restore the identifier to `stack_alloc`).

### B) Inspecting stack by code.

In the previous example we instrumented the code with breakpoints, and manually looked at the stack growing towards lower addresses. 

For a realistic program this method is unfeasible. The number of functions to instrument quickly becomes unwieldily large. Moreover as you may call into functions defined in 3rd party libraries, you may not be able to make such changes.

A common method to deal with this problem is by "stack-painting". The idea is simple, at boot, you "paint" the stack region (end of heap until end of ram), with a known pattern. Then you take a "snapshot" of the stack by reading the memory to find the lowest address tainted (written to by the program up-until this point). 

The `cortex-m-rt` provides a Rust run-time system, which runs before the user application is started. It initializes heap variables (`statics` in Rust). Enabling the "paint-stack" feature (line 24 in `Cargo.toml`), will paint the stack with the constant `0xcccc_cccc`, before the user program is launched. 

Now, comment out the breakpoints, lines 35, 48, and 57, and re-run the experiment.

The program should now run to end (end up looping in `idle`).

Let's look at the output:

[ Paste the terminal output here ]

The SP values reported should look familiar and be very close to the ones you obtained by the manual instrumentation. (If not, go back and check that changes made earlier were reverted properly.)

Now let's look at the `idle`, `outer_fn` and `inner_fn`. They all use the `read_sp!()` (defined in `src/lib.rs`) to obtain the current value of the stack pointer.

[ Paste the `read_sp` macro definition here ]

Explain in your own words how this works. Hint, relate to documents referred to above. 

In `idle` we call `find_lowest`, explain in your own words what this function does?

[ Your answer here ]

### C) Discussion.

`find_lowest` takes a "snap-shot" of the stack memory region to determine the lowest address written to. Although being state-of-practice the approach is extremely fragile.

It gives only a "hint" on the behavior of the system, unless you can ensure that all possible paths have been executed. This is very hard, as functions might have side effects, thus all possible orders of functions and all possible starting states must be covered. Even that is not enough in case interrupts are present (which is the underlying mechanism for I/O interaction for all embedded platforms). Regarding interrupts, one would need to exhaust all possible preemptions and inter-leavings.

Commonplace kernels and schedulers attempts to address the latter problem by separating user stack from the kernel/scheduler stack, allowing each domain to be separately analyzed where all interrupts are served by the kernel. 

While this approach has such wide spread that ARM even provides hardware mechanisms to facilitate stack switching (between user and kernel), the underlying problem still remains. This is a band-aid solution, adding complexity with little or no use (as firm guarantees cannot be obtained).

Even worse, this does *NOT* prevent "stack-overflow" from occurring. In effect in case that "stack-overflow" would occur the system will run into undefined behavior, so all bets are off. In effect, the snapshots obtained might be compromised and should not be trusted (unless some means of HW protected region isolation are applied, e.g., stack/heap flipping).

The dual stack pointers of Cortex-m processors are exposed through MSP and PSP which an be accessed through the debugger, let's have a closer look.

Now, re-run the experiment. Press pause and look the set of registers. You will find three stack pointers, SP, MSP, and PSP.

[ Paste their values here ]

Two of them should hold the same value, while the third should be zero.

Lookup their documentation and summarize in your own words their intended use:

[ Your answer here ]

The reason is that RTIC does *NOT* rely on the underlying hardware implementing stack switching. Later in the course you will study opportunities to address the underlying problem in order to establish proven compile-time guarantees.

### D) External monitoring.

The efficiency of the `find_lowest` can be further improved on, e.g., by binary search. It can also be simplified by looking only at a guard region to ensure that the separation between stack and heap has not been violated. A shortcoming of the presented solution is the risk that the function would terminate with a faulty value, this would happen if the "paint" value is written to the stack. In that light, the `find_lowest` should be seen only as demonstrative to the purpose. 

If used as an online tool, it would consistently run in the background tracing the current "lowest" stack pointer value. This has the drawback that it consumes CPU. Another drawback is that if we would want to externally monitor the growth, the values would need to be somehow communicated back to some host.

An alternative solution is to use `probe-rs` as library, and let the host "poll" memory content from the target. 

The [d7020e Stack Tool](https://vesuvio-git.neteq.ltu.se/d7020e/d7020e_stack_tool), provides an example how we can poll the target.

Clone (or fork/clone) the repo and:

- 0) un-comment line 38.
- 1) Run the example on the target:
```shell
cargo run --example rtt_stack --release
```
[ Paste the output here ]

It should report a breakpoint (Error: CPU halted unexpectedly.
) at line 38. 

At this point the debug session is terminated.

- 2) Run the `stack` tool from the `d7020e_stack_tool` folder.

```shell
cargo run > stack_dump.txt
```

Inspect the output, and fill out the below table

| address    | value      |
| ---------- | ---------- |
| 0x2003febc | 0xcccccccc |
| 0x2003fec0 |            |
| 0x2003fec4 |            |
| 0x2003fec8 |            |

Now, compare with the output value `end_sp`, you should see a 1-1 to relation.

Take a look at the host side code `src/main.rs`, and describe in your own words how it works. 

[ Your answer here ]

### E) Some notes on stack frames.

The [ARM Architecture Procedure Call Standard (AAPCS)](https://developer.arm.com/documentation/107656/0101/Getting-started-with-Armv8-M-based-systems/Procedure-Call-Standard-for-Arm-Architecture--AAPCS-), states:

```txt
If an assembly function needs to call a C function, it should ensure that the current selected stack pointer points to a doubleword-aligned address location. 
```

In order for Rust to be compatible to C/assembly standard, a double word alignment of SP is enforced. In addition, Rust adds further alignment to data structures [Type Layout](https://doc.rust-lang.org/reference/type-layout.html).

This means, that values may be sparsely located on the stack. To my best understanding Rust does not stipulate that padding bytes are initialized.

With the "stack-paint" approach in mind, explain in your own words how that might affect a "binary-search"-based algorithm? 

[ Your answer here ]


### F) Inlining

During optimization the LLVM-backend may recursively inline inner functions. In the source code you may hint the backend by `#[inline(always)]` and `#[inline(never)]` (hints are typically followed but not guaranteed). For the crate compilation high-level optimization directives (such as `opt-level`) can be given in the `Cargo.toml` file.

Now, uncomment line 54, and comment out line 55. 

Run the example.

[ Paste output here ]

Compare the output to your results from exercise B, and explain in your own words how and why the stack allocation has changed. 

[ Your answer here ]

### G) Optional for higher grades

Later in the course, you can choose a subject to dig further into for higher grades. One possible topic is to refine the `d7020e_stack_tool` to extract memory region information from `elf` binaries. Further improvements can be to integrate as the tool as a cargo sub-command. This way the binary could be built, flashed, executed and monitored by a single tool.






























