global start

section .text
bits 32

start:
  mov word [0xb8000], 0x024F ; O
  mov word [0xb8002], 0x024B ; K
  hlt
