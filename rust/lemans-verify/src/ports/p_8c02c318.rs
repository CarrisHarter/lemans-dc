//! 0x8c02c318 — gather three strided floats plus a fourth weighted one into an
//! output triple.
//!
//! r4 = object base, r5 = element index, stack argument at [r15] = output
//! pointer. The index scales two strides: 52 (0x34) bytes for the three
//! sources based at `*(u32*)(r4+4)`, and 172 (0xac) bytes for the source
//! based at r4+88.
//!
//! ```text
//! base = *(u32*)(r4+4)
//! out[0] = f32[base + 0x1ac + 52*i]
//! out[1] = f32[base + 0x1b0 + 52*i]
//! out[2] = f32[base + 0x1b4 + 52*i] + f32[r4 + 88 + 172*i]
//! ```
//!
//! Both offsets are computed with wrapping 32-bit arithmetic, and the index is
//! treated as signed: `mul.l` writes the signed 64-bit product to MACH:MACL and
//! the code keeps only MACL, so a negative i biases both address calculations
//! downwards. There are no branches or calls, so this is the whole function.
//!
//! `r12`, `r13`, `r14` are pushed before being overwritten and popped in the
//! epilogue, so they return to their entry values. r0 ends as out+4 (the middle
//! store's address), r1 as 0x1ac, r6 as 0x1b0 and r7/r3/r4 as the last
//! computed source addresses; MACH:MACL hold the 172*i product. The `rts`
//! delay slot performs the final `mov.l @r15+,r14`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c02c318: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c02c31a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c02c31c: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c02c31e: mov.l @(12,r15),r12
    cpu.r[12] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    // 0x8c02c320: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c02c322: mov #52,r0
    cpu.r[0] = 52;
    // 0x8c02c324: mul.l r0,r14
    let p = (cpu.r[0] as i32 as i64).wrapping_mul(cpu.r[14] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    // 0x8c02c326: sts macl,r14
    cpu.r[14] = cpu.macl;
    // 0x8c02c328: mov.l 0x8c02c368,r1
    cpu.r[1] = mem.read_u32(0x8c02c368).unwrap_or(0xac);
    // 0x8c02c32a: mov r12,r0
    cpu.r[0] = cpu.r[12];
    // 0x8c02c32c: mov.l @(4,r4),r13
    cpu.r[13] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    // 0x8c02c32e: mul.l r1,r5
    let p = (cpu.r[1] as i32 as i64).wrapping_mul(cpu.r[5] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    // 0x8c02c330: mov.l 0x8c02c36c,r3
    cpu.r[3] = mem.read_u32(0x8c02c36c).unwrap_or(0x1b4);
    // 0x8c02c332: mov r13,r7
    cpu.r[7] = cpu.r[13];
    // 0x8c02c334: sts macl,r5
    cpu.r[5] = cpu.macl;
    // 0x8c02c336: add r3,r7
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    // 0x8c02c338: add #88,r4
    cpu.r[4] = cpu.r[4].wrapping_add(88);
    // 0x8c02c33a: add r14,r7
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[14]);
    // 0x8c02c33c: add r5,r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    // 0x8c02c33e: mov.l 0x8c02c370,r6
    cpu.r[6] = mem.read_u32(0x8c02c370).unwrap_or(0x1b0);
    // 0x8c02c340: mov r13,r3
    cpu.r[3] = cpu.r[13];
    // 0x8c02c342: mov.l 0x8c02c374,r1
    cpu.r[1] = mem.read_u32(0x8c02c374).unwrap_or(0x1ac);
    // 0x8c02c344: add r6,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]);
    // 0x8c02c346: add r14,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[14]);
    // 0x8c02c348: add r1,r13
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[1]);
    // 0x8c02c34a: fmov @r7,fr2
    cpu.fr[2] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    // 0x8c02c34c: add r14,r13
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[14]);
    // 0x8c02c34e: fmov @r4,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    // 0x8c02c350: fmov @r3,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    // 0x8c02c352: fadd fr0,fr2
    cpu.fr[2] += cpu.fr[0];
    // 0x8c02c354: fmov @r13,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[13]).unwrap_or(0.0);
    // 0x8c02c356: add #4,r0
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    // 0x8c02c358: fmov fr0,@r12
    let _ = mem.write_f32(cpu.r[12], cpu.fr[0]);
    // 0x8c02c35a: fmov fr1,@r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
    // 0x8c02c35c: add #8,r12
    cpu.r[12] = cpu.r[12].wrapping_add(8);
    // 0x8c02c35e: fmov fr2,@r12
    let _ = mem.write_f32(cpu.r[12], cpu.fr[2]);
    // 0x8c02c360: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c02c362: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c02c364: rts ; 0x8c02c366: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
