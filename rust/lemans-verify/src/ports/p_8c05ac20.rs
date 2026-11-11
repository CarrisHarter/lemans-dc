//! 0x8c05ac20 — store fr4 at `*(r4+4) + 0xc` and fr5 at `*(r4+4) + 0x10`.
//!
//! Original: `mov.l @(4,r4),r0 ; add #12,r0 ; fmov fr4,@r0 ;
//!            mov.l @(4,r4),r0 ; add #16,r0 ; rts ; fmov fr5,@r0` (delay).
//! Side effects the C does not show: the base pointer is reloaded from
//! `r4+4` for the second store (not derived from the first r0), and r0 is
//! left holding `base + 0x10`. Stores are 32-bit, bit-exact via `to_bits`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // 0x8c05ac20 mov.l @(4,r4),r0
    let a = base.wrapping_add(0xc); // 0x8c05ac22 add #12,r0
    let _ = mem.write_u32(a, cpu.fr[4].to_bits()); // 0x8c05ac24 fmov fr4,@r0
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // 0x8c05ac26 reload
    cpu.r[0] = cpu.r[0].wrapping_add(0x10); // 0x8c05ac28 add #16,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[5].to_bits()); // 0x8c05ac2c rts delay: fmov fr5,@r0
}
