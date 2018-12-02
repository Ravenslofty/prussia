use core::hint;

mod syscall;

#[no_mangle]
static COMMON_HANDLERS: [extern "C" fn(); 16] = [
    // Interrupt
    unimplemented_handler,
    // TLB Modified
    unimplemented_handler,
    // TLB Invalid (Load)
    unimplemented_handler,
    // TLB Invalid (Store)
    unimplemented_handler,
    // Address Error (Load)
    unimplemented_handler,
    // Address Error (Store)
    unimplemented_handler,
    // Bus Error (instruction fetch)
    unimplemented_handler,
    // Bus Error (data load/store)
    unimplemented_handler,
    // System Call
    unimplemented_handler,
    // Breakpoint
    unimplemented_handler,
    // Reserved Instruction
    unimplemented_handler,
    // Coprocessor Unusable
    unimplemented_handler,
    // Arithmetic Overflow
    unimplemented_handler,
    // Trap
    unimplemented_handler,
    // (not used)
    unimplemented_handler,
    // (not used)
    unimplemented_handler,
];

extern "C" fn unimplemented_handler() {
    unimplemented!("common exception handler");
}

#[naked]
#[no_mangle]
#[link_section = ".ee_exc_common"]
unsafe extern "C" fn handler() -> ! {
    asm!(
        "
        // Load the COP0 cause register and extract the exception code.
        mfc0 $$k0, $$13
        andi $$k0, 0x3F
        
        // Save ra and t0 to stack.
        addi $$sp, -32
        sw   $$ra, 0($$sp)
        sw   $$t0, 16($$sp)

        // Load the base exception handler table and add the exception code.
        la   $$t0, COMMON_HANDLERS
        add  $$k0, $$t0

        // Restore t0
        lw   $$t0, 16($$sp)

        // Get the exception handler address and jump to the handler.
        lw   $$k0, 0($$k0)
        jalr $$k0

        // Restore the return address and pop stack.
        lw   $$ra, 0($$sp)
        addi $$sp, 32

        // Flush the pipeline and return to user code.
        .int 0x0000000f // sync
        .int 0x42000018 // eret
    "
    );
    hint::unreachable_unchecked();
}
