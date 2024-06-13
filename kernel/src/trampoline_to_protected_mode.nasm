%include "src/gdt.nasm"

%define CodeSegment32 0x08
%define DataSegment32 0x10
%define CodeSegment32WithBase 0x18
%define DataSegment32WithBase 0x20

[BITS 16]

; data selector should be same as code selector
mov ax, cs
mov ds, ax
mov sp, 0x1000

; code selector looks like 0x1234
; the base of the gdt entry needs to look like 0x0001_2340
; the base is divided into three parts, like 0xHHMM_LLLL
; thus we shift the code selector 4 bits to the left and set the result to the low bits (LLLL)
; we then shift the code selctor 12 bits to the right and set the result to the mid bits (MM)
; where the high half of the mid bits will always be 0x00
; set base low from code selector
mov ax, cs
shl ax, 4
mov [gdt32.CodeBaseLow], ax
mov [gdt32.DataBaseLow], ax
add [gdtr32.PointerLow], ax

; set base mid from code selector
mov ax, cs
shr ax, 12
mov [gdt32.CodeBaseMid], al
mov [gdt32.DataBaseMid], al
mov [gdtr32.PointerHigh], al

cli
lgdt [ds:gdtr32]
mov eax, cr0
or al, 1 ; TODO: add constant?
mov cr0, eax

mov ax, DataSegment32WithBase
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax
jmp CodeSegment32WithBase:protected_mode

align 8
gdt32:
    ; null descriptor
    dq 0
    ; kernel code segment with base 0
    dw 0xffff
    dw 0x0000
    db 0x00
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_EXECUTABLE | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_SEGMENT_32BIT | 0x0f
    db 0x00
    ; kernel data segment with base 0
    dw 0xffff
    dw 0x0000
    db 0x00
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_SEGMENT_32BIT | 0x0f
    db 0x00
    ; temporary kernel code segment with non-zero base
    dw 0xffff
    .CodeBaseLow dw 0x0000
    .CodeBaseMid db 0x00
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_EXECUTABLE | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_SEGMENT_32BIT | 0x0f
    db 0x00
    ; temporary kernel data segment with non-zero base
    dw 0xffff
    .DataBaseLow dw 0x0000
    .DataBaseMid db 0x00
    db GDT_ACCESS_PRESENT | GDT_ACCESS_NOT_SYSTEM_SEGMENT | GDT_ACCESS_READ_WRITE | GDT_ACCESS_ACCESSED
    db GDT_FLAG_GRANULARITY_4K | GDT_FLAG_SEGMENT_32BIT | 0x0f
    db 0x00
gdtr32:
    dw gdtr32 - gdt32 - 1
    .PointerLow dw gdt32
    .PointerHigh dw 0

[BITS 32]
protected_mode:

; get the real mode segment selector and put it in the `b` register
mov ebx, [gdt32.CodeBaseLow]
and ebx, 0x0000_ffff
mov ecx, [gdt32.CodeBaseMid]
and ecx, 0x0000_00ff
shl ecx, 16
or ebx, ecx
add ebx, protected_mode_zeroed_segment_selector

push CodeSegment32
push ebx
retf

protected_mode_zeroed_segment_selector:
mov ax, DataSegment32
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax
