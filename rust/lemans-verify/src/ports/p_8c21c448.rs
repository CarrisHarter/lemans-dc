//! 0x8c21c448 — save the SR IMASK nibble to `*r4`, then raise IMASK to 15.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[3] = mem.read_u16(0x8c21c572).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0]; // ldc r0,sr
}
