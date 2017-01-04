header_start:
  dd 0xe85250d6                ; multiboot magic number
  dd 0                         ; protected mode code
  dd header_end - header_start ; header length

  ; checksum
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ; end tag
  dw 0 ; type
  dw 0 ; flags
  dd 8 ; size
header_end:
