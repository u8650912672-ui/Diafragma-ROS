// pat pat cmd :DD
use core::arch::asm;

fn puts2(c: u8, has_fb: bool) { //duel print it works
    if c == 8 {
        crate::serial::ser_puts("\x08");
        if has_fb { crate::fb::fb_putc(8); }
    } else if c == b'\n' {
        crate::serial::ser_puts("\n");
        if has_fb { crate::fb::fb_putc(b'\n'); }
    } else {
        crate::serial::ser_puts(core::str::from_utf8(&[c]).unwrap_or("?"));
        if has_fb { crate::fb::fb_putc(c); }
    }
}


pub fn shell_run(has_fb: bool) {
    let mut line = [0u8; 128];
    let mut n: usize = 0;
    crate::serial::ser_puts("cmd prompt should have run ");
    if has_fb { crate::fb::fb_puts("# -> "); }
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)); }
        while let Some(sc) = crate::ps2::ps2_poll() {
            if sc == 0x01 { //this is to be deleted in short
                crate::serial::ser_puts("\nEsc detected stopping\n");
                if has_fb { crate::fb::fb_puts("\nesc detected halting machine\n"); }
                loop { unsafe { asm!("cli; hlt", options(nomem, nostack, preserves_flags)); } }
            }
            if let Some(c) = crate::ps2::ps2_cook(sc) {
                if c == b'\n' {
                    puts2(b'\n', has_fb);
                    crate::cmd::exec(&line[..n], has_fb);
                    n = 0;
                    crate::serial::ser_puts("cmd prompt should have ran");
                    if has_fb { crate::fb::fb_puts("# -> "); }
                } else if c == 8 {
                    if n > 0 { n -= 1; puts2(8, has_fb); }
                } else if n < 127 {
                    line[n] = c; n += 1;
                    puts2(c, has_fb);
                }
            }
        }
    }
}
