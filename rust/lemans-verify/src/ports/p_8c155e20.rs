//! 0x8c155e20 - clear a global slot, then free the object when the count is positive.
//!
//! Twin of 0x8c150818 with the counter at 0x8c2abd50. Null r4 exits after the
//! prologue (T=1). Otherwise `[0x8c2abd50]` is zeroed in the `bf.s` delay slot
//! and, for a positive sign-extended low half of r5, the object is freed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[3] = mem.read_u32(0x8c155e44).unwrap_or(0x8c2abd50); // mov.l 0x8c155e44,r3
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // bf.s delay: mov.l r1,@r3
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c155e48).unwrap_or(0x8c01032c); // mov.l 0x8c155e48,r0
            cpu.pr = 0x8c155e3c;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
