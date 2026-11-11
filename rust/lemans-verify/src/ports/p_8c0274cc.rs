//! 0x8c0274cc — `return table_base + index` as a byte: `DAT_8c0274d4[param_1]`.
//!
//! `mov.l 0x8c0274d4,r0 ; mov.b @(r0,r4),r0 ; rts ; extu.b r0,r0`
//! The C hides that r0 is first the table base (0x170) and that the
//! `extu.b` delay slot is what actually zero-extends the sign-extended
//! `mov.b` load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let table = mem.read_u32(0x8c0274d4).unwrap(); // 0x170
    let addr = table.wrapping_add(cpu.r[4]);
    cpu.r[0] = mem.read_u8(addr).unwrap() as u32; // mov.b @(r0,r4),r0 (sign-extends)
    cpu.r[0] &= 0xff; // extu.b r0,r0 — delay slot of `rts`
}
