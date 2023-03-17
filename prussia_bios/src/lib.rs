//! A Rust implementation of the PlayStation 2 BIOS.

#![no_std]
#![deny(missing_docs)]
#![feature(asm_experimental_arch)]

use core::arch::asm;

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
        asm!(
            "syscall",
            in("$3") 0x2, // v1
            in("$4") imode as i16, // a0
            in("$5") vmode as i16, // a1
            in("$6") ffmode as i16, // a2
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
        asm!(
            "syscall",
            in("$3") 0x4, // v1
            in("$4") return_code, // a0
            options(noreturn),
        );
    }
}
