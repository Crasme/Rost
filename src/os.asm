mov ah, 0x0e ; mode teletype

mov al, 'R'
int 0x10
mov al, 'O'
int 0x10
mov al, 'S'
int 0x10
mov al, 'T'
int 0x10

boucle:
    jmp boucle

times 510 -( $ - $$ ) db 0

dw 0xaa55