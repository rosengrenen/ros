%include "src/gdt.nasm"

%define CodeSegment64 0x8
%define DataSegment64 0x10

[BITS 32]
lgdt [gdtr64]         ; Load the 64-bit global descriptor table.
mov ax, DataSegment64
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax

jmp CodeSegment64:long_mode_64bit       ; Set the code segment and enter 64-bit long mode.


align 8
gdt64:
    ; null descriptor
    dq 0
    ; kernel code segment
    .CodeSegment:
    ; limit low
    dw 0xffff
    .CodeBaseLow dw 0x0000
    .CodeBaseMid db 0x00
    ; access
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_EXECUTABLE | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_LONG_MODE | 0x0f
    .CodeBaseHigh db 0x00
    ; kernel data segment
    .DataSegment:
    ; limit low
    dw 0xffff
    .DataBaseLow dw 0x0000
    .DataBaseMid db 0x00
    ; access
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_LONG_MODE | 0x0f
    .DataBaseHigh db 0x00
gdtr64:
    dw gdtr64 - gdt64 - 1
    dd gdt64 + 0x9f000

long_mode_64bit:
