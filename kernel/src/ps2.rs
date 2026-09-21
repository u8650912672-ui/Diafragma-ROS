// ps2 so only ring0 talks to 0x60 0x64 (i used the arch linux source code to make this)
use core::arch::asm;

const DATA: u16 = 0x60;
const STAT: u16 = 0x64;
const BUFN: usize = 64;

static mut KBUF: [u8; BUFN] = [0; BUFN];
static mut KHEAD: usize = 0;
static mut KTAIL: usize = 0;

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
pub fn kbd_irq() { //called from IRQ33 later instead of user
    unsafe {
        let sc = inb(DATA);
        let next = (KHEAD + 1) % BUFN;
        if next == KTAIL { return; } //full then drop like i usually do in swedish we use the same word for full and drunk
        KBUF[KHEAD] = sc;
        KHEAD = next;
    }    
}

fn hexb(b: u8) { //temp debug
    let h = b"0123456789ABCDEF";
    let s = [h[(b >> 4) as usize], h[(b & 0xF) as usize]];
    for c in s { crate::serial::ser_puts(core::str::from_utf8(&[c]).unwrap_or("?")); }
    crate::serial::ser_puts(" ");
}

pub fn ps2_poll() -> Option<u8> {
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
        let v = if KHEAD == KTAIL { None } else {
            let b = KBUF[KTAIL];
            KTAIL = (KTAIL + 1) % BUFN;
            Some(b)
        };
        asm!("sti", options(nomem, nostack, preserves_flags));
        v
    }
}

//horror if you go beyond
//YOU HAVE BEEN WARNED



static mut SHIFTED1: bool = false; //for shifty styff
static mut CAPSLOCK: bool = false; //OPPS CAPS LOCK IS STUCK
static mut EEE: bool = false; //who the fuck even uses this anyways this is a very nice copy paste

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

pub fn ps2_cook(sc: u8) -> Option<u8> { //raw hex into letts none = spooky letter rewritten from c to rust basicly the same
    unsafe {
        if EEE { EEE = false; return None; } //fuck this for now idk?
        if sc == 0xE0 { EEE = true; return None; }
        if sc & 0x80 != 0 { //release go brrrrr hihi
            let k = sc & 0x7F;
            if k == 0x2A || k == 0x36 { SHIFTED1 = false ; }
            return None; // now if you still dont think this is bad have my horrendus IF SPAM
        }
        if sc == 0x2A || sc == 0x36 { SHIFTED1 = true; return None; }
        if sc == 0x3A { CAPSLOCK = !CAPSLOCK; return None; } //caps lock stuck please help
        if (sc as usize) >= 128 { return None; }
        let mut c = NORMAL[sc as usize];
        if SHIFTED1 { c = SHIFTED2[sc as usize]; } 
        else if CAPSLOCK && c >= b'a' && c <= b'z' { c = c - (b'a' - b'A'); } //loudinater
        if c == 0 { return None; }
        Some(c)   
    }
}