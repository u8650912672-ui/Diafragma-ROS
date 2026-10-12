//user stub so ring3 has a place to live 

#[repr(align(4096))] // page aligned so 2 marked pages cover it :3
struct AlignedUStack(pub [u8; 8192]);

static mut USTACK: AlignedUStack = AlignedUStack([0; 8192]); //ring3 stack mapped u next piece

pub fn user_entry() { //called via sysret later for now just POC
    crate::serial::ser_puts("haii from ring3 stub (still CPL0, no sysret yet tho) :3\n");    
}
pub fn user_stack_fill() { //poke msg into ustack before entry so stub has a valid u ptr to use :3?
    unsafe {
        let b = b"haii ring3 :)\n";
        for i in 0..b.len() { core::ptr::addr_of_mut!(USTACK).cast::<u8>().add(i).write_volatile(b[i]); }
    }
}
#[link_section = ".usertext"] //own page so marking U dont give out the fucking kernel .text (security :D)
#[unsafe(naked)]
pub extern "C" fn user_stub() -> ! {
    core::arch::naked_asm!(
        "mov rax, 1", "syscall", //N write(rdi = base rsi = len and set by ring3 enter)
        "mov rax, 2", "syscall", //n exit dies :3
        "2: hlt", "jmp 2b",
    );
}
pub fn user_stack_range() -> (u64, u64) {
    unsafe { (core::ptr::addr_of!(USTACK) as u64, 8192) }
}
pub fn user_code_range() -> (u64, u64) {
    (user_stub as *const () as u64, 4096)  // own page not kernel .text :3 
}

pub fn ring3_enter() { // theater sysret still s only pages so we skip isolation for now
    unsafe {
        let ustack_top = core::ptr::addr_of!(USTACK) as u64 + 8192;
        let entry = user_stub as *const () as u64;
        let base = core::ptr::addr_of!(USTACK) as u64;
        // RCX=rip r11=rflags rsp = user top cs=0x23/ss=0x1B from star
        core::arch::asm!(
           "cli
            mov rdi, {2}
            mov rsi, 14
            mov rcx, {0}
            mov r11, 0x202
            push 0x1B
            push {1}
            push 0x202
            push 0x23
            push {0}
            iretq",
            in(reg) entry, in(reg) ustack_top, in(reg) base,
            options(nostack),
        );
    }
}