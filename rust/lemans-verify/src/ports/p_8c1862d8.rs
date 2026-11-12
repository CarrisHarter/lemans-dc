//! 0x8c1862d8 — record initialiser.
//!
//! Saves r14/r13/pr, calls the template builder 0x8c1863e8 with `r4 + 12`
//! (`add #12,r4` in the `jsr` delay slot), then copies the 12-byte global at
//! 0x8c1d3f68 (pool 0x8c186310) into r4+0, +4, +8, clears the byte at r4+0x80
//! (`r1 = 0`, `r0 = 0x80`) and stores the second argument at r4+0x84
//! (`r0 = 0x84`). Returns r4.
//!
//! Final regs: r0 = r4, r1 = 0, r3 = *(B+8), r4 = *B, r5 = B, r6 = *(B+4),
//! r7 = *B (both left by the callee), r13/r14 restored, r2 untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];

    cpu.r[0] = mem.read_u32(0x8c18630c).unwrap_or(0); // 0x8c1863e8
    cpu.r[4] = cpu.r[4].wrapping_add(12); // delay slot of `jsr`
    crate::registry::p_8c1863e8::run(cpu, mem);

    cpu.r[5] = mem.read_u32(0x8c186310).unwrap_or(0); // 0x8c1d3f68
    cpu.r[1] = 0;
    cpu.r[0] = mem.read_u32(0x8c186314).unwrap_or(0); // 0x80
    let b = cpu.r[5];
    cpu.r[4] = mem.read_u32(b).unwrap_or(0);
    cpu.r[3] = mem.read_u32(b.wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[13], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[3]);
    cpu.r[3] = mem.read_u32(b.wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[3]);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[1] as u8);
    cpu.r[0] = mem.read_u32(0x8c186318).unwrap_or(0); // 0x84
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[14]);
    cpu.r[0] = cpu.r[13];

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
