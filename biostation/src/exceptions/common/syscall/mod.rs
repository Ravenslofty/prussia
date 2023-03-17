use core::arch::{asm, global_asm};

use crate::cache;
use crate::gs;
use crate::thread;

global_asm!(include_str!("handler.S"));

extern "C" {
    static mut SYSCALL_HANDLERS: [usize; 128];

    /// Assembly dispatch function for system calls.
    pub fn syscall_handler();
}

#[no_mangle]
extern "C" fn unimplemented_syscall_handler() {
    let syscall_num: u32;
    unsafe { asm!("move {}, $v1", out(reg) syscall_num) };
    unimplemented!("Handler for syscall {:x}", syscall_num);
}

pub fn init() {
    unsafe {
        for x in SYSCALL_HANDLERS.iter_mut() {
            *x = unimplemented_syscall_handler as usize;
        }

        SYSCALL_HANDLERS[0x3C] = thread::init_main_thread as usize;
        SYSCALL_HANDLERS[0x3D] = thread::init_heap as usize;
        SYSCALL_HANDLERS[0x64] = cache::flush as usize;
        SYSCALL_HANDLERS[0x70] = gs::imr::imr as usize;
        SYSCALL_HANDLERS[0x71] = gs::imr::put_imr as usize;
    }
}
