//! 0x8c1cc5dc - clear a global slot, then free when the count is positive.
//!
//! Null r14 exits with T=1. Otherwise `[0x8c2b8728] = 0` lands in the `bf.s`
//! delay slot on both paths, and a positive `exts.w(r5)` frees r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[3] = mem.read_u32(0x8c1cc600).unwrap_or(0x8c2b8728); // mov.l 0x8c1cc600,r3
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // bf.s delay: mov.l r1,@r3
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1cc604).unwrap_or(0x8c01032c); // mov.l 0x8c1cc604,r0
            cpu.pr = 0x8c1cc5f8; // jsr delay is nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
