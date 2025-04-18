/* We need the linker to start executing code from _start to bring up the Rust runtime before Rust runs */
ENTRY(_start);
EXTERN(_start);

MEMORY {
    BIOS    : ORIGIN = 0x00000000, LENGTH = 528K /* 0x00000000 - 0x00084000: BIOS memory */
    BRAM    : ORIGIN = 0x00084000, LENGTH = 496K /* 0x00084000 - 0x00100000: BIOS unused memory */
    GRAM    : ORIGIN = 0x00100000, LENGTH =  31M /* 0x00100000 - 0x02000000: GAME memory */
    KSEG0   : ORIGIN = 0x80000000, LENGTH = 0x1FFFFFFF
    KSEG1   : ORIGIN = 0xA0000000, LENGTH = 0x1FFFFFFF
}

REGION_ALIAS("MAIN_REGION", GRAM);

PHDRS {
    /* Load the text segment from file. */
    text PT_LOAD;
}

SECTIONS {
    /* The .text segment contains the executable machine code.
     * PS2 programs must load at 0x0010'0000 or higher, because the EE kernel reserves the first megabyte
     * of memory for internal use.
     */
    .text : {
        *(.text .text.*);
    } > MAIN_REGION :text

    .sdata ALIGN(128) : {
        *(.sdata .sdata.*);
    } > MAIN_REGION

    /* The .data segment contains mutable globals initialised with a non-zero value. */
    .data ALIGN(128) : {
        *(.data .data.*);
    } > MAIN_REGION

    /* The .rodata segment contains constant globals. */
    .rodata ALIGN(128) : {
        *(.rodata .rodata.*);
    } > MAIN_REGION

    /* The .bss segment contains mutable globals initialised with a zero value. */
    .sbss ALIGN(128) : {
        START_OF_BSS = .;
        *(.sbss .sbss.*);
    } > MAIN_REGION

    .bss ALIGN(128) : {
        *(.bss .bss.*);
        END_OF_BSS = .;
    } > MAIN_REGION

    PROVIDE(END_OF_RAM = 0x02000000);

    /DISCARD/ : { *(.reginfo) *(.MIPS.abiflags) }
}
