/* SPDX-License-Identifier: GPL-3.0-or-later */

ENTRY(_start);

SECTIONS {
    .text 0x00000000 : {
        KEEP(*(.ee_exc_tlbmiss));

        . = 0x080;
        KEEP(*(.ee_exc_counter));

        . = 0x100;
        KEEP(*(.ee_exc_debug));

        . = 0x180;
        KEEP(*(.ee_exc_common));

        . = 0x200;
        KEEP(*(.ee_exc_interrupt));

        . = 0x280;
        *(.text .text.*);
    }

    .data : {
        *(.rodata .rodata.*);
        *(.data .data.*);
    }

    .bss : {
        START_OF_BSS = .;
        *(.bss .bss.*);
    }

    END_OF_BSS = .;
}
