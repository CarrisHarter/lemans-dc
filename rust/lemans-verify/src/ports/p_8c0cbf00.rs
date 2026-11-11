//! 0x8c0cbf00 — initialise a 19-dword struct at r4: zero/identity floats at
//! +48..+72 (fr1 = 0 via fldi0, fr0 = 1.0 via fldi1) and 12 dwords copied
//! from the four image pointers 0x8c1d3f68/0x8c1d3f30/0x8c1d3f50/0x8c1d3f40.
//!
//! Residue the C does not show: fr0 = 1.0 and fr1 = 0.0 stay live on exit;
//! r0 = r4, r1 = *(p6+8), r3 = *p6, r5 = 0x8c1d3f40, r6 = r4+24,
//! r7 = 0x8c1d3f50; r12/r13/r14 are popped back to their entry values.
//! Stores happen in instruction order: the 7 fmovs, then +0,+4,+8 from p3,
//! +12,+16,+20 from p2, +36,+40,+44 from p9, +24,+28,+32 from p6.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let saved12 = cpu.r[12];
    let saved13 = cpu.r[13];
    let saved14 = cpu.r[14];
    cpu.fr[1] = 0.0; // fldi0 fr1
    let _ = mem.write_f32(r4.wrapping_add(48), cpu.fr[1]); // fmov fr1,@r1
    let _ = mem.write_f32(r4.wrapping_add(52), cpu.fr[1]); // fmov fr1,@r0
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_f32(r4.wrapping_add(56), cpu.fr[0]); // fmov fr0,@r6
    let _ = mem.write_f32(r4.wrapping_add(60), cpu.fr[1]); // fmov fr1,@r5
    let _ = mem.write_f32(r4.wrapping_add(64), cpu.fr[1]); // fmov fr1,@r3
    let _ = mem.write_f32(r4.wrapping_add(68), cpu.fr[1]); // fmov fr1,@r1
    let _ = mem.write_f32(r4.wrapping_add(72), cpu.fr[1]); // fmov fr1,@r0
    let p3 = mem.read_u32(0x8c0cbf84).unwrap_or(0); // -> 0x8c1d3f68
    let p2 = mem.read_u32(0x8c0cbf88).unwrap_or(0); // -> 0x8c1d3f30
    let p9 = mem.read_u32(0x8c0cbf8c).unwrap_or(0); // -> 0x8c1d3f50
    let p6 = mem.read_u32(0x8c0cbf90).unwrap_or(0); // -> 0x8c1d3f40
    let r1 = mem.read_u32(p3).unwrap_or(0); // mov.l @r3,r1
    let r0 = mem.read_u32(p3.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r3),r0
    let _ = mem.write_u32(r4, r1); // mov.l r1,@r4
    let _ = mem.write_u32(r4.wrapping_add(4), r0); // mov.l r0,@(4,r4)
    let r0 = mem.read_u32(p3.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r3),r0
    let _ = mem.write_u32(r4.wrapping_add(8), r0); // mov.l r0,@(8,r4)
    let r3 = mem.read_u32(p2).unwrap_or(0); // mov.l @r13,r3
    let r1 = mem.read_u32(p2.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r13),r1
    let _ = mem.write_u32(r4.wrapping_add(12), r3); // mov.l r3,@r12
    let _ = mem.write_u32(r4.wrapping_add(16), r1); // mov.l r1,@(4,r12)
    let r1 = mem.read_u32(p2.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r13),r1
    let _ = mem.write_u32(r4.wrapping_add(20), r1); // mov.l r1,@(8,r12)
    let r3 = mem.read_u32(p9).unwrap_or(0); // mov.l @r7,r3
    let r1 = mem.read_u32(p9.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r7),r1
    let _ = mem.write_u32(r4.wrapping_add(36), r3); // mov.l r3,@r14
    let _ = mem.write_u32(r4.wrapping_add(40), r1); // mov.l r1,@(4,r14)
    let r1 = mem.read_u32(p9.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r7),r1
    let _ = mem.write_u32(r4.wrapping_add(44), r1); // mov.l r1,@(8,r14)
    let r3 = mem.read_u32(p6).unwrap_or(0); // mov.l @r5,r3
    let r1 = mem.read_u32(p6.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r5),r1
    let _ = mem.write_u32(r4.wrapping_add(24), r3); // mov.l r3,@r6
    let _ = mem.write_u32(r4.wrapping_add(28), r1); // mov.l r1,@(4,r6)
    let r1 = mem.read_u32(p6.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r5),r1
    let _ = mem.write_u32(r4.wrapping_add(32), r1); // mov.l r1,@(8,r6)
    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = p6;
    cpu.r[6] = r4.wrapping_add(24);
    cpu.r[7] = p9;
    cpu.r[12] = saved12; // popped
    cpu.r[13] = saved13;
    cpu.r[14] = saved14;
}
