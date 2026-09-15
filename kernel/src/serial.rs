//com 1 serial so i can actually have a visual understanding before something dies :D
use core::arch::asm;

unsafe fn outb(p: u16, v: u8) {
    asm!("out dx, al", in("dx") p, in("al") v);
}

unsafe fn inb(p: u16) -> u8 {
    let v: u8;
    asm!("in al, dx", in("dx") p, out("al") v);
    v
}

pub fn ser_init() { //init com 1 38400 8n1 so qemu is happy ig? 
    unsafe {
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x80);
        outb(0x3F8, 0x03);
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x03);
        outb(0x3F8 + 2, 0xC7);
    }
}

fn ser_putb(b: u8) {
    unsafe {
        while inb(0x3F8 + 5) & 0x20 == 0 {} //wait till its not in use
        outb(0x3F8, b);
    }
}

pub fn ser_puts(s: &str) { //console spamming the whole string to serial cuz we can :D
    for b in s.bytes() {
        if b == b'\n' { ser_putb(b'\r'); } //qemu likes to nom nom on the \r or else lines fucked
        ser_putb(b);
    }
}
