# To disable and enable interrupts, the PlayStation 2 has two instructions called di and ei.
# However, LLVM doesn't understand these instructions, so we're putting them in separate
# assembly functions to be called by interrupt::disable and interrupt::enable.

# _di() disables interrupts, and returns the Coprocessor 0 status register.
# Note that the `di` instruction has a bug in that it might not disable interrupts 
.globl _di
.ent _di
_di:
    di                  # Disable interrupts.
    sync.p              # Ensure the interrupt disable instruction has fully processed.
    mfc0    $2, $12     # Set the coprocessor 0 status register as return value.
    jr      $ra         # And return.
    nop
.end _di

# _ei() enables interrupts.
.globl _ei
.ent _ei
_ei:
    ei                  # Enable interrupts.
    sync.p              # Ensure the interrupt enable instruction has fully processed.
    jr      $ra         # And return.
    nop
.end _ei
