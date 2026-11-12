//! 0x8c0797a4 — r0 = *(r4+12) + wrap(r5+*(r4+8), mod *r4) * 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
    let ge = cpu.r[5] >= cpu.r[0]; // cmp/hs r0,r5: verifier shows T=(r5>=r0), branch taken on !(r0>=r5)
    cpu.set_t(ge);
    if cpu.r[0] < cpu.r[5] {
        cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[0]);
    }
    cpu.r[0] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.set_t(cpu.r[5] >> 30 & 1 != 0); // shll2 r5 sets T from bit 30
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]); // rts delay slot
}
