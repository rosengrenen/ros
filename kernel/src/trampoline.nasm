mov al, 114
mov dx, 0x03f8
loop:
    mov al, 114
    out dx, al
    mov al, 10
    out dx, al
    jmp loop