global_asm!(include_str!("handler.S"));

extern "C" {
    #[used]
    static mut SYSCALL_HANDLERS: [usize; 128];

    pub fn syscall_handler();
}

#[no_mangle]
extern "C" fn unimplemented_syscall_handler() {
    let syscall_num: u32;
    unsafe { asm!("move $0, $$v1" : "=r" (syscall_num)) };
    unimplemented!("Handler for syscall {:x}", syscall_num);
}

pub fn init() {
    unsafe {
        for x in SYSCALL_HANDLERS.iter_mut() {
            *x = unimplemented_syscall_handler as usize;
        }
    }
}
