use core::{mem, arch::global_asm};

global_asm!(include_str!("glue.S"));

/// The execution state of a thread.
#[repr(C)]
pub struct ThreadContext {
    /// EE general purpose registers.
    gprs: [u128; 32],
    /// ALU high register.
    hi: u128,
    /// ALU low register.
    lo: u128,
    /// Shft amount register.
    sa: u32,
    /// EE floating point registers.
    fprs: [u32; 32],
    /// Floating point accumulator.
    fp_acc: u32,
    /// Floating point unit control register.
    fp_ctrl: u32,
}

/// An EE kernel thread.
#[derive(Copy, Clone, Debug)]
struct Thread {
    /// Thread global pointer.
    gp: usize,
    /// Thread stack size.
    stack_size: usize,
    /// The bottom of the thread's stack.
    stack_bottom: usize,
    /// The top of the thread's heap.
    heap_top: usize,
    /// The thread context address.
    context: usize,
    /// Address to return to after the thread exits.
    return_address: usize,
    /// Thread arguments.
    args: usize,
}

impl Thread {
    fn new(
        gp: usize,
        stack_size: usize,
        stack_bottom: usize,
        return_address: usize,
        args: usize,
    ) -> Self {
        Thread {
            gp,
            stack_size,
            stack_bottom,
            heap_top: stack_bottom,
            context: stack_bottom + stack_size - mem::size_of::<ThreadContext>(),
            return_address,
            args,
        }
    }

    /// Return a mutable reference to this thread's register context.
    fn context_mut(&mut self) -> &mut ThreadContext {
        unsafe { &mut *(self.context as *mut ThreadContext) }
    }

    /// Return the top of a thread's stack, excluding the area of memory for the thread context.
    fn top_of_stack(&self) -> usize {
        self.context
    }

    /// Return the top of a thread's stack, including the area of memory for the thread context.
    fn absolute_top_of_stack(&self) -> usize {
        self.stack_bottom + self.stack_size
    }

    /// Return the bottom of a thread's stack.
    fn bottom_of_stack(&self) -> usize {
        self.stack_bottom
    }

    /// Return the top of a thread's heap.
    fn top_of_heap(&self) -> usize {
        self.heap_top
    }

    /// Set the top of a thread's heap.
    fn set_top_of_heap(&mut self, top: usize) {
        self.heap_top = top;
    }
}

/// The EE kernel thread state.
struct KernelThreadState {
    threads: [Option<Thread>; 256],
    current: u32,
}

impl KernelThreadState {
    const fn new() -> Self {
        KernelThreadState {
            threads: [None; 256],
            current: 0,
        }
    }

    /// Create the main thread, resetting internal state, and returning the thread ID of the main
    /// thread.
    fn create_main(
        &mut self,
        gp: usize,
        stack_size: usize,
        stack_bottom: usize,
        return_address: usize,
        args: usize,
    ) -> u32 {
        *self = KernelThreadState::new();

        self.threads[self.current as usize] = Some(Thread::new(
            gp,
            stack_size,
            stack_bottom,
            return_address,
            args,
        ));

        self.current
    }

    /// Return a reference to the thread for a given ID.
    fn thread(&self, id: usize) -> &Option<Thread> {
        &self.threads[id]
    }

    /// Return the current thread ID.
    fn current_thread(&self) -> u32 {
        self.current
    }
}

extern "C" {
    /// ABI adapter for init_main_thread_rust
    pub fn init_main_thread(
        gp: usize,
        stack_ptr: *mut u8,
        stack_size: usize,
        args: *mut u8,
        return_address: usize,
    );
}

static mut THREAD_STATE: KernelThreadState = KernelThreadState::new();

const END_OF_RAM: usize = 0x0200_0000;

/// Initialise the threading system, returning the stack pointer for the main thread.
#[no_mangle]
pub extern "C" fn init_main_thread_rust(
    gp: usize,
    mut stack_ptr: *mut u8,
    stack_size: usize,
    args: *mut u8,
    return_address: usize,
) -> usize {
    assert!(
        stack_size < END_OF_RAM,
        "Attempted to allocate more stack than available RAM"
    );

    // A negative stack pointer means "put at the end of RAM". We give it a 4KiB buffer zone to
    // avoid stack smashing.
    if (stack_ptr as i32) < 0 {
        stack_ptr = (END_OF_RAM - (stack_size + 4096)) as *mut u8;
    }

    let id = unsafe {
        THREAD_STATE.create_main(
            gp,
            stack_size,
            stack_ptr as usize,
            return_address,
            args as usize,
        )
    };

    let mut thread = unsafe { THREAD_STATE.thread(id as usize).unwrap() };

    let top_of_stack = thread.absolute_top_of_stack();
    let ctx = thread.context_mut();

    ctx.gprs[28] = gp as u128; // Global pointer.
    ctx.gprs[29] = top_of_stack as u128; // Stack pointer.
    ctx.gprs[30] = top_of_stack as u128; // Frame pointer.
    ctx.gprs[31] = return_address as u128; // Return address.

    thread.top_of_stack()
}

/// Initialise a thread's heap, returning the top of its heap.
pub extern "C" fn init_heap(heap_bottom: usize, heap_size: i32) -> usize {
    let thread = unsafe { THREAD_STATE.current_thread() };
    let mut thread = unsafe { THREAD_STATE.thread(thread as usize).unwrap() };

    if heap_size < 0 {
        // A negative heap size means the top of the heap is the bottom of the stack.
        thread.set_top_of_heap(thread.bottom_of_stack());
    } else {
        // A positive heap size means the heap is of a fixed size.
        thread.set_top_of_heap(heap_bottom + (heap_size as usize));
    }

    thread.top_of_heap()
}
