//! 0x8c0572fc — copy the 16 bytes at 0x8c23dd60 to [[r15]] through a stack spill.
//!
//! ```text
//! r1 = 0x8c23dd60 ; r15 -= 16 ; r0 = 0x8c23dd68   (literal pool)
//! spill fr1/fr0 words to [r15+0..12] while reloading them from r1/r0
//! r4 = @(16,r15)            ; i.e. the dword at the ORIGINAL r15
//! copy [r15+0..12] to [r4+0..12]
//! rts ; add #16,r15         ; delay slot restores r15
//! ```
//! Leftovers the C does not show: r2 = r15-4, r3 = r15-16 after the run
//! (r15 itself is restored by the delay slot), r0/r1 end as the last two
//! spilled words, fr0/fr1 keep the bits of [0x8c23dd6c]/[0x8c23dd64], and
//! the four spill stores at r15-16..r15-4 are real writes (the spec seeds
//! r15 inside the image so they land in compared memory).
//! Verified against fixtures/lemans24-ext.bin (globals at 0x8c23dd60 lie
//! past the end of analysis/lemans24.bin).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r15_orig = cpu.r[15];
    let a = mem.read_u32(0x8c057340).unwrap_or(0); // 0x8c23dd60
    let b = mem.read_u32(0x8c057344).unwrap_or(0); // 0x8c23dd68
    cpu.r[15] = r15_orig.wrapping_sub(16); // add #-16,r15
    let sp = cpu.r[15];
    cpu.fr[1] = f32::from_bits(mem.read_u32(a).unwrap_or(0)); // fmov @r1,fr1
    cpu.r[2] = sp.wrapping_add(0); // mov #0,r2 ; add r15,r2
    let _ = mem.write_u32(cpu.r[2], cpu.fr[1].to_bits()); // fmov fr1,@r2
    cpu.fr[1] = f32::from_bits(mem.read_u32(a.wrapping_add(4)).unwrap_or(0)); // fmov @r1,fr1
    cpu.r[2] = sp.wrapping_add(4); // mov #4,r2 ; add r15,r2
    let _ = mem.write_u32(cpu.r[2], cpu.fr[1].to_bits());
    cpu.fr[0] = f32::from_bits(mem.read_u32(b).unwrap_or(0)); // fmov @r0,fr0
    cpu.r[2] = sp.wrapping_add(8); // mov #8,r2 ; add r15,r2
    let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits()); // fmov fr0,@r2
    cpu.fr[0] = f32::from_bits(mem.read_u32(b.wrapping_add(4)).unwrap_or(0)); // fmov @r0,fr0
    cpu.r[2] = sp.wrapping_add(12); // mov #12,r2 ; add r15,r2
    cpu.r[4] = mem.read_u32(r15_orig).unwrap_or(0); // mov.l @(16,r15),r4
    let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits()); // fmov fr0,@r2
    cpu.r[3] = sp; // mov r15,r3
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r1
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r3),r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // mov.l r1,@r4
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r4)
    cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r3),r1
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r3),r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r4)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[0]); // mov.l r0,@(12,r4)
    cpu.r[15] = r15_orig; // rts delay slot: add #16,r15
}
