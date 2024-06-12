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
mov al, 114
mov dx, 0x03f8
loop:
    mov al, 114
    out dx, al
    mov al, 10
    out dx, al
    jmp loop

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