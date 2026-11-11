//! 0x8c026e34 — move the (0x2c,0x30) pair from src to dst and clear src.
//!
//! The C hides that r0 ends as 0 and r1 keeps the *second* loaded field
//! (src+0x30), and that the final zeroing store is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src + 0x2c).unwrap();
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(dst + 0x2c, cpu.r[1]);
    cpu.r[1] = mem.read_u32(src + 0x30).unwrap();
    let _ = mem.write_u32(dst + 0x30, cpu.r[1]);
    let _ = mem.write_u32(src + 0x2c, cpu.r[0]);
    let _ = mem.write_u32(src + 0x30, cpu.r[0]); // delay slot of `rts`
}
