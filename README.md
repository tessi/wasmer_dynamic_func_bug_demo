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
