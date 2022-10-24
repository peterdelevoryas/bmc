.globl start

.section .text.start

start:
    mrc p15, 0, r0, c0, c0, 5
    and r0, r0, #3
    cmp r0, #0
    bne hang

    ldr sp, =0x10006000
    bl main

hang:
    wfe
    b hang
