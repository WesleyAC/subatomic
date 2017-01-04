global start

section .text
bits 32

start:
  ; Point the first entry of the level 4 page table to the first entry in the p3 table
  mov eax, p3_table
  or eax, 0b11 ; set present and writable
  mov dword [p4_table + 0], eax

  ; Point the first entry of the level 3 page table to the first entry in the p2 table
  mov eax, p2_table
  or eax, 0b11 ; set present and writable
  mov dword [p3_table + 0], eax

  ; point each page table level two entry to a page
  mov ecx, 0         ; counter variable
.map_p2_table:
  mov eax, 0x200000  ; 2MiB
  mul ecx
  or eax, 0b10000011 ; set huge page, present, writable
  mov [p2_table + ecx * 8], eax
  inc ecx
  cmp ecx, 512
  jne .map_p2_table

  ; move page table address to cr3
  mov eax, p4_table
  mov cr3, eax

  ; enable Physical Address Extention
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  ; set the long mode bit
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr

  ; enable paging
  mov eax, cr0
  or eax, 1 << 31
  or eax, 1 << 16
  mov cr0, eax

  lgdt [gdt64.pointer]

  ; update selectors
  mov ax, gdt64.data
  mov ss, ax
  mov ds, ax
  mov es, ax

  ; jump to long mode!
  jmp gdt64.code:long_mode_start

  hlt


section .bss

align 4096

p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096

section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)
.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64

section .text
bits 64
long_mode_start:
  mov rax, 0x2f592f412f4b2f4f
  mov qword [0xb8000], rax

  hlt
