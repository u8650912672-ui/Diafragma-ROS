//pic so only keyboard irq1 call talk :3
use core::arch::asm;
unsafe fn outb(p: u16, b: u8) {
    asm!("out dx, al", in("dx") p, in("al") b, options(nomem, nostack, preserves_flags));
}

pub fn pic_init() { //i bet yall want to understand so ill comment this time for every line untill i wont
    unsafe {
        outb(0x20, 0x11); outb(0xA0, 0x11); // this is init but hey init WAIT FOR ICW2
        outb(0x21, 0x20); outb(0xA1, 0x28); //vector offset by 32+40 so irq is 33 :3
        outb(0x21, 0x04); outb(0xA1, 0x02); // cascade and slave at irq2
        outb(0x21, 0x01); outb(0xA1, 0x01); // 8086 mode instead of MCS80 shit
        outb(0x21, 0xFD); outb(0xA1, 0xFF); //mask all except IRQ1 on master 0xFD means only keyboiard talsk
    }
}

pub fn pic_eoi(irq: u8) {
    unsafe {
        if irq >= 8 { outb(0xA0, 0x20); } //slave: HEY MASTER i got stuff to tell you :D
        outb(0x20, 0x20); //master:slave silcene
    }
}
