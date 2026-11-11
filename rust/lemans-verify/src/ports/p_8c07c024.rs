//! 0x8c07c024 — float_is_zero_to_byte.
//!
//! `fldi0 fr0 ; fcmp/eq fr0,fr4 ; subc r1,r1 ; add #1,r1 ; mov #32,r0 ;
//! rts ; mov.b r1,@(r0,r4)` — writes 1 when `fr4` is non-zero and 0 when it is
//! zero, and `r0` ends as 32.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = 0.0;
    let equal = cpu.fr[0] == cpu.fr[4];
    cpu.set_t(equal); // fcmp/eq fr0,fr4
    cpu.r[1] = 0u32.wrapping_sub(equal as u32); // subc r1,r1
    cpu.r[1] = cpu.r[1].wrapping_add(1);
    cpu.r[0] = 32;
    let _ = mem.write_u8(cpu.r[4] + 32, cpu.r[1] as u8); // delay slot of `rts`
}
