// rust is intresting i have been coding in C so long and honestly when i moved here
// i have a singel line to describe how rust feels
// C but they removed the floor
//nothing more thats genuinly how it feels, its very familiar but different
//there was never a floor in C either but they tell you that there is 
// rust just tells you straigth out that there is no floor

#![no_std]
#![no_main]

// the moderation wall 
use crate::limine::virt_to_phys;

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
mod syscall;
mod user;

use core::arch::asm;
use core::panic::PanicInfo;

fn halt() -> ! { loop { unsafe { asm!("hlt"); } } }

//dont open a merge request or issue about this its a work of art

// okay when i update this main ill fix it so first to be fixed 
#[panic_handler] fn panic(info: &PanicInfo) -> ! { /*taken from my old panic.c*/ serial::ser_puts("\nPANIC: "); let _ = info; serial::ser_puts("something failed so shit de...\n"); if fb::fb_on() { fb::fb_puts("\nPANIC RIP :c\n"); } halt() }
#[unsafe(no_mangle)] pub extern "C" fn _start() -> ! {
    serial::ser_init();
    let has_fb = fb::fb_init();
    gdt::gdt_init(); serial::ser_puts("gdt running :3\n"); paging::paging_probe(); idt::idt_init(); pic::pic_init(); ps2::ps2_init(); syscall::syscall_init(); serial::ser_puts("hhdm="); if crate::limine::hhdm().is_some() { serial::ser_puts("ok "); } else { serial::ser_puts("NONE\n"); } serial::ser_puts("exec="); if crate::limine::exec_bases().is_some() { serial::ser_puts("ok\n"); } else { serial::ser_puts("NONE\n"); } user::user_entry();
    unsafe { //mark ustack 2 pages + stub 1 page u verify before-> after still cpl0 so you cant exploit it yet
        let h = b"0123456789ABCDEF";
        let mut ph = |n: u64| {
            crate::serial::ser_puts("0x");
            for i in 0..16 {
                let b = h[((n >> (60 - i * 4)) & 0xF) as usize];
                crate::serial::ser_puts(core::str::from_utf8(&[b]).unwrap_or("?"));
            }
        };
        let (saddr, slen) = user::user_stack_range();
        let (caddr, clen) = user::user_code_range();
        if let (Some(_sp), Some(_cp)) = (virt_to_phys(saddr), virt_to_phys(caddr)) {
            //stack page 0 now
            let b0 = crate::paging::pte_of(saddr);
            crate::paging::mark_user_page(saddr);
            let a0 = crate::paging::pte_of(saddr);
            crate::serial::ser_puts("U stack0: "); ph(b0); crate::serial::ser_puts("->"); ph(a0); crate::serial::ser_puts("\n");
            //stack page 1 / ustack 8k = 2 pages
            let b1 = crate::paging::pte_of(saddr + 0x1000);
            crate::paging::mark_user_page(saddr + 0x1000);
            let a1 = crate::paging::pte_of(saddr + 0x1000);
            crate::serial::ser_puts("U stack1: "); ph(b1); crate::serial::ser_puts("-> "); ph(a1); crate::serial::ser_puts("\n");
            //stub code page ig?
            let b2 = crate::paging::pte_of(caddr);
            crate::paging::mark_user_page(caddr);
            let a2 = crate::paging::pte_of(caddr);
            crate::serial::ser_puts("U stub: "); ph(b2); crate::serial::ser_puts("->"); ph(a2); crate::serial::ser_puts("\n");
            //second half of stub if straddles? ( i want cuddles not straddels :3) user_code_range len 4096 is for 1 page enough for stub if not more then 4k and it wont be i think;-; and yea...
            let _ = (slen, clen);
            crate::serial::ser_puts("user pages marked u verified they are safe (probably) :3\n");
        } else {
            crate::serial::ser_puts("virt_to_phys NONE, no u markesmabye a stale iso? idfk\n");
        }
    } 

    serial::ser_puts("DiafragmaOS has kinda started or booted congrats stuff will break and im a lazy guy press esc halts :3\n");
    if has_fb { fb::fb_puts("you can press buttons esc will halt the OS\n"); }
    
    unsafe { asm!("sti", options(nomem, nostack, preserves_flags)); } //let irq33 in ig?
    // shell::shell_run(has_fb);
    crate::ps2::focus_user(); crate::ps2::flush_user();
    crate::user::user_stack_fill();
    crate::serial::ser_puts("entering ring3 like normal :3\n");
    crate::user::ring3_enter();
    halt();
}
