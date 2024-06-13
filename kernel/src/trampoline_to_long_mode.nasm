%include "src/gdt.nasm"

%define CodeSegment64 0x08
%define DataSegment64 0x10

[BITS 32]
lgdt [gdtr64 + 0x9f000] ; TODO: should not be hard coded

push CodeSegment64
mov eax, long_mode_64bit
add eax, 0x9f000 ; TODO: should not be hard coded
push eax
retf

align 8
gdt64:
    ; null descriptor
    dq 0
    ; kernel code segment
    dw 0xffff
    dw 0x0000
    db 0x00
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_EXECUTABLE | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_LONG_MODE | 0x0f
    db 0x00
    ; kernel data segment
    dw 0xffff
    dw 0x0000
    db 0x00
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_LONG_MODE | 0x0f
    db 0x00
gdtr64:
    dw gdtr64 - gdt64 - 1
    dq gdt64 + 0x9f000 ; TODO: should not be hard coded

[BITS 64]
long_mode_64bit:
mov ax, DataSegment64
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax

mov rsp, 0xffff80000097b000 ; TODO: should not be hard coded

; works
push 0xffffffff80005170 ; TODO: should not be hard coded
ret

; does not work
; generates page fault trying to access ffffffff800a4160
; which is not at all the address we want to jump to
; jmp 0xffffffff80005160 ; TODO: should not be hard coded
