//! 0x8c010fa0 — call the pow kernel (0x8c013fc0) on dr4 (fr4 = low word,
//! fr5 = high word), then classify the f32 it returns in fr0: NaN writes
//! 0x44d to 0x8c23d8b8, Inf writes 0x44c, finite writes nothing. Returns fr0.
//!
//! Shape: `sts.l pr,@-r15 ; add #-4,r15` reserves the slot that receives
//! `fmov fr0,@r15`; the literal pool at 0x8c010fe0..0x8c010fef holds
//! 0x8c013fc0, 0x7f800000, 0x007fffff and 0x8c23d8b8, while the 16-bit
//! literals at 0x8c010fda/0x8c010fdc encode 0x44d/0x44c. The jsr delay slot
//! is a nop; the `bf 0x8c010fd0` skips the classification when the exponent
//! field of the low result word is not all ones (finite), `bt 0x8c010fca`
//! distinguishes the zero mantissa (Inf) from nonzero (NaN). The epilogue
//! reloads fr0 from the slot and restores pr, so r15 nets out and T ends as
//! the last cmp/eq or tst of the taken path (0 on finite/NaN, 1 on Inf).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c010fa0..0x8c010fa8: push pr, reserve the result slot, call the kernel
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = mem.read_u32(0x8c010fe0).unwrap_or(0); // 0x8c013fc0
    cpu.pr = 0x8c010faa;
    crate::registry::p_8c013fc0::run(cpu, mem);

    // 0x8c010faa..0x8c010fb6: stash fr0 and test its exponent field
    cpu.r[3] = cpu.r[15];
    cpu.r[4] = mem.read_u32(0x8c010fe4).unwrap_or(0); // 0x7f800000
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]);
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] &= cpu.r[4];
    cpu.set_t(cpu.r[2] == cpu.r[4]); // cmp/eq r4,r2
    if cpu.t() {
        // 0x8c010fb8..0x8c010fce: exponent all ones — NaN or Inf?
        cpu.r[2] = cpu.r[15];
        cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c010fe8).unwrap_or(0); // 0x007fffff
        cpu.set_t(cpu.r[3] & cpu.r[1] == 0); // tst r3,r1
        if !cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c010fec).unwrap_or(0); // 0x8c23d8b8
            cpu.r[0] = mem.read_u16(0x8c010fda).unwrap_or(0) as i16 as i32 as u32; // 0x44d
            let _ = mem.write_u32(cpu.r[2], cpu.r[0]); // bra delay slot
        } else {
            cpu.r[1] = mem.read_u32(0x8c010fec).unwrap_or(0);
            cpu.r[2] = mem.read_u16(0x8c010fdc).unwrap_or(0) as i16 as i32 as u32; // 0x44c
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
        }
    }

    // 0x8c010fd0..0x8c010fd8: reload fr0, pop the temp and pr
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
