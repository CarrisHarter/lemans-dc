//! 0x8c013bfa — if *r4 == 1 return 0, else return (*(r4+4) + 16) byte & 0x40.
//!
//! Original side effects: r0 = *r4, T = (r0 == 1); equal path: r0 := 0 (rts
//! delay slot); other path: r0 = *(r4+4), r1 = 16, r0 = mem.b[r0+16] & 0x40.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.r[0] == 1 {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        return;
    }
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0); // mov.l @(4,r4),r0
    cpu.r[1] = 16;
    let b = mem.read_u8(cpu.r[0] + cpu.r[1]).unwrap_or(0); // mov.b @(r0,r1),r0
    cpu.r[0] = (b as u32) & 0x40; // and #64,r0
}
