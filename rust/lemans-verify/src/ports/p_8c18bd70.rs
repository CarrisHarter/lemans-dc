//! 0x8c18bd70 — copy-assign a 20-byte record.
//!
//! `r4` is the destination record, `r5` the source. Self-assignment returns the
//! destination without touching memory. Otherwise the routine first hands the
//! two `+12` fields to 0x8c10ef64 (a bounded string copy plus NUL terminator),
//! then copies `+8`, the four halfwords at `+0..+6`, and `+16` from source to
//! destination.
//!
//! The field order is observable: when the records overlap, a store can change
//! a later load, so every field is re-read from `r14` at its own instruction.
//! `mov.w` loads sign-extend into `r0`; the stores keep the low half. The
//! routine ends with `r0 = r13`, the flag state the callee left (T=1 on both of
//! its paths), and the pushed `r13`/`r14` restored to their entry values.

use sh4_oracle::{Cpu, Memory};

/// `mov.w @(disp,r14),r0`: sign-extended halfword load.
fn ldsw(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // mov.l r13,@-r15
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    let t = cpu.r[13] == cpu.r[14]; // cmp/eq r14,r13
    cpu.set_t(t);
    if t {
        cpu.r[0] = cpu.r[13]; // delay slot of `bra`
    } else {
        cpu.r[5] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r14),r5
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0); // delay slot of `jsr`
        crate::registry::p_8c10ef64::run(cpu, mem);
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r14),r0
        let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]); // mov.l r0,@(8,r13)
        cpu.r[0] = ldsw(mem, cpu.r[14]); // mov.w @r14,r0
        let _ = mem.write_u16(cpu.r[13], cpu.r[0] as u16); // mov.w r0,@r13
        cpu.r[0] = ldsw(mem, cpu.r[14].wrapping_add(2)); // mov.w @(2,r14),r0
        let _ = mem.write_u16(cpu.r[13].wrapping_add(2), cpu.r[0] as u16); // mov.w r0,@(2,r13)
        cpu.r[0] = ldsw(mem, cpu.r[14].wrapping_add(4)); // mov.w @(4,r14),r0
        let _ = mem.write_u16(cpu.r[13].wrapping_add(4), cpu.r[0] as u16); // mov.w r0,@(4,r13)
        cpu.r[0] = ldsw(mem, cpu.r[14].wrapping_add(6)); // mov.w @(6,r14),r0
        let _ = mem.write_u16(cpu.r[13].wrapping_add(6), cpu.r[0] as u16); // mov.w r0,@(6,r13)
        cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0); // mov.l @(16,r14),r1
        cpu.r[0] = cpu.r[13]; // mov r13,r0
        let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[1]); // mov.l r1,@(16,r13)
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
