//! 0x8c150878 - clear a global, drain the sub-array, then free when positive.
//!
//! The first `tst r14,r14` short-circuits null r4 (T=1); the second, in the
//! `bt.s` fragment, is dead (r14 != 0 there) but its delay slot `[0x8c2abc88] = 0`
//! always runs. 0x8c1508cc keeps r4, then a positive exts.w(r13) frees r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[1] = mem.read_u32(0x8c1508ac).unwrap_or(0x8c2abc88); // mov.l 0x8c1508ac,r1
        cpu.r[0] = 0; // mov #0,r0
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14 (dead: r14 != 0)
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // bt.s delay: mov.l r0,@r1
        // bt not taken: fall through
        cpu.r[0] = mem.read_u32(0x8c1508b0).unwrap_or(0x8c1508cc); // mov.l 0x8c1508b0,r0
        cpu.pr = 0x8c150896;
        crate::registry::p_8c1508cc::run(cpu, mem);
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w r13,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1508b4).unwrap_or(0x8c01032c); // mov.l 0x8c1508b4,r0
            cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
            cpu.pr = 0x8c1508a2;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
