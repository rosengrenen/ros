[BITS 16]
; Enter protected mode
mov ax, cs
mov ds, ax

cli
lgdt [ds:gdt_descriptor]
mov eax, cr0
or al, 1
mov cr0, eax

mov ax, 0x10
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax
jmp 0x08:protected_mode

[BITS 32]
protected_mode:
mov eax, 0x80000001    ; Set the A-register to 0x80000001.
cpuid                  ; CPU identification.
test edx, 1 << 29      ; Test if the LM-bit, which is bit 29, is set in the D-register.
jz NoLongMode         ; They aren't, there is no long mode.

mov eax, cr4                 ; Set the A-register to control register 4.
or eax, 1 << 5               ; Set the PAE-bit, which is the 6th bit (bit 5).
mov cr4, eax                 ; Set control register 4 to the A-register.

mov ecx, 0xC0000080          ; Set the C-register to 0xC0000080, which is the EFER MSR.
rdmsr                        ; Read from the model-specific register.
or eax, 1 << 8               ; Set the LM-bit which is the 9th bit (bit 8).
wrmsr                        ; Write to the model-specific register.

mov eax, cr0                 ; Set the A-register to control register 0.
or eax, 1 << 31              ; Set the PG-bit, which is the 32nd bit (bit 31).
mov cr0, eax                 ; Set control register 0 to the A-register.

; lgdt [GDT.Pointer]         ; Load the 64-bit global descriptor table.
; jmp GDT.Code:Realm64       ; Set the code segment and enter 64-bit long mode.

mov al, 114
mov dx, 0x03f8
loop:
    mov al, 114
    out dx, al
    mov al, 10
    out dx, al
    jmp loop

NoLongMode:
mov al, 115
mov dx, 0x03f8
loop2:
    mov al, 115
    out dx, al
    mov al, 10
    out dx, al
    jmp loop2

align 8
gdt_start:
    dq 0
    ; kernel code segment
    dq 0x00c0_9a09_f000_ffff
    ; kernel data segment
    dq 0x00c0_9209_f000_ffff
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start + 0x9f000

    ; Access bits
PRESENT        equ 1 << 7
NOT_SYS        equ 1 << 4
EXEC           equ 1 << 3
DC             equ 1 << 2
RW             equ 1 << 1
ACCESSED       equ 1 << 0
 
; Flags bits
GRAN_4K       equ 1 << 7
SZ_32         equ 1 << 6
LONG_MODE     equ 1 << 5
 
GDT:
    .Null: equ $ - GDT
        dq 0
    .Code: equ $ - GDT
        dd 0xFFFF                                   ; Limit & Base (low, bits 0-15)
        db 0                                        ; Base (mid, bits 16-23)
        db PRESENT | NOT_SYS | EXEC | RW            ; Access
        db GRAN_4K | LONG_MODE | 0xF                ; Flags & Limit (high, bits 16-19)
        db 0                                        ; Base (high, bits 24-31)
    .Data: equ $ - GDT
        dd 0xFFFF                                   ; Limit & Base (low, bits 0-15)
        db 0                                        ; Base (mid, bits 16-23)
        db PRESENT | NOT_SYS | RW                   ; Access
        db GRAN_4K | SZ_32 | 0xF                    ; Flags & Limit (high, bits 16-19)
        db 0                                        ; Base (high, bits 24-31)
    .TSS: equ $ - GDT
        dd 0x00000068
        dd 0x00CF8900
    .Pointer:
        dw $ - GDT - 1
        dq GDT


[BITS 64]
 
Realm64:
    cli                           ; Clear the interrupt flag.
    mov ax, 0x10            ; Set the A-register to the data descriptor.
    mov ds, ax                    ; Set the data segment to the A-register.
    mov es, ax                    ; Set the extra segment to the A-register.
    mov fs, ax                    ; Set the F-segment to the A-register.
    mov gs, ax                    ; Set the G-segment to the A-register.
    mov ss, ax                    ; Set the stack segment to the A-register.

mov al, 116
mov dx, 0x03f8
loop3:
    mov al, 116
    out dx, al
    mov al, 10
    out dx, al
    jmp loop3