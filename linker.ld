ENTRY(main)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x7e00, l = 12K
}

SECTIONS {
    . = 0x7e00;

    .text.main : {
        *(.text.main)
    } >ram

    .text : {
        *(.text .text._ZN9panicking18panic_bounds_check20hdbeec153928a3c46UPvE)
    } >ram

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
