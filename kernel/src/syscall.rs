//oh this shit gonna be funnnnnn C:<
//this is so ring3 can beg on its knees and hope that i as the develepor or you as a modder allows ring3 to use stuff :D
pub const N_READ_KEY: u64 = 0; //blocking returns ascci or 0x1B for esc
pub const N_WRITE: u64 = 1; // write ptr and len 256B ascii only cap so there is no heap bs i have to fix
pub const N_EXIT: u64 = 2; // replaces ESC halt later 
pub const EFER_SCE: u64 = 1; // syscall enable bit in the EFER
pub const FMASK_IF: u64 = 0x200; //clear IF on entry so no irq mids syscall TF/DF later
use core::arch::asm;

fn wrmsr(msr: u32, v: u64) {
    let lo = (v & 0xFFFFFFFF) as u32;
    let hi = (v >> 32) as u32;
    unsafe { asm!("wrmsr", in("ecx") msr, in("eax") lo, in("edx") hi, options(nomem, nostack, preserves_flags)); }
}

fn rdmsr(msr: u32) -> u64 {
    let lo: u32; let hi: u32;
    unsafe { asm!("rdmsr", in("ecx") msr, out("eax") lo, out("edx") hi, options(nomem, nostack, preserves_flags)); }
    ((hi as u64) << 32) | (lo as u64)
}

#[unsafe(naked)]
pub extern "C" fn syscall_entry() {
    core::arch::naked_asm!(
        "swapgs
         push rcx
         push r11
         call {h}
         pop r11
         pop rcx
         swapgs
         sysretq",
        h = sym syscall_handler,
    );
}


//dispath rax=num rdi/rsi/rdx=args 256N ascii only writing
static mut KWRITE_BUF: [u8; 256] = [0; 256]; //copy first then we print TOCTOU PROOF WARNING FOR RACE CONDITION 

fn user_ptr_ok(ptr: u64, len: u64) -> bool {
    if len == 0 || len > 256 { return false; } //cap so no heap games
    if ptr == 0 { return false; }
    if ptr >= 0x0000800000000000 { return false; } //low canonical only (hgih half means for kenrel to fuck off)
    if ptr.checked_add(len).is_none() { return false; }
    if ptr + len > 0x0000800000000000 { return false; }
    true
}

fn do_read_key() -> u64 { //blocking sleeps with hlt till irq33
    loop {
        //drain one cahr if present
        let sc_opt: Option<u8> = {
            // raw poll
            crate::ps2::ps2_poll()
        };
        if let Some(sc) = sc_opt {
            if sc == 0x01 { return 0x1B; }//esc raw exit
            if let Some(c) = crate::ps2::ps2_cook(sc) {
                if c < 0x80 { return c as u64; } //ascii only 
                //if none ascii? uhhhhhhhh nah impossible :3 istg this will fuck me later
            }
            //no
        } else {
            unsafe { core::arch::asm!("sti; hlt", options(nomem, nostack, preserves_flags)); } //sleep till enxt irq
        }
    }
}

fn do_write(ptr: u64, len: u64) -> u64 {
    if !user_ptr_ok(ptr, len) { return u64::MAX; } //-1= fuck off 
    unsafe {
        //copy then check then print prevents TOCTOU
        for i in 0..(len as usize) {
            let b = *(ptr as *const u8).add(i);
            if b >= 0x80 { return u64::MAX; } 
            KWRITE_BUF[i] = b;
        }
        //print from kernel buf not user ptr
        for i in 0..(len as usize) {
            let c = KWRITE_BUF[i];
            if c == b'\n' {
                crate::serial::ser_puts("\n");
            } else {
                crate::serial::ser_puts(core::str::from_utf8(&KWRITE_BUF[i..i+1]).unwrap_or("?"));
            }
        }
        // fb too (ring0 can ring3 cant security=good )
        for i in 0..(len as usize) {
            crate::fb::fb_putc(KWRITE_BUF[i]);
        }
    }
    len //return bytes written just like real oses do
}

pub extern "C" fn syscall_handler() {
    let (num, a, b): (u64, u64, u64);
    unsafe {
        core::arch::asm!(
           "mov {0}, rax
            mov {1}, rdi
            mov {2}, rsi",
            out(reg) num, out(reg) a, out(reg) b,
            options(nomem, nostack, preserves_flags),
        );
    }
    let ret = match num {
        N_READ_KEY => do_read_key(),
        N_WRITE => do_write(a, b),
        N_EXIT => { //replaces esc halt later this is the last RING0 elevation user can do for now
            crate::serial::ser_puts("\nexit() called dies\n");
            loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack, preserves_flags)); } }
        }
        _ => u64::MAX, // if unknown num = byeeee >:3
    };
    unsafe {
        core::arch::asm!("mov rax, {0}", in(reg) ret, options(nomem, nostack, preserves_flags));
    }
}

pub fn syscall_init() { //real star still no cpl3 entry
    unsafe {
        let efer = rdmsr(0xC0000080);
        wrmsr(0xC0000080, efer | EFER_SCE);
        wrmsr(0xC0000081, ((0x10u64) << 48) | ((0x08u64) << 32));
        wrmsr(0xC0000082, syscall_entry as u64);
        wrmsr(0xC0000084, FMASK_IF);
    }
    crate::serial::ser_puts("syscall not dead start set and dispatch is ready to mingle :D\n");
}
