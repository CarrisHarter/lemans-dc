//! 0x8c013226 — flag check on byte at r4+3: bit 2 clear decrements *(r4+4);
//! set increments **(*(r4+4))+4 and decrements **(*(r4+4)).
//!
//! Original side effects: r0 = byte, T = ((r0 & 4) == 0); clear path: r2 =
//! *(r4+4) - 1 stored back (bra delay slot); set path: r1 = *(r4+4),
//! r3 = *(r1+4) + 1 stored back, then r2 = *(r4+4), r3 = *r2 - 1 stored back.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let b = mem.read_u8(cpu.r[4] + 3).unwrap_or(0); // mov.b @(3,r4),r0
    cpu.r[0] = b as i8 as i32 as u32;
    cpu.set_t(b & 4 == 0); // tst #4,r0
    if b & 4 == 0 {
        cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0).wrapping_sub(1);
        let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[2]); // bra delay slot
    } else {
        cpu.r[1] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
        let mut v = mem.read_u32(cpu.r[1] + 4).unwrap_or(0);
        v = v.wrapping_add(1);
        cpu.r[3] = v;
        let _ = mem.write_u32(cpu.r[1] + 4, v);
        cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
        let mut w = mem.read_u32(cpu.r[2]).unwrap_or(0);
        w = w.wrapping_sub(1);
        cpu.r[3] = w;
        let _ = mem.write_u32(cpu.r[2], w);
    }
}
