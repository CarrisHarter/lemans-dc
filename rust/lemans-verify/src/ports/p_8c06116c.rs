//! 0x8c06116c — `*(u32 *)0x8c23dea0 = r4`.
//!
//! Original: `mov.l 0x8c061174,r0 ; rts ; mov.l r4,@r0`
//! Side effects the C hides: r0 ends as the loaded pointer 0x8c23dea0 (not the
//! stored value), and the store happens in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = mem.read_u32(0x8c061174).unwrap(); // 0x8c23dea0
    cpu.r[0] = dst;
    let _ = mem.write_u32(dst, cpu.r[4]); // delay slot of rts
}
