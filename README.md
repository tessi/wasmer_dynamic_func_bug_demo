# wasmer_dynamic_func_bug_demo
A demo program showcasing a bug I have using DynamicFunc

Running

```
RUST_BACKTRACE=full cargo run
```

results in errors on wasmer.

Tested with:

* rustc 1.43.0 (4fb7144ed 2020-04-20)
* wasmer from git's `master` branch. Commit `77de93ec0e0521d06477b00389321ef8e9ea6ebf`

The error is different each time, but always one of the following:

```
❯ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/wasmer_demo`
Hello from the WASI test program!

Arguments:

Environment:

Current Time (Since Unix Epoch):
1588966859

wasmer_demo(3275,0x10fee1dc0) malloc: can't allocate region
:*** mach_vm_map(size=3377586554535936, flags: 60000100) failed (error code=3)
wasmer_demo(3275,0x10fee1dc0) malloc: *** set a breakpoint in malloc_error_break to debug
memory allocation of 3377586554535936 bytes failed[1]    3275 abort      RUST_BACKTRACE=full cargo run
```

or

```
❯ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/wasmer_demo`
Hello from the WASI test program!

Arguments:

Environment:

Current Time (Since Unix Epoch):
1588966878

thread 'main' panicked at 'capacity overflow', src/liballoc/raw_vec.rs:750:5
stack backtrace:
   0:        0x105f3d06f - std::sys_common::at_exit_imp::push::h9d2b273ed4ea5358
   1:        0x105f5b48e - <&mut W as core::fmt::Write>::write_fmt::h25d58e21bc6e71c9
   2:        0x105f3a717 - std::io::Write::write_fmt::hf0a8cad0ca2251e3
   3:        0x105f3f07a - std::panicking::default_hook::{{closure}}::h166f557a03fd111a
   4:        0x105f3edbc - std::panicking::default_hook::h16057d891db6b023
   5:        0x105f3f6e8 - <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get::hb0321a5224bfb474
   6:        0x105f3f2b2 - std::panicking::try::do_call::h7c88c220bfff6b21
   7:        0x105f6bd6f - std::panicking::begin_panic::h1eb3d94191b6aea5
   8:        0x105f6bcc7 - std::panicking::begin_panic::h1eb3d94191b6aea5
   9:        0x105f54f8c - alloc::raw_vec::RawVec<T,A>::reserve_internal::h03d9a4879d77c9ff
  10:        0x105dd9d87 - alloc::raw_vec::alloc_guard::ha062a71d21d85229
  11:        0x105e4fcba - alloc::vec::SetLenOnDrop::new::h3f87c66392359504
  12:        0x105e52b80 - alloc::vec::SetLenOnDrop::new::h3f87c66392359504
  13:        0x105e543ac - alloc::vec::SetLenOnDrop::new::h3f87c66392359504
  14:        0x105e560df - alloc::vec::SetLenOnDrop::new::h3f87c66392359504
  15:        0x105e33847 - std::io::impls::<impl std::io::Read for &[u8]>::read_exact::h5e50e7b5129762aa
  16:        0x105e00b41 - wasmer_runtime_core::typed_func::DynamicFunc::new::do_enter_host_polymorphic::h9d3da410403150e4
  17:        0x105e010a2 - wasmer_runtime_core::typed_func::DynamicFunc::new::do_enter_host_polymorphic::{{closure}}::h73ae26eb50750d45
fatal runtime error: failed to initiate panic, error 5
[1]    3412 abort      RUST_BACKTRACE=full cargo run
```

or

```
❯ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/wasmer_demo`
Hello from the WASI test program!

Arguments:

Environment:

Current Time (Since Unix Epoch):
1588966927

[/Users/tessi/.cargo/git/checkouts/wasmer-f11f30e62739aa29/77de93e/lib/runtime-core/src/instance.rs:609] &error = InvokeError(
    UnknownTrap {
        address: 0,
        signal: "segmentation violation",
    },
)
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: RuntimeError: InvokeError(UnknownTrap { address: 0, signal: "segmentation violation" })', src/main.rs:27:5
stack backtrace:
   0:        0x1062d606f - std::sys_common::at_exit_imp::push::h9d2b273ed4ea5358
   1:        0x1062f448e - <&mut W as core::fmt::Write>::write_fmt::h25d58e21bc6e71c9
   2:        0x1062d3717 - std::io::Write::write_fmt::hf0a8cad0ca2251e3
   3:        0x1062d807a - std::panicking::default_hook::{{closure}}::h166f557a03fd111a
   4:        0x1062d7dbc - std::panicking::default_hook::h16057d891db6b023
   5:        0x1062d86e8 - <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get::hb0321a5224bfb474
   6:        0x1062d82b2 - std::panicking::try::do_call::h7c88c220bfff6b21
   7:        0x106304d6f - std::panicking::begin_panic::h1eb3d94191b6aea5
   8:        0x106304c75 - std::panicking::begin_panic::h1eb3d94191b6aea5
   9:        0x105c1df60 - core::result::Result<T,E>::unwrap::hc3f82c8fcbaa85e6
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550/src/libcore/result.rs:1003
  10:        0x105c1a5a8 - wasmer_demo::main::h15487e35983c1498
                               at src/main.rs:27
  11:        0x105c1c9de - std::rt::lang_start::{{closure}}::hca85ff321b4abf32
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550/src/libstd/rt.rs:67
  12:        0x1062d81b8 - std::panicking::try::do_call::h7c88c220bfff6b21
  13:        0x1062db64b - panic_unwind::dwarf::eh::read_encoded_pointer::h37bb3f695deaeae6
  14:        0x1062d8bba - std::panicking::rust_panic_without_hook::h6ca5ec7e0059bf4e
  15:        0x105c1c9c1 - std::rt::lang_start::h23aabe723db67e46
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550/src/libstd/rt.rs:67
  16:        0x105c1a702 - wasmer_demo::main::h15487e35983c1498
```
