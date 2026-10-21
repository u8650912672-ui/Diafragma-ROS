//user stub so ring3 has a place to live 
#[link_section = ".usertext"]
#[unsafe(naked)]
pub extern "C" fn sys3(num: u64, a: u64, b: u64) -> u64 { //syscallin from ring3
    core::arch::naked_asm!(
        "mov rax, rdi",
        "mov rdi, rsi",
        "mov rsi, rdx",
        "syscall",
        "ret",
    );
}

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
#[link_section = ".usertext"]
pub extern "C" fn shell_main() -> ! { //ring 3 shell edit line kernel now runs cmds
    unsafe {
        let base = core::ptr::addr_of!(USTACK) as *mut u8; //line buf is at 0 bs byte is 201
        let bs_ptr = base.add(201);
        bs_ptr.write(8);
        let mut n: usize = 0;
        loop {
            let c = sys3(0, 0, 0); //N_READ_KEY :D
            if c == u64::MAX - 1 { continue; } //you spin me rigth round baby till ya time the fuck outtttt
            if c == 0x1B { sys3(2, 0, 0,); } //N_exit
            let ch = c as u8;
            if ch == b'\n' {
                sys3(4, base as u64, n as u64); // thats N_CMD
                n = 0;
            } else if ch == 8 {
                if n > 0 { n -= 1; sys3(1, bs_ptr as u64, 1); }
            } else {
                if n < 127 {
                    base.add(n).write(ch);
                    n += 1;
                    sys3(1, base.add(n - 1) as u64, 1); // echo it
                }
            }
        }
    }
}


pub fn user_stack_range() -> (u64, u64) {
    unsafe { (core::ptr::addr_of!(USTACK) as u64, 8192) }
}
pub fn user_code_range() -> (u64, u64) {
    (shell_main as *const () as u64, 4096)  // own page not kernel .text :3 
}

#[repr(align(4096))]
struct AlignedUData(pub [u8; 4096]);
#[link_section = ".userdata"]
static mut UDATA: AlignedUData = AlignedUData([0; 4096]);

pub fn user_data_range() -> (u64, u64) {
    unsafe { (core::ptr::addr_of!(UDATA) as u64, 4096) }
}

pub fn ring3_enter() { // theater sysret still s only pages so we skip isolation for now
    unsafe {
        let ustack_top = core::ptr::addr_of!(USTACK) as u64 + 8192;
        let entry = shell_main as *const () as u64;
        // RCX=rip r11=rflags rsp = user top cs=0x23/ss=0x1B from star NERD STUFFFFF
        //even more deleted here that ACTUALLY AI HELPED WITH HOLY SHIT took me a while and said fuck it 
        core::arch::asm!(
           "cli
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