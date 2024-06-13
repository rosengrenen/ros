%include "src/gdt.nasm"

%define CodeSegment 0x8
%define DataSegment 0x10

[BITS 16]
; data selector should be same as code selector
mov ax, cs
mov ds, ax

; code selector looks like 0x1234
; the base of the gdt entry needs to look like 0x0001_2340
; the base is divided into three parts, like 0xHHMM_LLLL
; thus we shift the code selector 4 bits to the left and set the result to the low bits (LLLL)
; we then shift the code selctor 12 bits to the right and set the result to the mid bits (MM)
; where the high half of the mid bits will always be 0x00
; set base low from code selector
mov ax, cs
shl ax, 4
mov [gdt.CodeBaseLow], ax
mov [gdt.DataBaseLow], ax

; set base mid from code selector
mov ax, cs
shr ax, 12
mov [gdt.CodeBaseMid], al
mov [gdt.DataBaseMid], al

cli
lgdt [ds:gdtr]
mov eax, cr0
or al, 1 ; TODO: add constant?
mov cr0, eax

mov ax, DataSegment
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax
jmp CodeSegment:protected_mode

align 8
gdt:
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
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_SEGMENT_32BIT | 0x0f
    .CodeBaseHigh db 0x00
    ; kernel data segment
    .DataSegment:
    ; limit low
    dw 0xffff
    .DataBaseLow dw 0x0000
    .DataBaseMid db 0x00
    ; access
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_SEGMENT_32BIT | 0x0f
    .DataBaseHigh db 0x00
gdtr:
    dw gdtr - gdt - 1
    dd gdt + 0x9f000

protected_mode: