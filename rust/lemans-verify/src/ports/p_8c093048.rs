//! 0x8c093048 — result is 1 iff the byte at r4+0x95 is nonzero, else 0.
//!
//! Bytes: mov.l (0x95),r0; mov.b @(r0,r4),r0; extu.b x2; tst; bf.s
//! 0x8c093064 (delay: mov #1,r1). The fall-through re-reads at r4+0x96,
//! tests again, then `bf 0x8c093064` whose delay slot is `mov #0,r1`; both
//! outcomes reach `rts` with r1 = 0. The `rts` delay slot is `mov r1,r0`,
//! so r0 = r1 and the +0x96 test decides only T, not the return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x95;
    let b1 = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[0] = b1 as u32; // extu.b x2
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[1] = 1; // bf.s delay slot: mov #1,r1
    if cpu.r[0] == 0 {
        cpu.r[0] = 0x96;
        let b2 = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.r[0] = b2 as u32; // extu.b x2
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        cpu.r[1] = 0; // bf delay slot: mov #0,r1 (runs on both paths)
    }
    cpu.r[0] = cpu.r[1]; // rts delay slot: mov r1,r0
}
