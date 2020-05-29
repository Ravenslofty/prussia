//! A Rust implementation of the PlayStation 2 BIOS.

#![no_std]
#![deny(missing_docs)]
#![feature(llvm_asm)]

/// The Graphics Synthesizer video mode, passed to `set_gs_crt`.
#[repr(i16)]
pub enum VideoMode {
    /// PAL, 720x576@50Hz interlaced, as used in most of Europe.
    Pal = 2,
    /// NTSC, 720x480@59.97Hz interlaced, as used in North America and Japan.
    Ntsc = 3,
}

/// Whether interlacing is enabled, passed to `set_gs_crt`.
#[repr(i16)]
pub enum Interlacing {
    /// Do not interlace frames.
    Noninterlaced = 0,
    /// Interlace frames, blending between them.
    Interlaced = 1,
}

/// Whether to read every line or every other line, passed to `set_gs_crt`.
#[repr(i16)]
pub enum FieldFrameMode {
    /// Read every line from a frame.
    Frame = 0,
    /// Read every other line from a frame.
    Field = 1,
}

/// Configure the Graphics Synthesizer's display controller to output a given VideoMode.
pub fn set_gs_crt(imode: Interlacing, vmode: VideoMode, ffmode: FieldFrameMode) {
    unsafe {
        llvm_asm!("li $$v1, 0x02; move $$a0, $0; move $$a1, $1; move $$a2, $2; syscall"
             :
             : "r" (imode), "r" (vmode), "r" (ffmode)
             : "$$v1", "$$a0", "$$a1", "$$a2"
             : "volatile"
        );
    }
}

/// Exit the program and return to the PS2 browser.
///
/// This function is not recommended for use if developing with PS2Link; it won't return to PS2Link
/// but instead go straight to the browser, meaning you'll need to relaunch PS2Link. Instead, use
/// `sleep_thread`.
pub fn exit(return_code: u32) -> ! {
    unsafe {
        llvm_asm!("li $$v1, 0x04; move $$a0, $0; syscall"
             :
             : "r" (return_code)
             : "$$v1", "$$a0"
             : "volatile"
        );
    }
    unreachable!();
}
