//user stub so ring3 has a place to live 

#[repr(align(16))]
struct AlignedUStack(pub [u8; 8192]);

static mut USTACK: AlignedUStack = AlignedUStack([0; 8192]); //ring3 stack mapped u next piece

pub fn user_entry() { //called via sysret later for now just POC
    crate::serial::ser_puts("haii from ring3 stub (still CPL0, no sysret yet tho) :3\n");    
}
pub fn user_stack_range() -> (u64, u64) {
    unsafe { (core::ptr::addr_of!(USTACK) as u64, 8192) }
}
pub fn user_code_range() -> (u64, u64) {
    (user_entry as u64, 4096) // single page 
}

pub fn ring3_enter() { // theater sysret still s only pages so we skip isolation for now
    unsafe {
        let ustack_top = core::ptr::addr_of!(USTACK) as u64 + 8192;
        let entry = user_entry as u64;
        // RCX=rip r11=rflags rsp = user top cs=0x23/ss=0x1B from star
        core::arch::asm!(
           "mov rcx, {0}
            mov r11, 0x202
            mov rsp, {1}
            push 0x1B
            push {1}
            push 0x202
            push 0x23
            push {0}
            iretq",
            in(reg) entry, in(reg) ustack_top,
            options(nostack),
        );
    }
}