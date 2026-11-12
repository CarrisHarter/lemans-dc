//! 0x8c2205d0 — clear word 28 of each record in a 4-entry array, then a global.
//!
//! `r13 = 0x00d0` (word load at 0x8c220604) is the array's byte size and
//! `r12 = 0x8c23d468` holds the array pointer. Each iteration reloads that base,
//! adds 4 and the walking offset `r14`, and calls 0x8c2205b0 with the record's id
//! slot: the callee clears `base[*id] + 28`, where `base = *(0x8c23d468)`.
//! After the fourth call (`r14 = 0xd0`, `cmp/hs` sets T) it clears the global at
//! 0x8c2bde5c and returns 0. The prologue pushes r14/r13/r12/r11/pr and the
//! epilogue pops them, so those four registers and pr end at their entry values.
//! The last flag-setting instruction is `cmp/hs 0xd0,0xd0`, so T ends set.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov #0,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = 0;
    // mov.l r13,@-r15 ; mov.l r12,@-r15 ; mov.l r11,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov.w 0x8c220604,r13 (0x00d0, sign-extended word)
    cpu.r[13] = mem.read_u16(0x8c220604).unwrap_or(0) as i16 as i32 as u32;
    // mov.l 0x8c22060c,r12 (0x8c23d468) ; mov.l 0x8c220608,r11 (0x8c2205b0)
    cpu.r[12] = mem.read_u32(0x8c22060c).unwrap_or(0);
    cpu.r[11] = mem.read_u32(0x8c220608).unwrap_or(0);
    loop {
        cpu.r[4] = mem.read_u32(cpu.r[12]).unwrap_or(0); // mov.l @r12,r4
        cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[14]); // delay slot of `jsr @r11`
        cpu.pr = 0x8c2205ea; // jsr @r11
        crate::registry::p_8c2205b0::run(cpu, mem);
        cpu.r[14] = cpu.r[14].wrapping_add(52); // add #52,r14
        cpu.set_t(cpu.r[14] >= cpu.r[13]); // cmp/hs r13,r14
        if cpu.r[14] < cpu.r[13] {
            continue; // bf 0x8c2205e2
        }
        break;
    }
    // mov.l 0x8c220610,r3 (0x8c2bde5c) ; mov #0,r2 ; mov r2,r0 ; mov.l r2,@r3
    cpu.r[3] = mem.read_u32(0x8c220610).unwrap_or(0);
    cpu.r[2] = 0;
    cpu.r[0] = cpu.r[2];
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    // lds.l @r15+,pr ; mov.l @r15+,r11 ; mov.l @r15+,r12 ; mov.l @r15+,r13
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
