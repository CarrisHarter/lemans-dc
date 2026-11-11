//! 0x8c0da7b0 — build a 28-word (112-byte) struct at r4.
//!
//! Straight-line (the decompiler's FPSCR.SZ branch does not exist in the
//! machine code; the oracle's `fmov` is always 32-bit). Layout written to
//! r4+0..r4+108:
//!
//! ```text
//! [r4+  0] = fr4            fmov fr4,@r4
//! [r4+  4] = @r5+0  .. [r4+16] = @r5+12   (4 words via fr0)
//! [r4+ 20] = fr6, +24 = fr7, +28 = fr8, +32 = fr9, +36 = fr10, +40 = fr11
//! [r4+ 44] = r6            mov.l r6,@(44,r4)
//! [r4+ 48] = @(sp+4)
//! [r4+ 52] = @r7+0  .. [r4+64] = @r7+12    (4 words)
//! [r4+ 68] = @r1+0  .. [r4+80] = @r1+12    (r1 = @(sp+8))
//! [r4+ 84] = @r0+0  .. [r4+96] = @r0+12    (r0 = @(sp+12))
//! [r4+100] = @(sp+16)
//! [r4+104] = fr5
//! [r4+108] = @(sp+0)
//! ```
//!
//! Hidden side effects the decompiled C does not show: r1 = r4+108,
//! r2 = entry sp, r3 = r4+104, r5 = r4+100, r6 = r4+96, r7 = r4+92,
//! r0 = r4 (return value), fr0 = @(entry sp) (last load), r12/r13/r14
//! restored, r15 back to entry sp. T is untouched (no compares).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[15];
    let saved12 = cpu.r[12];
    let saved13 = cpu.r[13];
    let saved14 = cpu.r[14];
    cpu.r[15] = s.wrapping_sub(12); // push r14,r13,r12
    let frame = cpu.r[15];
    let r1 = mem.read_u32(frame + 20).unwrap(); // @(sp+8)
    let r0 = mem.read_u32(frame + 24).unwrap(); // @(sp+12)
    let d = cpu.r[4];
    let r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let r7 = cpu.r[7];

    let mut fr0;
    let _ = mem.write_f32(d, cpu.fr[4]); // fmov fr4,@r4
    fr0 = mem.read_f32(r5).unwrap();
    let _ = mem.write_f32(d + 4, fr0);
    fr0 = mem.read_f32(r5 + 4).unwrap();
    let _ = mem.write_f32(d + 8, fr0);
    fr0 = mem.read_f32(r5 + 8).unwrap();
    let _ = mem.write_f32(d + 12, fr0);
    fr0 = mem.read_f32(r5 + 12).unwrap();
    let _ = mem.write_f32(d + 16, fr0);
    let _ = mem.write_f32(d + 20, cpu.fr[6]);
    let _ = mem.write_f32(d + 24, cpu.fr[7]);
    let _ = mem.write_f32(d + 28, cpu.fr[8]);
    let _ = mem.write_f32(d + 32, cpu.fr[9]);
    let _ = mem.write_f32(d + 36, cpu.fr[10]);
    let _ = mem.write_f32(d + 40, cpu.fr[11]);
    let _ = mem.write_u32(d + 44, r6); // mov.l r6,@(44,r4)
    fr0 = mem.read_f32(frame + 16).unwrap(); // r2 = 16; add r15,r2 -> @(sp+4)
    let _ = mem.write_f32(d + 48, fr0);
    fr0 = mem.read_f32(r7).unwrap();
    let _ = mem.write_f32(d + 52, fr0);
    fr0 = mem.read_f32(r7 + 4).unwrap();
    let _ = mem.write_f32(d + 56, fr0);
    fr0 = mem.read_f32(r7 + 8).unwrap();
    let _ = mem.write_f32(d + 60, fr0);
    fr0 = mem.read_f32(r7 + 12).unwrap();
    let _ = mem.write_f32(d + 64, fr0);
    fr0 = mem.read_f32(r1).unwrap();
    let _ = mem.write_f32(d + 68, fr0);
    fr0 = mem.read_f32(r1 + 4).unwrap();
    let _ = mem.write_f32(d + 72, fr0);
    fr0 = mem.read_f32(r1 + 8).unwrap();
    let _ = mem.write_f32(d + 76, fr0);
    fr0 = mem.read_f32(r1 + 12).unwrap();
    let _ = mem.write_f32(d + 80, fr0);
    fr0 = mem.read_f32(r0).unwrap();
    let _ = mem.write_f32(d + 84, fr0);
    fr0 = mem.read_f32(r0 + 4).unwrap();
    let _ = mem.write_f32(d + 88, fr0);
    fr0 = mem.read_f32(r0 + 8).unwrap();
    let _ = mem.write_f32(d + 92, fr0);
    fr0 = mem.read_f32(r0 + 12).unwrap();
    let _ = mem.write_f32(d + 96, fr0);
    fr0 = mem.read_f32(frame + 28).unwrap(); // r2 = 28 -> @(sp+16)
    let _ = mem.write_f32(d + 100, fr0);
    let _ = mem.write_f32(d + 104, cpu.fr[5]); // fmov fr5,@r3
    fr0 = mem.read_f32(frame + 12).unwrap(); // r2 = 12 -> @(sp+0)
    let _ = mem.write_f32(d + 108, fr0);

    cpu.fr[0] = fr0;
    cpu.r[0] = d;
    cpu.r[1] = d + 108;
    cpu.r[2] = s;
    cpu.r[3] = d + 104;
    cpu.r[5] = d + 100;
    cpu.r[6] = d + 96;
    cpu.r[7] = d + 92;
    cpu.r[12] = saved12;
    cpu.r[13] = saved13;
    cpu.r[14] = saved14;
    cpu.r[15] = s;
}
