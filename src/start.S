.globl start

.section .text.start

start:
    mrc p15, 0, r0, c0, c0, 5
    and r0, r0, #3
    mov r1, r0
    add r1, r1, #1
    ldr r2, =0x00001000
    mul r2, r2, r1
    orr r2, #0x10000000

    mov sp, r2
    sub sp, #4
    str r0, [sp]
    bl main

hang:
    wfe
    b hang
