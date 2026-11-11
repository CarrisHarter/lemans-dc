//! 0x8c084308 — return &base[idx] (base = *(r4+24), idx = r5*32) or 0 if base[idx] == 0.
//!
//! Side effects the C does not show: r3 ends as base+idx (or 0), r1 and r0 end
//! as the zero-extended byte, T = (byte == 0); the `mov r3,r0` is the rts
//! delay slot, so r0 always equals r3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let idx = (cpu.r[5] & 0xffff).wrapping_mul(32); // extu.w; shll2; shll2; shll
    cpu.r[0] = idx;
    let base = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0); // mov.l @(24,r4),r3
    cpu.r[3] = base;
    let b = mem.read_u8(base.wrapping_add(idx)).unwrap_or(0); // mov.b @(r0,r3),r1
    cpu.r[1] = b as u32;
    cpu.r[3] = base.wrapping_add(idx); // add r0,r3
    cpu.r[1] &= 0xff; // extu.b r1,r1
    cpu.r[0] = cpu.r[1]; // extu.b r1,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.r[3] = 0; // bt -> mov #0,r3 (bra skips nothing else)
    }
    cpu.r[0] = cpu.r[3]; // rts delay slot: mov r3,r0
}
