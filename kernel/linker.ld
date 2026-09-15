ENTRY(_start)
OUTPUT_ARCH(i386:x86-64)
SECTIONS {
  . = 0xffffffff80000000;
  .text : {
    *(.text .text.*)
  }
  . = ALIGN(0x1000);
  .rodata : {
    *(.rodata .rodata.*)
    *(.limine_requests_start* .limine_requests* .limine_requests_end*)
    *(.eh_frame* .eh_frame_hdr*)
  }
  . = ALIGN(0x1000);
  .data : {
    *(.data .data.*)
    *(.got* )
  }
  . = ALIGN(0x1000);
  .bss : {
    *(.bss .bss.*)
    *(COMMON)
  }
}