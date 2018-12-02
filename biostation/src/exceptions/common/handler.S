.section .ee_exc_common, "a"
.ent common_handler
common_handler:
        mfc0    $k0, $13                // Load Cause register.
        andi    $k0, 0x3F               // Extract Cause.ExcCode.
        
        addi    $sp, -20                // Save $ra and $t0.
        sw      $ra, 0($sp)
        sw      $t0, 16($sp)

        la      $t0, COMMON_HANDLERS    // Load start of handler table.
        add     $k0, $t0                // Add ExcCode to handler table.

        lw      $t0, 16($sp)            // Restore $t0 from stack.

        lw      $k0, 0($k0)             // Get the handler address.
        jalr    $k0                     // Jump to the handler.

        lw      $ra, 0($sp)             // Restore $ra from stack.
        addi    $sp, 20                 // Pop stack.

        .int 0x0000000f // sync
        .int 0x42000018 // eret
.end common_handler

.section .bss
COMMON_HANDLERS:
.space 16*4
