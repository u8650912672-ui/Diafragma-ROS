//framebuffer i hope it works i rewrote it from C to rust and geniuinly wanna end myself
use crate::font;
use crate::limine;

static mut FB: *mut u8 = core::ptr::null_mut();
static mut W: u64 = 0;
static mut H: u64 = 0;
static mut PITCH: u64 = 0;
static mut BPPB: u64 = 0; //bytes per px simple and clean
static mut COL: u64 = 0;
static mut ROW: u64 = 0;
static mut ON: bool = false;

pub fn fb_on() -> bool { unsafe { ON } } //my way of telling rust that i know there is a race but i dont care :3

fn px(x: u64, y: u64, on: bool) {
    unsafe {
        if x >= W || y >= H { return; } //dont you fucking dare draw offscreen
        let p = FB.add((y * PITCH + x * BPPB) as usize);
        let v = if on { 0xFF } else { 0x00 };
        for i in 0..BPPB { *p.add(i as usize) = v; } //white vs black 
    }
}

fn draw_at(c: u8, x: u64, y: u64) {
    let g = font::glyph(c);
    for r in 0..16 {
        for xx in 0..8 {
            px(x + xx, y + r as u64, ((g[r] >> (7 - xx)) & 1) == 1);
        }
    }
}
//oh and yes all math was alredy done in my prevoius framebuffer in my past os :3

pub fn fb_clear() {
    unsafe { //rust iv told you to shush i know what i am doing 
        if FB.is_null() { return; }
        for i in 0..(PITCH * H) { *FB.add(i as usize) = 0; }
        COL = 0; ROW = 0;    
        }
}

pub fn fb_init() -> bool { //returns true if we get a fb if not... rip
    unsafe {
        let f = match limine::fb() {
            Some(f) => f,
            None => return false,
        };
        if f.w < 8 || f.h < 32 || f.pitch == 0 { return false; }
        FB = f.addr;
        W = f.w; H = f.h; PITCH = f.pitch;
        BPPB = (f.bpp / 8) as u64;
        if BPPB == 0 { return false; }
        ON = false;
        fb_clear();
        ON = true;
        true
    }
}

fn scroll() {
    unsafe {
        let line = PITCH * 16;
        for i in 0..(PITCH * H - line) {
            *FB.add(i as usize) = *FB.add((i + line) as usize);
        }
        for i in (PITCH * (H - 16))..(PITCH * H) {
            *FB.add(i as usize) = 0;
        }
        if ROW > 0 { ROW -= 1; }
    }
}

pub fn fb_putc(c: u8) {
    unsafe {
        if !ON { return; }
        if c == b'\n' { COL = 0; ROW += 1; }
        else if c == b'\x08' { // must be backspace ? rigth?
            if COL > 0 { COL -= 1; }
            else if ROW > 0 { ROW -= 1; COL = W / 8 - 1; }
            draw_at(b' ', COL * 8, ROW * 16);
            return;
        } else {
            draw_at(c, COL * 8, ROW * 16);
            COL += 1;
        }
        if COL >= W / 8 { COL = 0; ROW += 1; }
        if ROW >= H / 16 { scroll(); }
    }
}

pub fn fb_puts(s: &str) {
    for b in s.bytes() { fb_putc(b); }
}
