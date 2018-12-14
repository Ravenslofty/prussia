use prussia_rt::atomic;

/// The Graphics Synthesizer Interrupt Mask Register.
const GS_IMR: usize = 0x1200_1010;

/// A local version of the IMR. Only valid if accessed exclusively through this API.
static mut IMR: u64 = 0x0000_7F00;

#[no_mangle]
pub extern "C" fn gs_get_imr() -> u64 {
    unsafe { IMR }
}

#[no_mangle]
pub extern "C" fn gs_put_imr(imr: u64) {
    atomic::write_u64(GS_IMR, imr);

    unsafe {
        IMR = imr;
    }
}
