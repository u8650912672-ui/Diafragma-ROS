//idt so gp and pf + keyboard irq dont reboot the pc cuz that would be bad
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    off_lo: u16,
    sel: u16,
    ist: u8,
    attr: u8,
    off_mid: u16,
    off_hi: u32,
    zero: u32,
}

#[repr(C, packed)]
pub struct IdtPtr {
    pub limit: u16,
    pub base: u64,
}

#[repr(align(16))]
pub struct AlignedIdt(pub [IdtEntry; 256]);
static mut IDT: AlignedIdt = AlignedIdt([IdtEntry {
    off_lo: 0, sel: 0, ist: 0, attr: 0,
    off_mid: 0, off_hi: 0, zero: 0,
}; 256]); // cant derive copy cuz packed so init one by one later like waht i did last time

fn set_gate(n: usize, h: u64) {
    unsafe {
        let e = &mut (*core::ptr::addr_of_mut!(IDT)).0[n];
        e.off_lo = (h & 0xFFFF) as u16;
        e.off_mid = ((h >> 16) & 0xFFFF) as u16;
        e.off_hi = ((h >> 32) & 0xFFFFFFFF) as u32;
        e.sel = 0x08; //64 bit code segment :3
        e.ist = 0;
        e.attr = 0x8E; //present ring0 64 but interrupt gate
        e.zero = 0;
    }
}

use core::arch::asm;
use core::ptr::addr_of;
use core::arch::naked_asm;

//default boom handler serial no fb then halt like in the past
#[unsafe(no_mangle)]
pub extern "C" fn isr_default() {
    crate::serial::ser_puts("\nFAULT TRIGGERD OH FUCK SERIAL CHECK IT and ofc REBOOT THE SYSTEM\n");
    loop { unsafe { asm!("cli; hlt", options(nomem, nostack, preserves_flags)); } }
}
// naked stub to push dummy err and vector , then just jump to isr default without regs saved cuz it dont matter either way
#[unsafe(naked)]
pub extern "C" fn isr_stub_default() {
        naked_asm!(
            "push rax
             push rcx
             push rdx
             call {f}
             pop rdx
             pop rcx
             pop rax
             iretq",
            f = sym isr_default,
        );
}
pub fn idt_init() {
    unsafe {
        let h = isr_stub_default as u64;
        for i in 0..256 { set_gate(i, h); }//all to default first pre vector next
        set_gate(33, isr_stub_33 as u64); //keyboard has its own stub
        let p = IdtPtr { limit: (256 *16 - 1) as u16, base: addr_of!(IDT) as u64 };
        asm!("lidt [{}]", in(reg) &p, options(readonly, nostack, preserves_flags));
    }
    crate::serial::ser_puts("idt alive, all faults go to default :D\n");
}

#[unsafe(naked)]
pub extern "C" fn isr_stub_33() {
    naked_asm!(
       "push rax
        push rcx
        push rdx
        push rsi
        push rdi
        push r8
        push r9
        push r10
        push r11
        call {f}
        pop r11
        pop r10
        pop r9
        pop r8
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rax
        iretq",
    f = sym irq33_handler,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn irq33_handler() {
    crate::ps2::kbd_irq(); //now push the raw scancodes to ring0
    crate::pic::pic_eoi(1); //eoi master only 
}