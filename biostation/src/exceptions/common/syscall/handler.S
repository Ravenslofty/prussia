.extern init_main_thread

.text
.globl syscall_handler
.ent syscall_handler
syscall_handler:
        addi    $sp, -16                // Save $ra to stack.
        sw      $ra, 0($sp)

        la      $k0, SYSCALL_HANDLERS   // Load the start of the syscall table.
        andi    $v1, 0x7F               // Mask out the "sign" bit of the syscall.
        sll     $v1, 2                  // Convert syscall number to index.
        addu    $k0, $3                 // Calculate the syscall table offset.
        lw      $k0, 0($k0)             // Find the handler address.

        jalr    $k0                     // Jump to it.

        lw      $ra, 0($sp)             // Restore $ra from stack.
        addi    $sp, 16

        mfc0    $k0, $14                // Increment EPC to run the next instruction.
        addi    $k0, 4
        mtc0    $k0, $14

	jr      $ra
.end syscall_handler

.section .bss
SYSCALL_HANDLERS:
    .space 128*4
