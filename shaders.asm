section .data
    ; Definindo as variáveis de entrada e saída para o shader
    input_position   db 0x00, 0x00, 0x00   ; Posição do vértice
    input_color      db 0xFF, 0x00, 0x00   ; Cor vermelha

    output_position  db 0x00, 0x00, 0x00   ; Posição de saída do vértice
    output_color     db 0x00, 0x00, 0x00   ; Cor de saída (alterada)

section .text
global _start

_start:
    ; Calculando a posição do vértice 
    ; Vamos apenas mover o vértice para um novo local, alterando a posição
    mov al, [input_position]
    add al, 10           ; desloca 10 unidades no eixo x
    mov [output_position], al

    ; Alterando a cor com base na posição (simples manipulação)
    mov al, [input_position]
    cmp al, 5
    jg change_color

    ; Caso não mude a cor, mantemos a cor original
    mov [output_color], [input_color]
    jmp end_shader

change_color:
    ; Modificando a cor para verde quando a posição for maior que 5
    mov byte [output_color], 0x00   ; R
    mov byte [output_color + 1], 0xFF ; G
    mov byte [output_color + 2], 0x00 ; B

end_shader:
    ; Finalizando o shader e saindo 
    ret
