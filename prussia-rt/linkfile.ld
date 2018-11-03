/* We need the linker to start executing code from _start to bring up the Rust runtime before Rust runs */
ENTRY(_start);
EXTERN(_start);

SECTIONS {
    /* The .text segment contains the executable machine code.
     * PS2 programs must load at 0x0010'0000 or higher, because the EE kernel reserves the first megabyte
     * of memory for internal use.
     */
    .text 0x00100000: {
        *(.text.4095);
        *(.text .text.*);
    } > USER_MEM

    /* The .rodata segment contains constant globals. */
    .rodata ALIGN(128) : {
        *(.rodata .rodata.*);
    } > USER_MEM

    /* The .data segment contains mutable globals initialised with a non-zero value. */
    .data ALIGN(128) : {
        *(.data .data.*);
    } > USER_MEM

    /* The .bss segment contains mutable globals initialised with a zero value. */
    .bss ALIGN(128) : {
        START_OF_BSS = .;
        *(.bss .bss.*);
        END_OF_BSS = .;
    } > USER_MEM

    PROVIDE(END_OF_RAM = 0x02000000);
}
