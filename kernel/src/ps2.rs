// ps2 so only ring0 talks to 0x60 0x64 (i used the arch linux source code to make this)
use core::arch::asm;

const DATA: u16 = 0x60;
const STAT: u16 = 0x64;

struct RawQ { buf: [u8; 128], head: usize, tail: usize, shift: bool, caps: bool, e0: bool, e1: u8, dropped: u64 }
impl RawQ { const fn empty() -> Self { RawQ { buf: [0; 128], head: 0, tail: 0, shift: false, caps: false, e0: false, e1: 0, dropped: 0 } } }

static mut KQ: RawQ = RawQ::empty();
static mut UQ: RawQ = RawQ::empty();
static FOCUS: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);
pub fn focus_user() { FOCUS.store(1, core::sync::atomic::Ordering::Relaxed); }
pub fn focus_kernel() { FOCUS.store(0, core::sync::atomic::Ordering::Relaxed); }
pub fn flush_user() { unsafe { UQ.head = 0; UQ.tail = 0; UQ.shift = false; UQ.e0 = false; UQ.e1 = 0; } }
pub fn flush_kernel() { unsafe { KQ.head = 0; KQ.tail = 0; KQ.shift = false; KQ.e0 = false; KQ.e1 = 0; } }



unsafe fn outb(p: u16, b: u8) {
    asm!("out dx, al", in("dx") p, in("al") b, options(nomem, nostack, preserves_flags));
}
unsafe fn inb(p: u16) -> u8 {
    let b: u8;
    asm!("in al, dx", in("dx") p, out("al") b, options(nomem, nostack, preserves_flags));
    b
}

fn drain() {
    unsafe { while inb(STAT) & 1 != 0 { let _ = inb(DATA); } } //eat the stale stuff so it dont poisen later stuff
}
fn wait_data(timeout: i32) -> bool {
    let mut t = timeout;
    while t > 0 {
        unsafe { if inb(STAT) & 1 != 0 { return true; } }
        t -= 1;
    }
    false
}

fn send_dev(b: u8) {
    unsafe {
        while inb(STAT) & 2 != 0 {} //wait input till empty
        outb(DATA, b);
    }
}

pub fn ps2_init() { //kind a same as my old keyboard init as in cinderRebornOS
    unsafe {
        outb(STAT, 0xAD); // disable so it can be poked
        drain();
        send_dev(0xFF); // reset, make sure i dont have to  recode this ever again
        for _ in 0..3 { if wait_data(100000) { let _ = inb(DATA); } }
        send_dev(0xF0); // give me my fucking scancodes pleaseee
        if wait_data(100000) { let _ = inb(DATA); }
        send_dev(0x01); // set 1 plss :3
        if wait_data(100000) { let _ = inb(DATA); }
        outb(STAT, 0xAE); //enable it finally
        drain();
        outb(STAT, 0x20); //read config
        if wait_data(100000) {
            let mut cfg = inb(DATA);
            cfg = (cfg & !0x40) |  0x01; //clear xlate enable irq masked by pic for now
            while inb(STAT) & 2 != 0 {}
            outb(STAT, 0x60); // write config
            outb(DATA, cfg);
        }
    }
    crate::serial::ser_puts("ps2 is alive you can write now :D\n");
}

pub fn kbd_irq() {
    unsafe {
        let sc = inb(DATA);
        let q = if FOCUS.load(core::sync::atomic::Ordering::Relaxed) == 1 { &mut *core::ptr::addr_of_mut!(UQ) } else { &mut *core::ptr::addr_of_mut!(KQ) };
        let next = (q.head + 1) % 128;
        if next == q.tail { q.dropped += 1; return; }
        q.buf[q.head] = sc;
        q.head = next;
    }
}

fn hexb(b: u8) { //temp debug
    let h = b"0123456789ABCDEF";
    let s = [h[(b >> 4) as usize], h[(b & 0xF) as usize]];
    for c in s { crate::serial::ser_puts(core::str::from_utf8(&[c]).unwrap_or("?")); }
    crate::serial::ser_puts(" ");
}

fn pop_raw(q: &mut RawQ) -> Option<u8> {
    if q.head == q.tail { None } else { let b = q.buf[q.tail]; q.tail = (q.tail + 1) % 128; Some(b) }
}

pub fn ps2_poll() -> Option<u8> {
    unsafe {
        let f: u64; asm!("pushfq; pop {}; cli", out(reg) f, options(nomem, nostack));
        let v = pop_raw(&mut *core::ptr::addr_of_mut!(KQ));
        asm!("push {}; popfq", in(reg) f, options(nomem, nostack));
        v
    }
}

pub fn ps2_poll_user() -> Option<u8> {
    unsafe {
        let f: u64; asm!("pushfq; pop {}; cli", out(reg) f, options(nomem, nostack));
        let v = pop_raw(&mut *core::ptr::addr_of_mut!(UQ));
        asm!("push {}; popfq", in(reg) f, options(nomem, nostack));
        v
    }
}
//horror if you go beyond
//YOU HAVE BEEN WARNED



static NORMAL: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8,
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',
    0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
    0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1', b'2', b'3', b'0', b'.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static SHIFTED2: [u8; 128] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 8,
    b'\t', b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', b'\n',
    0, b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~',
    0, b'|', b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?',
    0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1', b'2', b'3', b'0', b'.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn cook_one(q: &mut RawQ, sc: u8) -> Option<u8> {
    if sc == 0xE1 { q.e1 = 2; q.e0 = false; return None; }
    if q.e1 > 0 { q.e1 -= 1; return None; }
    if q.e0 { q.e0 = false; return None; }
    if sc == 0xE0 { q.e0 = true; return None; }
    if sc & 0x80 != 0 {
        let k = sc & 0x7F;
        if k == 0x2A || k == 0x36 { q.shift = false; }
        return None;
    }
    if sc == 0x2A || sc == 0x36 { q.shift = true; return None; }
    if sc == 0x3A { q.caps = !q.caps; return None; }
    if (sc as usize) >= 128 { return None; }
    let mut c = NORMAL[sc as usize];
    if q.shift { c = SHIFTED2[sc as usize]; }
    else if q.caps && c >= b'a' && c <= b'z' { c = c - (b'a' - b'A'); }
    if c == 0 { return None; }
    Some(c)
}
pub fn ps2_cock(sc: u8) -> Option<u8> { unsafe { cook_one(&mut *core::ptr::addr_of_mut!(KQ), sc) } }
pub fn ps2_cook_user(sc: u8) -> Option<u8> { unsafe { cook_one(&mut *core::ptr::addr_of_mut!(UQ), sc) } }