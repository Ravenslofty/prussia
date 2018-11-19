# PruSSia - a Rust PS2 SDK.

I can be found in the Freenode IRC channel #psugnd, or in the EmuDev discord (https://discord.gg/dkmJAes).

## Lift Pitch - Why use Rust on the PS2?

- The PlayStation 2 is fairly cheap (~£40) at current prices, making it easy to obtain. It has many features: a capable GPU, embedded SIMD units, sound, controller, network and memory card support.
- Rust is a modern language with a large ecosystem, and its emphasis on safety avoids a whole class of bugs. It is also fast, being competitive with C and C++.
- Rust can be used to express side-effects of the PS2's hardware and avoid things like DMA reading unallocated memory.

## Okay, how do I get started?

### Prerequisites: PS2

You will need a memory card that has Free MCBoot to load unsigned code (memory cards with Free MCBoot installed can be found cheaply on eBay), and a *high-loading* (`LOADHIGH = 1` in the Makefile) version of [PS2Link](https://github.com/ps2dev/ps2link) on the PS2 memory card.

On your computer, you will need Linux (On Windows, a Linux VM may work, but WSL currently does not; macOS is untested), [PS2Client](https://github.com/ps2dev/ps2client), [binutils for MIPS](https://ftp.gnu.org/gnu/binutils/) and [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild). 

### Prerequisites: PC

You will need a *legal* PS2 BIOS dump, plus [PCSX2](https://pcsx2.net), [binutils for MIPS](https://ftp.gnu.org/gnu/binutils/) and [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild).

The PCSX2 first-time wizard will let you select your BIOS.

### Creating a project

As PruSSia is not currently on crates.io, create a new project with `cargo new --bin project_name` in the PruSSia root directory. Then copy `ps2.json` (which can be found in the `hello-rs` project) into your new project's root directory.

### An example project

PruSSia does not (yet) support the Rust Standard Library, so you will be working in `#![no_std]` land.

Here is a bare-bones example `src/main.rs`:
```rust
#![no_std]
#![no_main]

extern crate panic_halt;
extern crate prussia_rt;

#[no_mangle]
fn main() -> ! {
    loop {}
}
```
Plus an example `Cargo.toml` to go with it:
```toml
[package]
name = "project_name"
version = "0.1.0"
authors = ["John Smith <john.smith@example.com>", "Jane Bloggs <jane.bloggs@example.com>"]
edition = "2018"

[dependencies]
panic-halt = "0.2.0"
prussia_rt = { path = "../prussia_rt" }

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[package.metadata.cargo-xbuild]
memcpy = true
sysroot_path = "target/sysroot"
```

### Building your project

To build your project, run `cargo xbuild --target ps2.json` from the root directory of your crate. `cargo-xbuild` will then build and configure the Rust core library to be used in your crate.

### Running your project: PS2: PS2Link

You will need to launch PS2Link through either a Free MCBoot entry/hotkey (both of which can be set through the Free McBoot Configurator), or through uLaunchELF. It will block until it can configure the network, so if it does not show "Ready" after about 10 seconds, make sure your Ethernet cable is plugged in (if you have an SCPH-3xxxx console which does not come with an Ethernet port, you can get an Ethernet adapter which plugs into the back).

You will also need to find the name of your executable, which should be in `target/ps2/debug/`.

When PS2Link shows "Ready" on the screen, `ps2client execee host:path/to/executable`, and PS2Link will download and run your executable.

At the moment you will need to reset your console to run a new program, but when yielding support is implemented in PruSSia, then you could use `ps2client reset` to return to PS2Link.

### Running your project: PS2: uLaunchELF

You can also put your executable on a FAT32-formatted USB stick, and run it through the menus of uLaunchELF, although there is no possibility of remotely resetting your program after you have finished; you will need to use the power button to reset the console.

### Running your project: PCSX2

PCSX2 offers a "Run ELF" option under the System menu, that will let you select your executable to run. 

## Features

- Startup/bootstrapping crate (`prussia_rt`)
- Direct Memory Access Controller crate (`prussia_dma` - WIP)

## TODO (in rough order)

- GS Interface (GIF) to Graphics Synthesizer - requires `prussia_dma`
- SBUS Interface (SIF) to Input/Output Processor - requires `prussia_dma`
  - Console I/O - requires SIF
  - Controller I/O - requires SIF
  - Sound Output - requires SIF
  - Memory Card I/O - requires SIF
  - DEV9 (hard disk/ethernet controller) I/O - requires SIF
    - Ethernet I/O - requires DEV9
    - Hard Disk I/O - requires DEV9
- VU Interface 0 (VIF0) to Vector Unit 0 - requires `prussia_dma`
- VU Interface 1 (VIF1) to Vector Unit 1 - requires `prussia_dma`
- Image Processing Unit - requires `prussia_dma`
