//! 0x8c0bdbd0 — construct a 116-byte object at r4: call the initialiser
//! 0x8c14b1f4 (loaded from the literal at 0x8c0bdbf0), then install the
//! vtable pointer 0x8c1eb328 at offset 0 and zero the field at offset 104.
//! Returns the object (r4). r4 is passed through untouched to the callee.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! mov.l 0x8c0bdbf0,r0 (0x8c14b1f4) ; jsr @r0 ; nop
//! mov.l 0x8c0bdbf4,r0 (0x8c1eb328) ; mov #0,r1 ; mov.l r0,@r14
//! mov #104,r0 ; mov.l r1,@(r0,r14) ; mov r14,r0
//! lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay)
//! ```
//!
//! No conditional branches: the loop-free body has a single path, witnessed by
//! the oracle's step count.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0bdbd0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0bdbd2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0bdbd4: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0bdbd6: mov.l 0x8c0bdbf0,r0
    cpu.r[0] = mem.read_u32(0x8c0bdbf0).unwrap_or(0); // 0x8c14b1f4
    // 0x8c0bdbd8: jsr @r0 ; 0x8c0bdbda: nop (delay slot)
    cpu.pr = 0x8c0bdbdc;
    crate::registry::p_8c14b1f4::run(cpu, mem);
    // 0x8c0bdbdc: mov.l 0x8c0bdbf4,r0
    cpu.r[0] = mem.read_u32(0x8c0bdbf4).unwrap_or(0); // 0x8c1eb328
    // 0x8c0bdbde: mov #0,r1
    cpu.r[1] = 0;
    // 0x8c0bdbe0: mov.l r0,@r14
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    // 0x8c0bdbe2: mov #104,r0
    cpu.r[0] = 104;
    // 0x8c0bdbe4: mov.l r1,@(r0,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    // 0x8c0bdbe6: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0bdbe8: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0bdbea: rts ; 0x8c0bdbec: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
