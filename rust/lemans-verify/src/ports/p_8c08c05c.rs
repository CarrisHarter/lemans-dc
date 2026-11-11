//! 0x8c08c05c — `*(float*)(r4+12) = 0.0; return r4;`
//!
//! The C's FPSCR_SZ branch is fiction: the code unconditionally executes
//! `fldi0 fr0 ; fmov fr0,@r0`. r0 = r4 comes from the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(12); // mov r4,r0 ; add #12,r0
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
