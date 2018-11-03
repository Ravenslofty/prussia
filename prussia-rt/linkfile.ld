ENTRY(_start)

MEMORY {
    /* The EE kernel reserves the first megabyte of RAM to itself. */
    KERNEL_MEM = 0x00000000;
    USER_MEM   = 0x00100000;
}

SECTIONS {
    /* The .text segment contains the executable machine code. */
    .text : {
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
