//! 0x8c0bee14 — advance the time-of-day state machine one step.
//!
//! `param` (r4) points at a record whose word at +0x1c is the machine state.
//! When that word is nonzero the routine calls 0x8c085148(state, fr4 = 0),
//! the 1->2 transition; then it copies the four int16 stack arguments of its
//! own caller (at entry r15, +0, +2, +4, +6) down into an 8-byte scratch at
//! r15-16 and calls 0x8c0850a4(state, fr4 = 0) with that scratch as its stack
//! arguments — that routine stores the f32 elapsed value at param+0x20 and
//! forces the state word to 1; finally it calls 0x8c085110(state, fr4 = 0),
//! which therefore always sees state 1 and takes its skip arm.
//!
//! Side effects the C does not show: r0 ends 1 (the state 0x8c085110 loads),
//! r1 = param+0x20 (0x8c0850a4's `r4+4`), r3 = sign-extended ms (the last
//! halfword loaded), r4 = param+0x1c, r5 = sign-extended s, r6 = sign-extended
//! m, T = 0 (0x8c085110's `cmp/eq #2` against the state 1 it just read);
//! fr0..fr8 are whatever 0x8c0850a4 left (fr4 = 0 from the last `fldi0`),
//! fpul = m. r14 and pr are restored from the stack by the epilogue and r15 is
//! net unchanged. The scratch halfword stores happen in instruction order
//! (r15-16, -14, -12 in the body, -10 in the jsr delay slot) and only the
//! halfword loads sign-extend before the 16-bit store.

use sh4_oracle::{Cpu, Memory};

/// PC-relative literal slots in this function's own pool (0x8c0bee5c..-64).
const LIT_8C085148: u32 = 0x8c0bee5c;
const LIT_8C0850A4: u32 = 0x8c0bee60;
const LIT_8C085110: u32 = 0x8c0bee64;

/// Return address of each `jsr`: the instruction after its delay slot.
const RET_8C085148: u32 = 0x8c0bee28;
const RET_8C0850A4: u32 = 0x8c0bee48;
const RET_8C085110: u32 = 0x8c0bee54;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov r4,r14
    cpu.r[14] = cpu.r[4];
    // mov.l @(28,r14),r0 ; tst r0,r0 ; bt 0x8c0bee28
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(28)).unwrap_or(0);
    if cpu.r[0] != 0 {
        // mov.l 0x8c0bee5c,r0 ; add #28,r4 ; jsr @r0 ; fldi0 fr4
        cpu.r[0] = mem.read_u32(LIT_8C085148).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(28);
        cpu.pr = RET_8C085148;
        cpu.fr[4] = 0.0;
        crate::registry::p_8c085148::run(cpu, mem);
    }
    // mov r15,r3 ; add #8,r3 ; add #-8,r15
    cpu.r[3] = cpu.r[15].wrapping_add(8);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // mov.w @r3,r0 ; mov r14,r4 ; add #28,r4 ; mov.w r0,@r15
    cpu.r[0] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[4] = cpu.r[14].wrapping_add(28);
    let _ = mem.write_u16(cpu.r[15], cpu.r[0] as u16);
    // fldi0 fr4
    cpu.fr[4] = 0.0;
    // mov.w @(2,r3),r0 ; mov.l 0x8c0bee60,r1 ; mov.w r0,@(2,r15)
    cpu.r[0] = mem.read_u16(cpu.r[3].wrapping_add(2)).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[1] = mem.read_u32(LIT_8C0850A4).unwrap_or(0);
    let _ = mem.write_u16(cpu.r[15].wrapping_add(2), cpu.r[0] as u16);
    // mov.w @(4,r3),r0 ; mov.w r0,@(4,r15)
    cpu.r[0] = mem.read_u16(cpu.r[3].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32;
    let _ = mem.write_u16(cpu.r[15].wrapping_add(4), cpu.r[0] as u16);
    // mov.w @(6,r3),r0 ; jsr @r1 ; mov.w r0,@(6,r15)
    cpu.r[0] = mem.read_u16(cpu.r[3].wrapping_add(6)).unwrap_or(0) as i16 as i32 as u32;
    cpu.pr = RET_8C0850A4;
    let _ = mem.write_u16(cpu.r[15].wrapping_add(6), cpu.r[0] as u16);
    crate::registry::p_8c0850a4::run(cpu, mem);
    // add #8,r15
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    // add #28,r14
    cpu.r[14] = cpu.r[14].wrapping_add(28);
    // mov.l 0x8c0bee64,r0 ; mov r14,r4 ; jsr @r0 ; fldi0 fr4
    cpu.r[0] = mem.read_u32(LIT_8C085110).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.pr = RET_8C085110;
    cpu.fr[4] = 0.0;
    crate::registry::p_8c085110::run(cpu, mem);
    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
