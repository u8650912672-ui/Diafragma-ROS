// gdt so ring3 cant run run arbituary code in ring0 :3
#[repr(C, packed)]
pub struct Tss {
    reserved0: u32,
    pub rsp0: u64, pub rsp1: u64, pub rsp2: u64,
    reserved1: u64,
    pub ist1: u64, pub ist2: u64, pub ist3: u64,
    pub ist4: u64, pub ist5: u64, pub ist6: u64, pub ist7: u64,
    reserved2: u64,
    reserved3: u16,
    pub iopb: u16, //io bitmap offset and its set past end so it denys all ports for ring3 making bad stuff not happen
}

#[repr(C, packed)]
pub struct GdtPtr {
    pub limit: u16,
    pub base: u64,
}

static mut GDT: [u64; 10] = [0; 10]; //thats gonna be null + kcode &data + ucode &data + tss(with 2 slots) + spare
pub static mut TSS: Tss = Tss {
    reserved0: 0,
    rsp0: 0, rsp1: 0, rsp2: 0,
    reserved1: 0,
    ist1: 0, ist2: 0, ist3: 0, ist4: 0, ist5: 0, ist6: 0, ist7: 0,
    reserved2: 0,
    reserved3: 0,
    iopb: 0,
};

fn set_entry(i: usize, base: u32, limit: u32, access: u8, flags: u8) {
    unsafe {
        GDT[i] = (limit & 0xFFFF) as u64
            | (((base & 0xFFFF) as u64) << 16)
            | ((((base >> 16) & 0xFF) as u64) << 32)
            | ((access as u64) << 40)
            | ((((limit >> 16) & 0x0F) as u64) << 48)
            | (((flags & 0x0F) as u64) << 52)
            | ((((base >> 24) & 0xFF) as u64) << 56);
    }
}

fn set_entry64(i: usize, base: u64, limit: u32, access: u8, flags: u8) {
    set_entry(i, base as u32, limit, access, flags);
    unsafe { GDT[i + 1] = base >> 32; }
}

use core::arch::asm;
use core::ptr::{addr_of, addr_of_mut};

fn gdt_flush(p: &GdtPtr) {
    unsafe {
        asm!("lgdt [{0}]", in(reg) p, options(readonly, nostack, preserves_flags));
        //reload segments like my old gdtload asm did cuz im lazy to code a fully new one
        asm!(
           "push 0x08
            lea {tmp}, [rip + 2f]
            push {tmp}
            retfq
            2:",
            tmp = lateout(reg) _,
            options(nostack, preserves_flags),
        );
        asm!(
           "mov ax, 0x10
            mov ds, ax
            mov es, ax
            mov fs, ax
            mov gs, ax
            mov ss, ax",
            out("ax") _,
            options(nostack, preserves_flags),
        );
    }
}

fn tss_flush() {
    unsafe {
        asm!("ltr ax", in("ax") 0x28u16, options(nostack, preserves_flags)); // 0x28 = tss selector from below
    }
}

pub fn gdt_init() {
    unsafe {
        for i in 0..10 { GDT[i] = 0; }
    }
    set_entry64(1, 0, 0xFFFFF, 0x9A, 0xA); //ring0 code (0x08) :3
    set_entry64(2, 0, 0xFFFFF, 0x92, 0xC); //ring0 data 0x10
    set_entry64(3, 0, 0xFFFFF, 0xFA, 0xA); //ring 3 code 0x18 why am i commenting this?
    set_entry64(4, 0, 0xFFFFF, 0xF2, 0xC); // ring 3 data 0x20
    unsafe {
        let tss_base = addr_of!(TSS) as u64;
        let tss_size = core::mem::size_of::<Tss>() as u32;
        set_entry64(5, tss_base, tss_size - 1, 0x89, 0x0); //tss 0x28 fuck i hate math
        (*addr_of_mut!(TSS)).iopb = tss_size as u16; // past end to deny all ports for ring 3 :)
        (*addr_of_mut!(TSS)).rsp0 = 0; // will set once i have a real ring0 stack mabye its 0 cuz im not gonna lie
        let p = GdtPtr { limit: (10 * 8 - 1) as u16, base: addr_of!(GDT) as u64 };
        gdt_flush(&p);
        tss_flush();
    }
}