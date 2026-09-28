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
    *(.eh_frame* .eh_frame_hdr*)
  }
  . = ALIGN(0x1000);
  .limine_requests : {
    KEEP(*(.limine_requests_start*))
    KEEP(*(.limine_requests))
    KEEP(*(.limine_requests_end*))
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