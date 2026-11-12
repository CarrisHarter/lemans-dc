//! 0x8c0b2ee4 — fill a 3x4 row-major float matrix at `r4`: rows 0..2 are the
//! identity, then `0x8c0b4ff0(r4 + 36)` clears the translation column.
//!
//! The decompiler shows a bogus `FPSCR.SZ` conditional; the original bytes are
//! a straight-line build of the stack frame before the `jsr`. Full listing:
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; add #-36,r15
//! fldi1 fr1 ; mov #24,r0 ; fmov fr1,@(r0,r15)
//! fldi0 fr0 ; mov #28,r0 ; fmov fr0,@(r0,r15)
//! mov r15,r3 ; mov #32,r0 ; fmov fr0,@(r0,r15) ; add #24,r3
//! mov.l @r3,r1 ; mov #12,r2 ; mov.l @(4,r3),r0
//! mov r15,r7 ; mov r4,r14 ; mov.l r1,@r4 ; mov r4,r6 ; mov.l r0,@(4,r4)
//! add r15,r2 ; mov.l @(8,r3),r1 ; add #12,r7
//! mov.l 0x8c0b2f64,r0 (0x8c0b4ff0) ; add #12,r14 ; mov r15,r5
//! mov.l r1,@(8,r4) ; add #24,r6 ; fmov fr0,@r2 ; add #36,r4
//! mov #16,r2 ; add r15,r2 ; fmov fr1,@r2
//! mov #20,r2 ; add r15,r2 ; fmov fr0,@r2
//! mov.l @r7,r3 ; mov #0,r2 ; mov.l @(4,r7),r1 ; add r15,r2
//! mov.l r3,@r14 ; mov.l r1,@(4,r14) ; mov.l @(8,r7),r1 ; mov.l r1,@(8,r14)
//! fmov fr0,@r2 ; mov #4,r2 ; add r15,r2 ; fmov fr0,@r2
//! mov #8,r2 ; add r15,r2 ; fmov fr1,@r2
//! mov.l @r5,r3 ; mov.l @(4,r5),r1 ; mov.l r3,@r6 ; mov.l r1,@(4,r6)
//! mov.l @(8,r5),r1 ; jsr @r0 ; mov.l r1,@(8,r6)
//! add #36,r15 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! Stores, in order: `r4+0 = 1.0f`, `+4 = 0`, `+8 = 0`, `+12 = 0`,
//! `+16 = 1.0f`, `+20 = 0`, `+24 = 0`, `+28 = 0`, `+32 = 1.0f` (the last one in
//! the `jsr` delay slot), then the callee clears `+36`, `+40`, `+44`.
//! Leftovers: r0=arg+40, r1=arg+44, r2=r15-36, r3=0, r4=arg+36, r5=r15-44,
//! r6=arg+24, r7=r15-32, fr0=0.0, fr1=1.0; r14/r15/pr are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15 ; add #-36,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(36);

    // fldi1 fr1 ; mov #24,r0 ; fmov fr1,@(r0,r15)
    cpu.fr[1] = 1.0;
    cpu.r[0] = 24;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[1]);
    // fldi0 fr0 ; mov #28,r0 ; fmov fr0,@(r0,r15)
    cpu.fr[0] = 0.0;
    cpu.r[0] = 28;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);
    // mov r15,r3 ; mov #32,r0 ; fmov fr0,@(r0,r15) ; add #24,r3
    cpu.r[3] = cpu.r[15];
    cpu.r[0] = 32;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(24);

    // mov.l @r3,r1 ; mov #12,r2 ; mov.l @(4,r3),r0
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] = 12;
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
    // mov r15,r7 ; mov r4,r14 ; mov.l r1,@r4
    cpu.r[7] = cpu.r[15];
    cpu.r[14] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    // mov r4,r6 ; mov.l r0,@(4,r4) ; add r15,r2
    cpu.r[6] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // mov.l @(8,r3),r1 ; add #12,r7
    cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
    cpu.r[7] = cpu.r[7].wrapping_add(12);
    // mov.l 0x8c0b2f64,r0 ; add #12,r14 ; mov r15,r5
    cpu.r[0] = 0x8c0b4ff0;
    cpu.r[14] = cpu.r[14].wrapping_add(12);
    cpu.r[5] = cpu.r[15];
    // mov.l r1,@(8,r4) ; add #24,r6 ; fmov fr0,@r2 ; add #36,r4
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[1]);
    cpu.r[6] = cpu.r[6].wrapping_add(24);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[4] = cpu.r[4].wrapping_add(36);
    // mov #16,r2 ; add r15,r2 ; fmov fr1,@r2
    cpu.r[2] = 16;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);
    // mov #20,r2 ; add r15,r2 ; fmov fr0,@r2
    cpu.r[2] = 20;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);

    // mov.l @r7,r3 ; mov #0,r2 ; mov.l @(4,r7),r1 ; add r15,r2
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[2] = 0;
    cpu.r[1] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // mov.l r3,@r14 ; mov.l r1,@(4,r14) ; mov.l @(8,r7),r1 ; mov.l r1,@(8,r14)
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]);
    // fmov fr0,@r2 ; mov #4,r2 ; add r15,r2 ; fmov fr0,@r2
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[2] = 4;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    // mov #8,r2 ; add r15,r2 ; fmov fr1,@r2
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);

    // mov.l @r5,r3 ; mov.l @(4,r5),r1 ; mov.l r3,@r6 ; mov.l r1,@(4,r6)
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[1]);
    // mov.l @(8,r5),r1 ; jsr @r0
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.pr = 0x8c0b2f5a; // return to the epilogue
    // delay slot: mov.l r1,@(8,r6)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(8), cpu.r[1]);
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    // add #36,r15 ; lds.l @r15+,pr ; rts ; delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(36);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
