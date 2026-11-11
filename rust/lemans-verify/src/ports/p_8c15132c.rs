//! 0x8c15132c — `*0x8c2abce0 = r4` (the store is the rts delay slot).
//!
//! Original:
//! ```text
//! mov.l 0x8c151334,r0   ; r0 = 0x8c2abce0
//! rts ; mov.l r4,@r0
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c151334).unwrap_or(0); // 0x8c2abce0
    cpu.r[0] = g;
    let _ = mem.write_u32(g, cpu.r[4]); // rts delay slot
}
