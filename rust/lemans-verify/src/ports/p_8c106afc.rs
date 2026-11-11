//! 0x8c106afc — `mov.l 0x8c106b04,r0 ; rts ; mov.l r4,@r0`
//!
//! Loads the global pointer 0x8c2ab3b0 into r0 and stores r4 through it in
//! the rts delay slot. r0 ends holding the pointer, r4 is the stored value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c106b04).unwrap(); // 0x8c2ab3b0
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // rts delay slot: mov.l r4,@r0
}
