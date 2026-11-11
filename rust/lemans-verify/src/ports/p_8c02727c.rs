//! 0x8c02727c — copy a float field: `obj->field_34 = obj->field_4`.
//!
//! `mov r4,r0 ; add #4,r4 ; fmov @r4,fr0 ; add #52,r0 ; rts ; fmov fr0,@r0`
//! Ghidra shows an FPSCR.SZ branch, but the shipped code is an unconditional
//! single-precision fmov. Side effects the C hides: r4 ends as `base+4`,
//! r0 as `base+0x34`, and fr0 holds the loaded bits.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = base; // mov r4,r0
    cpu.r[4] = base.wrapping_add(4); // add #4,r4
    let bits = mem.read_u32(cpu.r[4]).unwrap(); // fmov @r4,fr0
    cpu.fr[0] = f32::from_bits(bits);
    cpu.r[0] = cpu.r[0].wrapping_add(52); // add #52,r0
    let _ = mem.write_u32(cpu.r[0], bits); // delay slot: fmov fr0,@r0
}
