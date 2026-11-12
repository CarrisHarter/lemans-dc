//! 0x8c056c20 — pack four `ftrc` results and store them at 0x8c1f6fc0.
//!
//! Reads fr3 = [0x8c056c64] = 255.0f, multiplies the four floats at r4 by it
//! (fr0/fr2/fr1 end as x0/x1/x2 * 255, fr0 is reloaded with x3 and re-multiplied),
//! then runs four `ftrc` (bytes `3d f0`/`3d f2`/`3d f1`/`3d f0`) into fpul and
//! packs the results big-endian into r3. `ftrc FRn,FPUL` takes its source from
//! the n field (verified with `sh-elf-as`: `ftrc fr4,fpul` assembles to
//! `3d f4`), so the four convert fr0, fr2, fr1 and fr0. The store is the `rts`
//! delay slot.
//!
//! Leftovers the C does not show: r4 = param+12, r1 = fpul<<16,
//! r0 = 0x8c1f6fc0 (the pool pointer, not a converted value), fpul = the
//! last ftrc, fr0/fr1/fr2 = x3*c / x2*c / x1*c, fr3 = 255.0f, and T is the
//! carry of the last shift, `shll8 r0` at 0x8c056c52, i.e. bit 24 of the third
//! converted value (`or` does not touch T, so it survives to the return).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let c = f32::from_bits(mem.read_u32(0x8c056c64).unwrap_or(0)); // mova + fmov @r0,fr3
    cpu.fr[3] = c;
    cpu.fr[0] = f32::from_bits(mem.read_u32(base).unwrap_or(0)); // fmov @r4,fr0
    cpu.fr[0] = cpu.fr[0] * c; // fmul fr3,fr0
    cpu.r[4] = base.wrapping_add(12); // add #12,r4
    cpu.fr[2] = f32::from_bits(mem.read_u32(base.wrapping_add(4)).unwrap_or(0)); // fmov @r3,fr2
    cpu.fr[1] = f32::from_bits(mem.read_u32(base.wrapping_add(8)).unwrap_or(0)); // fmov @r1,fr1
    cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc (0xF03D): source is the n field, fr0
    cpu.fr[2] = cpu.fr[2] * c; // fmul fr3,fr2
    cpu.r[3] = cpu.fpul; // sts fpul,r3
    cpu.fr[0] = f32::from_bits(mem.read_u32(base.wrapping_add(12)).unwrap_or(0)); // fmov @r4,fr0
    cpu.r[3] = cpu.r[3] << 16; // shll16 r3
    cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc (0xF23D): fr2
    cpu.fr[1] = cpu.fr[1] * c; // fmul fr3,fr1
    cpu.r[1] = cpu.fpul; // sts fpul,r1
    cpu.fpul = cpu.fr[1] as i32 as u32; // ftrc (0xF13D): fr1
    cpu.r[3] = cpu.r[3] << 8; // shll8 r3
    cpu.r[1] = cpu.r[1] << 16; // shll16 r1
    cpu.fr[0] = cpu.fr[0] * c; // fmul fr3,fr0
    cpu.r[0] = cpu.fpul; // sts fpul,r0
    cpu.r[3] |= cpu.r[1]; // or r1,r3
    cpu.set_t(cpu.r[0] >> 24 & 1 != 0); // shll8 r0 sets T from bit 24
    cpu.r[0] = cpu.r[0] << 8; // shll8 r0
    cpu.r[3] |= cpu.r[0]; // or r0,r3
    cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc (0xF03D): fr0 again
    cpu.r[0] = cpu.fpul; // sts fpul,r0
    cpu.r[3] |= cpu.r[0]; // or r0,r3
    cpu.r[0] = mem.read_u32(0x8c056c68).unwrap_or(0); // mov.l 0x8c056c68,r0 = 0x8c1f6fc0
    let _ = mem.write_u32(cpu.r[0], cpu.r[3]); // rts delay slot: mov.l r3,@r0
}
