//! 0x8c21cf1c - 1 if bit 8 of the first word of slot `r4` is set, else 0.
//!
//! 0x8c2199ca(r4) returns the slot pointer. `tst` against the sign-extended
//! word 0x0100 decides the branch. The 1 is written in the `rts` delay slot,
//! so that path's r0 is 1 only after the return.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[3] = mem.read_u32(0x8c21d01c).unwrap_or(0x8c2199ca); // mov.l 0x8c21d01c,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // jsr delay: mov.l r4,@r15
    cpu.pr = 0x8c21cf26;
    crate::registry::p_8c2199ca::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[3] = mem.read_u16(0x8c21d01a).unwrap_or(0x0100) as i16 as i32 as u32; // mov.w
    cpu.set_t((cpu.r[3] & cpu.r[0]) == 0); // tst r3,r0
    let hit = !cpu.t();
    if hit {
        cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 1; // rts delay: mov #1,r0
    } else {
        cpu.r[0] = 0; // mov #0,r0
        cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
