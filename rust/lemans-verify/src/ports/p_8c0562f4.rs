//! 0x8c0562f4 — weighted sum of three floats, returned in fr0.
//!
//! fr0 = f32[r4+4]*[0x8c05631c] + f32[r4+8]*[0x8c056320] + f32[r4+12]*[0x8c056324]
//! (constants 0.2989, 0.5866, 0.1144 from the literal pool, multiplied in
//! exactly this instruction order — the adds are two separate fadds).
//!
//! ```text
//! mova 0x8c05631c,r0 ; fmov @r0,fr2 ; r3 = r4 ; r1 = r4
//! mova 0x8c056320,r0 ; r3 += 4 ; r1 += 8 ; fmov @r0,fr1 ; r4 += 12
//! fmov @r3,fr0 ; mova 0x8c056324,r0 ; fmov @r1,fr3
//! fmul fr2,fr0 ; fmov @r4,fr2 ; fmul fr1,fr3 ; fmov @r0,fr1
//! fadd fr3,fr0 ; fmul fr1,fr2 ; rts ; fadd fr2,fr0   (delay slot)
//! ```
//! Side effects the C does not show: r0 = 0x8c056324 (last mova), r1 = r4o+8,
//! r3 = r4o+4, r4 = r4o+12; fr1 = the third constant, fr2 = c*c2, fr3 = b*c1;
//! the final `fadd fr2,fr0` is the rts delay slot. T is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4o = cpu.r[4];
    let c0 = f32::from_bits(mem.read_u32(0x8c05631c).unwrap_or(0)); // fmov @r0,fr2
    cpu.fr[2] = c0;
    cpu.r[3] = r4o.wrapping_add(4); // mov r4,r3 ; add #4,r3
    cpu.r[1] = r4o.wrapping_add(8); // mov r4,r1 ; add #8,r1
    let c1 = f32::from_bits(mem.read_u32(0x8c056320).unwrap_or(0));
    cpu.fr[1] = c1; // fmov @r0,fr1
    cpu.r[4] = r4o.wrapping_add(12); // add #12,r4
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap_or(0)); // fmov @r3,fr0
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0)); // fmov @r1,fr3
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // fmul fr2,fr0
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0)); // fmov @r4,fr2
    cpu.fr[3] = cpu.fr[3] * cpu.fr[1]; // fmul fr1,fr3
    let c2 = f32::from_bits(mem.read_u32(0x8c056324).unwrap_or(0));
    cpu.fr[1] = c2; // fmov @r0,fr1
    cpu.fr[0] += cpu.fr[3]; // fadd fr3,fr0
    cpu.fr[2] = cpu.fr[2] * cpu.fr[1]; // fmul fr1,fr2
    cpu.fr[0] += cpu.fr[2]; // rts delay slot: fadd fr2,fr0
    cpu.r[0] = 0x8c056324; // last mova 0x8c056324,r0
}
