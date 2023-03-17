use core::arch::global_asm;

use prussia_rt::cop0;

mod syscall;

global_asm!(include_str!("handler.S"));

extern "C" {
    static mut COMMON_HANDLERS: [usize; 16];
}

#[no_mangle]
extern "C" fn unimplemented_common_handler() {
    let exc_code = (cop0::Cause::load() & cop0::Cause::EXC_CODE).bits() >> 2;
    let exc_code = cop0::L1Exception::try_from_common(exc_code as u8)
        .expect("Exception codes are between 1 and 13 on real hardware");

    unimplemented!("Exception {:?} handler", exc_code);
}

pub fn init() {
    unsafe {
        for x in COMMON_HANDLERS.iter_mut() {
            *x = unimplemented_common_handler as usize;
        }

        COMMON_HANDLERS[9] = syscall::syscall_handler as usize;
    }
    syscall::init();
}
