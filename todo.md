# STUFF TO DO


### race condition warning for myself
INVARIANT: No interrupt handler may touch KWRITE_BUF (or any syscall-local
static buffer). Currently enforced by: only the keyboard IRQ (33) has a
real handler, and it doesn't touch syscall state.
When this invariant is broken (timer IRQ, SMP, or a handler that wants to
log something), MOVE THE BUFFER to per-CPU or per-task storage.
This is not paranoia. This is the exact bug Linux had with per-CPU data
in the early days.

what should i do idfk? gimme ideas i guess?





### DONE STUFF

set a real ring0 stack in gdt.rs bz rigth now its 0