# `prussia_rt` - startup and PS2-specific routines

This crate contains startup routines to initialise the PS2 ready to hand over to your code.

## `main`
`prussia_rt` expects your program's entry point to be like this:

```rust
#[no_mangle]
fn main() -> ! {
    // Your code here.
}
```

I may later add a `cortex_m_rt`-style `entry!` macro or a `#[entry]` annotation.

## Interrupts
`prussia_rt` provides `cortex_m`-style `interrupt::enable()`, `interrupt::disable` and 
`interrupt::free` functions to change the current interrupt status or call a closure in an
interrupt-free context.`

## TODO:
- Register access.
  - Coprocessor 0 register
  - Coprocessor 1 (FPU) control registers
  - Coprocessor 2 (VU0) control registers
- EE kernel syscalls.
  - Threading, based on those syscalls?
