//! 0x8c0b26a4 — initialise a `vlSceneEnvironment` object (constructor).
//!
//! `r4` is the object; the routine returns it in `r0`. It writes:
//! - `[r4]` = the vtable pointer stored at 0x8c0b2700;
//! - `r4+4..20` = four words copied from the global table at 0x8c1d0f28;
//! - `r4+20..32` = the three floats at 0x8c1d3f50 negated. The negation is
//!   done by 0x8c0b4db4, which takes its source in `r4` and its destination
//!   pointer from the stack slot pushed here; the caller then copies the
//!   temporary back as words;
//! - `r4+32` and `r4+36` = the `0.5f` literal at 0x8c0b2710 (two `fmov`s).
//!
//! The single call is the only control flow: `pr` is set to the return
//! address 0x8c0b26bc (the instruction after the `jsr` delay slot), and the
//! stack temporary is live at `[sp]` when the callee starts.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // add #-12,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    // mov r4,r14
    cpu.r[14] = cpu.r[4];
    // mov.l @(0x8c0b2700),r0 ; mov.l r0,@r14
    cpu.r[0] = mem.read_u32(0x8c0b2700).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    // mov r15,r0 ; mov.l r0,@-r15   (destination pointer for the callee)
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // mov.l @(0x8c0b2704),r4 ; mov.l @(0x8c0b2708),r0
    cpu.r[4] = mem.read_u32(0x8c0b2704).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c0b2708).unwrap_or(0);
    // jsr @r0 ; nop
    cpu.pr = 0x8c0b26bc;
    crate::registry::p_8c0b4db4::run(cpu, mem);
    // add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // mov r15,r4 ; mov r14,r5 ; add #20,r5
    cpu.r[4] = cpu.r[15];
    cpu.r[5] = cpu.r[14];
    // mov.l @r4,r1
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(20);
    // mov.l @(4,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    // mov r14,r7 ; mov r14,r3 ; mov.l r1,@r5
    cpu.r[7] = cpu.r[14];
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
    // add #4,r7
    cpu.r[7] = cpu.r[7].wrapping_add(4);
    // mov.l @(0x8c0b270c),r6
    cpu.r[6] = mem.read_u32(0x8c0b270c).unwrap_or(0);
    // mov r14,r1 ; add #32,r3
    cpu.r[1] = cpu.r[14];
    cpu.r[3] = cpu.r[3].wrapping_add(32);
    // mov.l r0,@(4,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]);
    // add #36,r1
    cpu.r[1] = cpu.r[1].wrapping_add(36);
    // mov.l @(8,r4),r4
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // mova 0x8c0b2710,r0
    cpu.r[0] = 0x8c0b2710;
    // mov.l r4,@(8,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[4]);
    // mov.l @r6,r5 ; mov.l @(4,r6),r4
    cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
    // mov.l r5,@r7 ; mov.l r4,@(4,r7)
    let _ = mem.write_u32(cpu.r[7], cpu.r[5]);
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[4]);
    // mov.l @(8,r6),r5 ; mov.l @(12,r6),r4
    cpu.r[5] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap_or(0);
    // mov.l r5,@(8,r7) ; mov.l r4,@(12,r7)
    let _ = mem.write_u32(cpu.r[7].wrapping_add(8), cpu.r[5]);
    let _ = mem.write_u32(cpu.r[7].wrapping_add(12), cpu.r[4]);
    // fmov @r0,fr0 ; fmov fr0,@r3
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    // mov r14,r0 ; fmov fr0,@r1
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    // add #12,r15 ; lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
