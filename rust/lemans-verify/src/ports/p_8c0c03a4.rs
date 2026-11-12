//! 0x8c0c03a4 — release a two-pointer list header through the tree destructor.
//!
//! `r4` points at a four-word header. If word 1 is non-null, the routine passes
//! it in `r5` to 0x8c0c0614 (the tree release; literal pool word at 0x8c0c03d0),
//! then clears words 0 and 1 and writes `&header[1]` into word 3. The prologue
//! pushes r14/pr and the epilogue restores both, so r14 and pr end at their
//! entry values and r15 returns to entry. `add #4` does not set T, so T keeps
//! the `tst` result on the early-exit path and the callee's residue on the free
//! path; the callee owns r0-r7 on that path until the stores below set r0 to
//! `r14 + 4` and r1 to zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0c03a4  mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0c03a6  sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0c03a8  mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0c03aa  mov.l @(4,r14),r0
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    // 0x8c0c03ac  tst r0,r0
    let t = cpu.r[0] == 0;
    cpu.set_t(t);
    if !t {
        // 0x8c0c03b0  mov r14,r0
        cpu.r[0] = cpu.r[14];
        // 0x8c0c03b2  add #4,r0
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        // 0x8c0c03b4  mov.l @r0,r5
        cpu.r[5] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c0c03b6  mov.l 0x8c0c03d0,r0  (0x8c0c0614)
        cpu.r[0] = mem.read_u32(0x8c0c03d0).unwrap_or(0);
        // 0x8c0c03b8  jsr @r0 ; delay slot 0x8c0c03ba nop
        cpu.pr = 0x8c0c03bc;
        crate::registry::p_8c0c0614::run(cpu, mem);
        // 0x8c0c03bc  mov #0,r1
        cpu.r[1] = 0;
        // 0x8c0c03be  mov.l r1,@r14
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
        // 0x8c0c03c0  mov r14,r0
        cpu.r[0] = cpu.r[14];
        // 0x8c0c03c2  mov.l r1,@(4,r14)
        let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
        // 0x8c0c03c4  add #4,r0
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        // 0x8c0c03c6  mov.l r0,@(12,r14)
        let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[0]);
    }
    // 0x8c0c03c8  lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c03ca  rts ; delay slot 0x8c0c03cc mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
