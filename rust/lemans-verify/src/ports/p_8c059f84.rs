//! 0x8c059f84 — store r4 to the global pointer held in the literal pool.
//!
//! Original: `mov.l 0x8c059f8c,r0 ; rts ; mov.l r4,@r0` (delay slot).
//! The literal at 0x8c059f8c is 0x8c23de08. Side effects the C does not
//! show: r0 is left holding the global's address, and the store lands in
//! the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c059f8c).unwrap_or(0); // 0x8c059f84 mov.l 0x8c059f8c,r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // 0x8c059f88 rts delay: mov.l r4,@r0
}
