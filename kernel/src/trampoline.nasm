%include "src/trampoline_to_protected_mode.nasm"
; %include "src/trampoline_to_long_mode_compat.nasm"
; %include "src/trampoline_to_long_mode.nasm"

mov al, 114
mov dx, 0x03f8
loop_r:
    mov al, 'R'
    out dx, al
    mov al, 10
    out dx, al
    jmp loop_r