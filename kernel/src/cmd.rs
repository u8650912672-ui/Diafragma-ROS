fn str_eq(a: &[u8], b: &[u8]) -> bool { //like my old str eq no alloc shitery
    if a.len() != b.len() { return false; }
    for i in 0..a.len() { if a[i] != b[i] { return false; } }
    true
}

pub fn exec(line: &[u8], has_fb: bool) {
    if str_eq(line, b"patpat") {
        crate::serial::ser_puts("aweee cutee you want headpattssss :3\n");
        if has_fb { crate::fb::fb_puts("aweee cutee you want headpattssss :3\n"); }
    } else if line.len() == 0 {
        //this empty enter
    } else {
        crate::serial::ser_puts("ERR typo for a command mabye?\n");
        if has_fb { crate::fb::fb_puts("typo for a command mayhaps :3?\n"); }
    }
}