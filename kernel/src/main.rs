// rust is intresting i have been coding in C so long and honestly when i moved here
// i have a singel line to describe how rust feels
// C but they removed the floor
//nothing more thats genuinly how it feels, its very familiar but different
//there was never a floor in C either but they tell you that there is 
// rust just tells you straigth out that there is no floor

#![no_std]
#![no_main]

mod limine;
mod serial;
mod font;
mod fb;
mod gdt;

use core::arch::asm;
use core::panic::PanicInfo;

fn halt() -> ! { loop { unsafe { asm!("hlt"); } } }

//dont open a merge request or issue about this its a work of art

//let me do it again :3
#[panic_handler] fn panic(info: &PanicInfo) -> ! { /*taken from my old panic.c*/ serial::ser_puts("\nPANIC: "); let _ = info; serial::ser_puts("something failed so shit de...\n"); if fb::fb_on() { fb::fb_puts("\nPANIC RIP :c\n"); } halt() }
                                                        //down a line and its kernel start
#[unsafe(no_mangle)] pub extern "C" fn _start() -> ! { serial::ser_init(); let has_fb = fb::fb_init(); gdt::gdt_init(); serial::ser_puts("gdt alive :3\n"); serial::ser_puts("Diafragma_OS pre aplha is running on your pc\n"); serial::ser_puts("serial running? "); serial::ser_puts(if has_fb { "yes\n" } else { "nope\n" }); if has_fb { fb::fb_puts("diafragma worked and is running fully :DD\n"); fb::fb_puts("when you see this it means framebuffer worked\n"); } halt() }
