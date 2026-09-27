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


//enforce U/S below 
//not called also warning for horror 

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct Aligned4096(pub [u64; 512]);

static mut EXTRA_PDP: [Aligned4096; 4] = [Aligned4096([0; 512]); 4]; // C cant do this as malloc is problem
static mut EXTRA_PD: [Aligned4096; 4] = [Aligned4096([0; 512]); 4];
static mut EXTRA_PT: [Aligned4096; 4] = [Aligned4096([0; 512]); 4];
static mut NPDP: usize = 0;
static mut NPD: usize = 0;
static mut NPT: usize = 0;

fn pml4() -> *mut u64 {
    let c = cr3();
    c as *mut u64 //phys virt for low tables for now cant be botherd
}

unsafe fn alloc_table(pool: *mut Aligned4096, used: *mut usize) -> *mut u64 {
    let i = *used;
    *used += 1;
    let t = (*pool.add(i)).0.as_mut_ptr();
    for j in 0..512 { *t.add(j) = 0; }
    t
}

pub unsafe fn mark_user_page(phys: u64) { //exacly 1 4k page splits 2m huge 
    let p4 = pml4();
    let i4 = ((phys >> 39) & 0x1FF) as usize;
    let i3 = ((phys >> 30) & 0x1FF) as usize;
    let i2 = ((phys >> 21) & 0x1FF) as usize;
    let i1 = ((phys >> 12) & 0x1FF) as usize;
    *p4.add(i4) |= 0x4; //U/S bit baby 0x4 :)
    let p3 = (*p4.add(i4) & 0x000FFFFFFFFFF000) as *mut u64;
    *p3.add(i3) |= 0x4;
    let p2 = (*p3.add(i3) & 0x000FFFFFFFFFF000) as *mut u64;
    let pt: *mut u64;
    if *p2.add(i2) & 0x80 != 0 { //still huge splits it before touching page
        let base = *p2.add(i2) & 0x000FFFFFFFE00000;
        pt = alloc_table(EXTRA_PT.as_mut_ptr(), core::ptr::addr_of_mut!(NPT));
        for i in 0..512 {
            *pt.add(i) = (base + (i as u64) * 0x1000) | 0x3; //present rw still s only
        }
        *p2.add(i2) = (pt as u64) | 0x7;
    } else {
        pt = (*p2.add(i2) & 0x000FFFFFFFFFF000) as *mut u64;
    }
    *pt.add(i1) |= 0x4; //onlt rhis goes to ring3
    asm!("invlpg [{}]", in(reg) phys, options(nostack, preserves_flags));
}
pub unsafe fn map_user_range(phys: u64, size: u64) {
    let start = phys & !0xFFF;
    let end = (phys + size + 0xFFF) & !0xFFF;
    let mut a = start;
    while a < end {
        mark_user_page(a);
        a += 0x1000;
    }
}