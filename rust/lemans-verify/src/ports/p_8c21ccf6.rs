//! 0x8c21ccf6 - tail jump to 0x8c2199ca.
//!
//! The delay slot of the `jmp` undoes the `add #-4,r15`, so the callee sees
//! the entry stack pointer. The word stored at the temporary slot is in the
//! synthetic stack window and is excluded from the diff.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[3] = mem.read_u32(0x8c21cdd8).unwrap_or(0x8c2199ca); // mov.l 0x8c21cdd8,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[15] = cpu.r[15].wrapping_add(4); // jmp delay: add #4,r15
    crate::registry::p_8c2199ca::run(cpu, mem);
}
