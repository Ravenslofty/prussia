//! Functions for handling panics.
use core::fmt::Write;

use prussia_debug::EEOut;

/// Main entrpoint for the [panic_handler](https://doc.rust-lang.org/nomicon/panic-handler.html).
pub(crate) fn panic_entrypoint(info: &core::panic::PanicInfo) -> ! {
    writeln!(EEOut, "Uncaught panic. Info: \n\t{info}");

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
