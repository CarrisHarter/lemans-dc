//! 0x8c221e8e — memcpy(0x8c2bde98 + 108*r4, ...) with an 8-byte length.
//!
//! `r4 <<= 7` (four `shll2` plus one `shll`; the final `shll` sets T from bit
//! 24 of the index), `r4 += 0x8c2bde98`, the value is spilled to the scratch
//! word, `r4 += 108`, `r6 = 8`, then `jmp` to memcpy with the stack restored in
//! the delay slot. The source pointer is whatever the caller left in r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let mut r4 = cpu.r[4];
    r4 <<= 2; // shll2 r4
    r4 <<= 2; // shll2 r4
    r4 <<= 2; // shll2 r4
    cpu.set_t(r4 & 0x8000_0000 != 0); // shll r4 sets T from bit 31
    r4 <<= 1;
    cpu.r[3] = mem.read_u32(0x8c221ea8).unwrap_or(0x8c2bde98); // mov.l 0x8c221ea8,r3
    let dst = r4.wrapping_add(cpu.r[3]); // add r3,r4
    cpu.r[4] = dst;
    let _ = mem.write_u32(cpu.r[15], dst); // mov.l r4,@r15
    cpu.r[4] = dst.wrapping_add(108); // add #108,r4
    cpu.r[6] = 8; // mov #8,r6
    cpu.r[2] = mem.read_u32(0x8c221eac).unwrap_or(0x8c010e84); // mov.l 0x8c221eac,r2
    cpu.r[15] = cpu.r[15].wrapping_add(4); // delay slot: add #4,r15
    crate::registry::p_8c010e84::run(cpu, mem); // jmp @r2
}
