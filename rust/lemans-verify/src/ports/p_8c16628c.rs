//! 0x8c16628c - store `r5` at `[r4+73]`, then init the sub-object at `[r4+4]`.
//!
//! r4 is reloaded from its own +4 word; a null sub-object exits with T=1 and
//! no call. Otherwise 0x8c0e1f3c runs with the caller's r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = 73; // mov #73,r0
    let _ = mem.write_u8(cpu.r[4].wrapping_add(73), cpu.r[5] as u8); // mov.b r5,@(r0,r4)
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] != 0 {
        cpu.r[0] = mem.read_u32(0x8c1662a4).unwrap_or(0x8c0e1f3c); // mov.l 0x8c1662a4,r0
        cpu.pr = 0x8c16629e;
        crate::registry::p_8c0e1f3c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
