//! 0x8c237680 — copy six words out of a table hit found by 0x8c237780.
//!
//! The callee linear-searches the fixed-stride table at `[0x8c2377c4]` for
//! `r4` and returns 1 on a hit, 0 otherwise. On a miss the routine returns the
//! sentinel 0xeeee (held in r13) without touching `r5`. On a hit it reloads
//! `r4` from its stack frame and, when `r5` is non-null, copies six words from
//! `r4+24, +28, +44, +48, +64, +68` into `r5[0..6]` and returns 0. A null
//! `r5` still returns 0xeeee; the `bt.s` delay slot has already bumped `r4` by
//! 24 in that case.
//!
//! The callee is invoked with the original return address in `pr` and the
//! delay-slot assignment `r14 = r5` already applied. Its port leaves `mach`
//! alone where the original `mul.l` clears it, so specs seed `mach = 0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15 ; add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);

    cpu.r[3] = 0x8c237780; // mov.l 0x8c2376e4,r3
    cpu.r[13] = 0x0000eeee; // mov.l 0x8c2376e0,r13
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[14] = cpu.r[5]; // jsr delay slot: mov r5,r14
    cpu.pr = 0x8c237692;
    crate::registry::p_8c237780::run(cpu, mem);

    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
        cpu.r[4] = cpu.r[4].wrapping_add(24); // bt.s delay slot: add #24,r4
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14
        if !cpu.t() {
            cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r2
            cpu.r[13] = 0; // mov #0,r13
            let _ = mem.write_u32(cpu.r[14], cpu.r[2]); // mov.l r2,@r14
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r3
            let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]); // mov.l r3,@(4,r14)
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0); // mov.l @(20,r4),r2
            let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[2]); // mov.l r2,@(8,r14)
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0); // mov.l @(24,r4),r3
            let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[3]); // mov.l r3,@(12,r14)
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0); // mov.l @(40,r4),r2
            let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[2]); // mov.l r2,@(16,r14)
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(44)).unwrap_or(0); // mov.l @(44,r4),r3
            let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[3]); // mov.l r3,@(20,r14)
        }
    }

    // Epilogue: mov r13,r0 ; add #4,r15 ; lds.l @r15+,pr ;
    //           mov.l @r15+,r13 ; rts delay: mov.l @r15+,r14
    cpu.r[0] = cpu.r[13];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
