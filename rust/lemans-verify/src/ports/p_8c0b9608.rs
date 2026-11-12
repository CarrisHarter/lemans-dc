//! 0x8c0b9608 — guarded weight-accumulator entry.
//!
//! `r4` points at a one-byte enable flag; at `+4` it holds the entry-table
//! pointer handed to the weight accumulator (0x8c0b9784). The callee runs only
//! when both are non-zero; every other path leaves fr0 = 0.0.
//!
//! CFG:
//! ```text
//! entry: push pr
//!   r0 = extu.b(*r4) ; tst r0,r0 ; bt ZERO
//!   r4 = [r4+4]      ; tst r4,r4 ; bt ZERO
//!   r0 = literal(0x8c0b962c) = 0x8c0b9784
//!   jsr @r0 (pr = 0x8c0b9620, delay slot nop)
//!   bra RESTORE (delay slot nop)
//! ZERO 0x8c0b9624: fldi0 fr0
//! RESTORE 0x8c0b9626: lds.l @r15+,pr ; rts (delay slot nop)
//! ```
//!
//! The callee clobbers r0..r7, fr0..fr8 and fpul; r8..r14 and r15 come back
//! unchanged, so only the caller's push/pop and its two guards live here.
//! T ends as the last compare left it: 1 on both guard paths, the callee's
//! exit flag on the call path. The only store outside the stack is the
//! callee's weight write.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap() as u32; // mov.b @r4,r0 ; extu.b r0,r0 (x2)
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.fr[0] = 0.0; // fldi0 fr0 at 0x8c0b9624
    } else {
        cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // mov.l @(4,r4),r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] == 0 {
            cpu.fr[0] = 0.0; // fldi0 fr0 at 0x8c0b9624
        } else {
            cpu.r[0] = mem.read_u32(0x8c0b962c).unwrap_or(0); // literal 0x8c0b9784
            cpu.pr = 0x8c0b9620; // jsr @r0; delay slot nop
            crate::registry::p_8c0b9784::run(cpu, mem);
            // bra 0x8c0b9626; delay slot nop
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
