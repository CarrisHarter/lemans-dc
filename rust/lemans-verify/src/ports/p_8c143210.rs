//! 0x8c143210 — build a 16-byte header at r4+32 from three floats read
//! through r5 plus a literal 1.0, then copy 12 bytes from r6 to r4+48.
//!
//! The three floats and the 1.0 are spilled to a 16-byte stack frame at
//! [r15-16..r15) (stack writes are excluded from the diff) and read back, so
//! the stores at r4+32.. carry f32[r5], f32[r5+4], f32[r5+8], 1.0 in that
//! order. Final registers: r0 = [r6+8], r1 = [r6], r2 = r15-4, r3 = r15-16,
//! r4 = r4+48, r5 = r5+8, r7 = r4+32, r15 restored by the rts delay slot;
//! fr0 = 1.0, fr1 = f32[r5+8].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r15o = cpu.r[15];
    cpu.r[15] = r15o.wrapping_sub(16); // add #-16,r15
    let r5o = cpu.r[5];

    let f0 = mem.read_f32(r5o).unwrap_or(0.0); // fmov @r5,fr0
    cpu.fr[0] = f0;
    // [r15+0] = fr0 (stack, excluded)
    let f1 = mem.read_f32(r5o.wrapping_add(4)).unwrap_or(0.0); // fmov @(4,r5),fr1
    cpu.fr[1] = f1;
    cpu.r[5] = r5o.wrapping_add(8); // add #8,r5
    // [r15+4] = fr1 (stack, excluded)
    let f2 = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr1 (now r5+8)
    cpu.fr[1] = f2;
    cpu.fr[0] = 1.0; // fldi1 fr0
    // [r15+8] = fr1, [r15+12] = fr0 = 1.0 (stack, excluded)

    cpu.r[7] = cpu.r[4].wrapping_add(32); // mov r4,r7 ; add #32,r7
    cpu.r[3] = cpu.r[15]; // mov r15,r3
    cpu.r[4] = cpu.r[4].wrapping_add(48); // add #48,r4
    // r2 ends as r15+12 (with the decremented r15)
    cpu.r[2] = cpu.r[15].wrapping_add(12);

    let r7 = cpu.r[7];
    let r4 = cpu.r[4];
    // mov.l @r3,r1 / @(4,r3),r0 / @(8,r3),r1 / @(12,r3),r0 read the stack frame
    let l = [f0.to_bits(), f1.to_bits(), f2.to_bits(), 1.0f32.to_bits()];
    let _ = mem.write_u32(r7, l[0]);
    let _ = mem.write_u32(r7 + 4, l[1]);
    let _ = mem.write_u32(r7 + 8, l[2]);
    let _ = mem.write_u32(r7 + 12, l[3]);

    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(r4, cpu.r[1]);
    let _ = mem.write_u32(r4 + 4, cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(r4 + 8, cpu.r[0]);

    cpu.r[15] = cpu.r[15].wrapping_add(16); // rts delay slot: add #16,r15
}
