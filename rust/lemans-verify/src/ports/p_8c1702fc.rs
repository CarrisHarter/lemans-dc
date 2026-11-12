//! 0x8c1702fc — call 0x8c2206c0, return whether it returned zero.
//!
//! ```text
//! sts.l pr,@-r15
//! r3 = [0x8c170310]              ; 0x8c2206c0
//! jsr @r3                        ; pr = 0x8c170304
//! nop                            ; delay slot
//! lds.l @r15+,pr
//! tst r0,r0                      ; T = (r0 == 0)
//! rts
//! movt r0                        ; delay slot: r0 = T
//! ```
//!
//! Twin of 0x8c1702ec with a different callee. The callee always returns 0,
//! so this exits with r0 = 1 and T = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c170310).unwrap_or(0); // 0x8c2206c0
    cpu.pr = 0x8c170304; // jsr @r3
    // nop delay slot
    crate::registry::p_8c2206c0::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[0] = u32::from(cpu.t()); // movt r0 (rts delay slot)
}
