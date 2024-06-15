; data from BSP
%define SegmentBase 0x800
%define StackRoot 0x808
%define KernelStart 0x810
%define Pml4Addr 0x818

; GDT offsets
%define CodeSegment 0x08
%define DataSegment 0x10
%define CodeSegmentWithBase 0x18
%define DataSegmentWithBase 0x20

; gdt access bits
GDT_ACCESS_PRESENT equ 1 << 7
GDT_ACCESS_NOT_SYSTEM_SEGMENT equ 1 << 4
GDT_ACCESS_EXECUTABLE equ 1 << 3
GDT_ACCESS_READ_WRITE equ 1 << 1
GDT_ACCESS_ACCESSED equ 1 << 0

; gdt flag bits
GDT_FLAG_GRANULARITY_4K equ 1 << 7
GDT_FLAG_SEGMENT_32BIT equ 1 << 6
GDT_FLAG_LONG_MODE equ 1 << 5

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
mov ax, [SegmentBase]
mov [gdt32.CodeBaseLow], ax
mov [gdt32.DataBaseLow], ax
add [gdtr32.PointerLow], ax
add [gdtr64.PointerLow], ax

; set base mid from code selector
mov al, [SegmentBase + 2]
mov [gdt32.CodeBaseMid], al
mov [gdt32.DataBaseMid], al
mov [gdtr32.PointerHigh], al
mov [gdtr64.PointerHigh], al

cli
lgdt [ds:gdtr32]
mov eax, cr0
or al, 1 ; TODO: add constant?
mov cr0, eax

mov ax, DataSegmentWithBase
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax
jmp CodeSegmentWithBase:protected_mode


[BITS 32]
protected_mode:

mov ebx, [SegmentBase]
mov eax, ebx
add eax, protected_mode_zeroed_segment_selector

push CodeSegment
push eax
retf

protected_mode_zeroed_segment_selector:
mov ax, DataSegment
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax

add esp, ebx

; load bsp pml4 into cr3
mov eax, [ebx + Pml4Addr]
mov cr3, eax

; set pae bit of cr4
mov eax, cr4
or eax, 1 << 5
mov cr4, eax

mov ecx, 0xC0000080          ; Set the C-register to 0xC0000080, which is the EFER MSR.
rdmsr                        ; Read from the model-specific register.
or eax, 1 << 8               ; Set the LM-bit which is the 9th bit (bit 8).
wrmsr                        ; Write to the model-specific register.

mov eax, cr0                 ; Set the A-register to control register 0.
or eax, 1 << 31              ; Set the PG-bit, which is the 32nd bit (bit 31).
mov cr0, eax                 ; Set control register 0 to the A-register.


mov eax, ebx
add eax, long_mode_64bit

lgdt [ebx + gdtr64]

push CodeSegment
push eax
retf

[BITS 64]
long_mode_64bit:
mov ax, DataSegment
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax

; TODO: calc based on acpi id
mov rsp, [ebx + StackRoot]

mov rax, [ebx + KernelStart]
jmp rax

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
    .PointerLow dw gdt64
    .PointerHigh dw 0
