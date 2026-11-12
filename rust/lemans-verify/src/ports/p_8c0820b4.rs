//! 0x8c0820b4 — fill a 12-word normalized-rect table from fr4..fr7.
//!
//! Saves fr12..fr15/r14/pr, zeroes words 0 and 1, calls 0x8c056c04 with
//! `r4 = base + 24` to copy the two global dwords at 0x8c23dd60 into words
//! 6 and 7, then derives:
//!
//! ```text
//! w8  = 1/w6                 w9  = 1/w7
//! w2  = fr4*w8               w3  = (fr4+fr6)*w8
//! w4  = (fr5+fr7)*w9         w5  = fr5*w9
//! w10 = w3-w2                w11 = w4-w5
//! ```
//!
//! All arithmetic is single precision, matching the oracle. The routine ends
//! with fr0 = w5, fr1 = w4-w5, fr2 = w8; fr12..fr15 are restored. Registers
//! left behind: r0 = base+40, r1 = base+20, r3 = base+16, r4 = base+12,
//! r5 = base+8, r6 = base+36, r7 = base+32, r14 restored.
//!
//! The literal at 0x8c082150 is 0x8c056c04; the callee's rts delay slot is
//! the `[base+28]` store. No conditional branches.

use sh4_oracle::{Cpu, Memory};

fn push_f(cpu: &mut Cpu, mem: &mut Memory, n: usize) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[n]);
}

fn pop_f(cpu: &mut Cpu, mem: &mut Memory, n: usize) {
    cpu.fr[n] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push_f(cpu, mem, 15); // fmov fr15,@-r15
    push_f(cpu, mem, 14); // fmov fr14,@-r15
    push_f(cpu, mem, 13); // fmov fr13,@-r15
    push_f(cpu, mem, 12); // fmov fr12,@-r15
    cpu.fr[13] = cpu.fr[5]; // fmov fr5,fr13
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[12] = cpu.fr[4]; // fmov fr4,fr12
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.fr[14] = cpu.fr[6]; // fmov fr6,fr14
    cpu.fr[15] = cpu.fr[7]; // fmov fr7,fr15
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]); // fmov fr0,@r14
    cpu.r[1] = cpu.r[14]; // mov r14,r1
    cpu.r[0] = mem.read_u32(0x8C08_2150).unwrap_or(0); // 0x8c056c04
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
    // jsr @r0: the delay slot store runs before the callee.
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
    cpu.pr = 0x8c0820da;
    crate::registry::p_8c056c04::run(cpu, mem);

    cpu.r[1] = cpu.r[14]; // mov r14,r1
    cpu.r[1] = cpu.r[1].wrapping_add(24); // add #24,r1
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0 (w6)
    cpu.fr[2] = 1.0; // fldi1 fr2
    cpu.r[7] = cpu.r[14]; // mov r14,r7
    cpu.fr[1] = cpu.fr[2]; // fmov fr2,fr1
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    cpu.r[7] = cpu.r[7].wrapping_add(32); // add #32,r7
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    let _ = mem.write_f32(cpu.r[7], cpu.fr[1]); // fmov fr1,@r7 (w8)
    cpu.r[0] = cpu.r[0].wrapping_add(28); // add #28,r0
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1 (w7)
    cpu.r[6] = cpu.r[14]; // mov r14,r6
    cpu.r[6] = cpu.r[6].wrapping_add(36); // add #36,r6
    cpu.fr[2] = cpu.fr[2] / cpu.fr[1]; // fdiv fr1,fr2
    let _ = mem.write_f32(cpu.r[6], cpu.fr[2]); // fmov fr2,@r6 (w9)
    cpu.r[5] = cpu.r[14]; // mov r14,r5
    cpu.fr[0] = cpu.fr[12]; // fmov fr12,fr0
    cpu.fr[2] = mem.read_f32(cpu.r[7]).unwrap_or(0.0); // fmov @r7,fr2
    cpu.r[5] = cpu.r[5].wrapping_add(8); // add #8,r5
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // fmul fr2,fr0
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]); // fmov fr0,@r5 (w2)
    cpu.fr[12] = cpu.fr[12] + cpu.fr[14]; // fadd fr14,fr12
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap_or(0.0); // fmov @r7,fr0
    cpu.r[4] = cpu.r[4].wrapping_add(12); // add #12,r4
    cpu.fr[1] = cpu.fr[13]; // fmov fr13,fr1
    cpu.fr[12] = cpu.fr[12] * cpu.fr[0]; // fmul fr0,fr12
    let _ = mem.write_f32(cpu.r[4], cpu.fr[12]); // fmov fr12,@r4 (w3)
    cpu.r[3] = cpu.r[14]; // mov r14,r3
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // fmov @r6,fr0
    cpu.fr[1] = cpu.fr[1] + cpu.fr[15]; // fadd fr15,fr1
    cpu.r[3] = cpu.r[3].wrapping_add(16); // add #16,r3
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]); // fmov fr1,@r3 (w4)
    cpu.r[1] = cpu.r[14]; // mov r14,r1
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // fmov @r6,fr0
    cpu.r[1] = cpu.r[1].wrapping_add(20); // add #20,r1
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.fr[13] = cpu.fr[13] * cpu.fr[0]; // fmul fr0,fr13
    let _ = mem.write_f32(cpu.r[1], cpu.fr[13]); // fmov fr13,@r1 (w5)
    cpu.r[0] = cpu.r[0].wrapping_add(40); // add #40,r0
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // fmov @r4,fr1
    cpu.r[14] = cpu.r[14].wrapping_add(44); // add #44,r14
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr0
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // fmov fr1,@r0 (w10)
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr1
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
    let _ = mem.write_f32(cpu.r[14], cpu.fr[1]); // fmov fr1,@r14 (w11)

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    pop_f(cpu, mem, 12); // fmov @r15+,fr12
    pop_f(cpu, mem, 13); // fmov @r15+,fr13
    pop_f(cpu, mem, 14); // fmov @r15+,fr14
    pop_f(cpu, mem, 15); // rts delay slot: fmov @r15+,fr15
}
