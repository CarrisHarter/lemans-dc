//! 0x8c21caf6 - fill a two-word result from a record, or zero it.
//!
//! If bit 0 of `[r5]` is set and 0x8c219cce(sp, r5) returns negative, both
//! words at `r4` are cleared. Otherwise 0x8c219fb6(5) is consulted: a result
//! of 1 transforms the saved `r4` through 0x8c21cad4; any other result copies
//! the word at `sp+0` through. `[r4] = [r4+4]`, `[r4+36] = r5+20`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r0
    cpu.set_t((cpu.r[0] & 1) == 0); // tst #1,r0
    cpu.r[14] = cpu.r[4]; // bt.s delay: mov r4,r14
    let bit0 = !cpu.t();
    let mut early = false;
    if bit0 {
        cpu.r[2] = mem.read_u32(0x8c21cb98).unwrap_or(0x8c219cce); // mov.l 0x8c21cb98,r2
        cpu.r[5] = cpu.r[13]; // mov r13,r5
        cpu.r[4] = cpu.r[15]; // jsr delay: mov r15,r4
        cpu.pr = 0x8c21cb10;
        crate::registry::p_8c219cce::run(cpu, mem);
        cpu.set_t((cpu.r[0] as i32) >= 0); // cmp/pz r0
        if !cpu.t() {
            cpu.r[4] = 0; // mov #0,r4
            let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[4]); // mov.l r4,@(4,r14)
            let _ = mem.write_u32(cpu.r[14], cpu.r[4]); // bra delay: mov.l r4,@r14
            early = true;
        }
    }
    if !early {
        cpu.r[2] = mem.read_u32(0x8c21cb9c).unwrap_or(0x8c219fb6); // mov.l 0x8c21cb9c,r2
        cpu.r[4] = 5; // jsr delay: mov #5,r4
        cpu.pr = 0x8c21cb22;
        crate::registry::p_8c219fb6::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.t() {
            cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bsr delay: mov.l @r15,r4
            cpu.pr = 0x8c21cb2a;
            crate::registry::p_8c21cad4::run(cpu, mem);
            let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]); // bra delay
        } else {
            cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
            let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]); // mov.l r3,@(4,r14)
        }
        cpu.r[3] = cpu.r[13]; // mov r13,r3
        cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r2
        cpu.r[3] = cpu.r[3].wrapping_add(20); // add #20,r3
        let _ = mem.write_u32(cpu.r[14], cpu.r[2]); // mov.l r2,@r14
        let _ = mem.write_u32(cpu.r[14].wrapping_add(36), cpu.r[3]); // mov.l r3,@(36,r14)
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
