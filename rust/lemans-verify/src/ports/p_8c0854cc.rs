//! 0x8c0854cc — add a 4-short time-of-day vector in place, through 0x8c085684.
//!
//! `param_1` (r4) is saved to r14, a 4-short scratch buffer is carved out of
//! the stack (`r15` after `add #-8`) and its address is pushed as the callee's
//! third argument. The callee 0x8c085684 adds [dest] and [src] (r4/r5 pass
//! through untouched) with carry normalisation and writes the four halfwords to
//! the scratch; this function then copies the scratch into `*param_1` and
//! returns it. The port calls the ported callee, so the oracle's real bytes and
//! the port's callee must agree instruction for instruction.
//!
//! Side effects the C does not show: the `jsr` delay slot is a `nop`; after the
//! call r1 = the scratch address (entry r15 - 16) and stays there; the four
//! `mov.w` loads sign-extend before the 16-bit store; r0 ends as `param_1`, not
//! as the last loaded halfword; r3..r7 keep the callee's clobbers (r5 = 60,
//! r7 = 1000); T is whatever the callee's final `cmp/ge` left; r14 is restored
//! by the `rts` delay slot, pr by `lds.l`, and r15 is net unchanged.

use sh4_oracle::{Cpu, Memory};

/// Return address: the instruction after the `jsr` delay slot.
const JSR_RETURN_8C0854DE: u32 = 0x8c0854de;
/// PC-relative literal slot holding the callee entry (0x8c085684).
const CALLEE_LITERAL_8C0854FC: u32 = 0x8c0854fc;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dest = cpu.r[4];
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // add #-8,r15 (local_10..local_a: the 4-short scratch)
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // mov r4,r14
    cpu.r[14] = dest;
    // mov r15,r0 ; mov.l r0,@-r15 — push the scratch pointer as arg 3
    let scratch = cpu.r[15];
    cpu.r[0] = scratch;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], scratch);
    // mov.l 0x8c0854fc,r0 ; jsr @r0 ; nop
    cpu.r[0] = mem.read_u32(CALLEE_LITERAL_8C0854FC).unwrap_or(0);
    cpu.pr = JSR_RETURN_8C0854DE;
    crate::registry::p_8c085684::run(cpu, mem);
    // add #4,r15 — drop the pushed argument
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // mov r15,r1
    cpu.r[1] = cpu.r[15];
    // mov.w @r1,r0 ; mov.w r0,@r14 — mov.w loads sign-extend
    for off in [0u32, 2, 4, 6] {
        cpu.r[0] = mem.read_u16(cpu.r[1].wrapping_add(off)).unwrap_or(0) as i16 as i32 as u32;
        let _ = mem.write_u16(cpu.r[14].wrapping_add(off), cpu.r[0] as u16);
    }
    // mov r14,r0
    cpu.r[0] = cpu.r[14];
    // add #8,r15
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
