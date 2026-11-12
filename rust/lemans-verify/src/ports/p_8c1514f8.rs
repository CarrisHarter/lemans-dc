//! 0x8c1514f8 - stamp a vtable word, then free the object when the count is positive.
//!
//! Null r4 exits after the prologue (T=1). Otherwise `[r14] = 0x8c1ef40c` in the
//! `bf.s` delay slot and, for a positive sign-extended low half of r5,
//! 0x8c01032c frees r4. `mov r14,r0` returns the argument.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[1] = mem.read_u32(0x8c15151c).unwrap_or(0x8c1ef40c); // mov.l 0x8c15151c,r1
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]); // bf.s delay: mov.l r1,@r14
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c151520).unwrap_or(0x8c01032c); // mov.l 0x8c151520,r0
            cpu.pr = 0x8c151512;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
