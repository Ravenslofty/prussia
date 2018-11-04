# While writing this, I used the PS2SDK as reference, which is:
# Copyright (c) 2001-2007 ps2dev - http:#www.ps2dev.org
# Licenced under Academic Free License version 2.0
# Review ps2sdk README & LICENSE files for further details.

.text
# Used for setting up the global pointer register.
# Note that this is a pointer, rather than a value specifically.
.extern START_OF_BSS

# Used for marking the rest of RAM between the executable and the stack as heap space.
.extern END_OF_BSS

# Used to initialise the stack pointer register.
# This is a sentinel value which the EE kernel interprets as "at the end of RAM minus STACK_SIZE"
.set END_OF_RAM, -1

# The amount of stack we want to allocate - 128KB.
.set STACK_SIZE, 128 * 1024

# Used to initialise the heap.
# This is another sentinel value meaning "until the beginning of stack"
.set HEAP_SIZE, -1

# We want argument memory to be visible to the Rust runtime.
.globl ARG_SPACE

# And we want the end of heap to be visible too.
.globl END_OF_HEAP

# We want the Rust runtime to be visible to us too, though.
.extern _rust_start

# System call numbers used in initialisation
.set InitMainThread, 60
.set InitHeap, 61

# The beginning of the program. Our job here is to set up the stack (and we get the heap set up as
# a bonus) and then we call into _rust_start to initialise things from Rust.
.ent _start
.globl _start
_start:

# We need to set up the stack, and conveniently the EE kernel provides a routine called InitMainThread
# which we will call instead of doing it manually. It is a little poorly documented, though.
# It has a signature that (in C terms) looks like this:
# void* InitMainThread(void* start_of_globals, void* start_of_stack, int stack_size, void* start_of_args, void* root_func)
# And it returns the stack pointer for us to use.
    la      $gp, START_OF_BSS   # We want the global pointer register to reference our globals, wherever they may be.
    li      $3, InitMainThread  # Call InitMainThread, obviously.
    move    $4, $gp             # First argument: the global pointer register, which we just initialised.
    la      $5, END_OF_RAM      # Second argument: the stack pointer should be initialised at the end of ram. Note that it grows downwards.
    li      $6, STACK_SIZE      # Third argument: how much stack space to allocate.
    la      $7, ARG_SPACE       # Fourth argument: we want program arguments to be put in ARG_SPACE.
    la      $8, _InfiniteLoop   # Fifth argument: I have no idea what this does.
    syscall
    move    $sp, $2             # The kernel gives us a new stack pointer in return.

# While we're here, we should also tell the EE kernel to mark the rest of RAM as heap space. For
# that, the EE kernel has a routine called InitHeap, which looks like this:
# void* InitHeap(void* start_of_heap, int heap_size)
# And returns the end of the heap.
    li      $3, InitHeap        # Call InitHeap
    li      $4, END_OF_BSS      # First argument: we want heap to start at the end of static data.
    li      $5, HEAP_SIZE       # Second argument: we want heap to continue until the beginning of stack.
    syscall
    la      $8, END_OF_HEAP     # Make the end of heap pointer visible to Rust.
    sw      $2, 0($8)           # There is also a kernel function to return this, but I don't think it's worth using.

# With that out the way, it's showtime!
    la      $3, _rust_start     # Load the ps2-rt entry point into a register...
    jr      $3                  # And jump to it.
    nop
.end _start

# It's an...infinite loop.
# The padding NOPs are to work around a hardware bug.
.ent _InfiniteLoop
.globl _InfiniteLoop
_InfiniteLoop:
    nop
    nop
    nop
    nop
    nop
    j       _InfiniteLoop
    nop
.end _InfiniteLoop

.data
# Program arguments are stored here for Rust to parse and pass to the program.
# 4-byte argc plus 16 argv pointers to parts of a 256-byte buffer.
ARG_SPACE:
.space  4 + (16 * 4) + 256

# We need somewhere to put the end of heap value.
END_OF_HEAP:
.int 0

