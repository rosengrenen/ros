    ; Access bits
GDT_ACCESS_PRESENT                   equ 1 << 7
GDT_ACCESS_NOT_SYSTEM_SEGMENT        equ 1 << 4
GDT_ACCESS_EXECUTABLE                equ 1 << 3
GDT_ACCESS_READ_WRITE                equ 1 << 1
GDT_ACCESS_ACCESSED                  equ 1 << 0

; Flags bits
GDT_FLAG_GRANULARITY_4K equ 1 << 7
GDT_FLAG_SEGMENT_32BIT  equ 1 << 6
GDT_FLAG_LONG_MODE      equ 1 << 5
