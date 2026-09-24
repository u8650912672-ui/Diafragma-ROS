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
mod paging;
mod idt;
mod pic;
mod ps2;
mod shell;
mod cmd;

use core::arch::asm;
use core::panic::PanicInfo;

fn halt() -> ! { loop { unsafe { asm!("hlt"); } } }

//dont open a merge request or issue about this its a work of art

//let me do it again :3
// okay when i update this main ill fix it so first to be fixed 
#[panic_handler] fn panic(info: &PanicInfo) -> ! { /*taken from my old panic.c*/ serial::ser_puts("\nPANIC: "); let _ = info; serial::ser_puts("something failed so shit de...\n"); if fb::fb_on() { fb::fb_puts("\nPANIC RIP :c\n"); } halt() }
#[unsafe(no_mangle)] pub extern "C" fn _start() -> ! {
    serial::ser_init();
    let has_fb = fb::fb_init();
    gdt::gdt_init(); serial::ser_puts("gdt running :3\n");
    paging::paging_probe(); idt::idt_init(); pic::pic_init(); ps2::ps2_init();
    serial::ser_puts("DiafragmaOS has kinda started or booted congrats stuff will break and im a lazy guy press esc halts :3\n");
    if has_fb { fb::fb_puts("you can press buttons esc will halt the OS\n"); }
    unsafe { asm!("sti", options(nomem, nostack, preserves_flags)); } //let irq33 in ig?
    shell::shell_run(has_fb);
    halt();
}
