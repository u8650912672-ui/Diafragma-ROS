//paging so you cant touchy touchy ring 0 from ring3 or FB/kernel cuz that would be bad stolen from cidnerros
use core::arch::asm;
use crate::serial;

fn cr3() -> u64 {
    let v: u64; //trauma of v returns
    unsafe { asm!("mov {}, cr3", out(reg) v, options(nostack, preserves_flags)); }
    v // cr3 physical addr of pm14, identity mapped so we can read it also fuck you V
}

fn hex(n: u64) {
    let h = b"0123456789ABCDEF";
    let mut s = [b'0'; 18];
    s[0] = b'0'; s[1] = b'x';
    for i in 0..16 {
        s[2 + i] = h[((n >> (60 - i * 4)) & 0xF) as usize];
    }
    //print without fmt cuz no_std :)
    let mut buf = [0u8; 18];
    buf.copy_from_slice(&s);
    for b in buf { crate::serial::ser_puts(core::str::from_utf8(&[b]).unwrap_or("?")); }
    crate::serial::ser_puts("\n");
}

pub fn paging_probe() { //R/O before we touch anything
    let c = cr3();
    serial::ser_puts("paging probe, cr3=");
    hex(c);
    serial::ser_puts("US isolation hasent been implemented :c next will mark user pages :3\n");
}
