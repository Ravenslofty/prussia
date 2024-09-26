//! Functions for handling panics.
use prussia_debug::println_ee;

use crate::cop0::CoP0Dump;

/// Main entrpoint for the [panic_handler](https://doc.rust-lang.org/nomicon/panic-handler.html).
pub(crate) fn panic_entrypoint(info: &core::panic::PanicInfo) -> ! {
    println_ee!("Uncaught panic. Info: \n\t{info}");

    let cop0_dump = CoP0Dump::load();

    println_ee!("CoP0 registers: {cop0_dump:#?}");

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
