fn str_eq(a: &[u8], b: &[u8]) -> bool { //like my old str eq no alloc shitery
    if a.len() != b.len() { return false; }
    for i in 0..a.len() { if a[i] != b[i] { return false; } }
    true
}

//command identifier and executer

pub fn exec(line: &[u8], has_fb: bool) {
    if str_eq(line, b"patpat") {
        crate::serial::ser_puts("command patpat executed :3\n");
        if has_fb { crate::fb::fb_puts("aweee cutee you want headpattssss :3\n"); }
    } else if line.len() == 0 {
        //this empty enter
    
    } else if str_eq(line, b"clear") {
        cmd_clear(has_fb);
    } else if str_eq(line, b"help") {
        cmd_help(has_fb);
    } else {
        crate::serial::ser_puts("ERR1 command not found :3?\n");
        if has_fb { crate::fb::fb_puts("typo for a command mayhaps :3?\n"); }
    }
}


//commands 

fn cmd_clear(has_fb: bool) { //easy clear thing as it was in my last project
    crate::serial::ser_puts("\ncommand clear executed\n");
    if has_fb { crate::fb::fb_clear(); }
}

fn cmd_help(has_fb: bool) { //lists our massive bank of commands :D
    crate::serial::ser_puts("command help executed\n");
    if has_fb { crate::fb::fb_puts("commands include:\n patpat, \n clear, \n help \n :3\n"); }
}