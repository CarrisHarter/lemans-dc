//! 0x8c082178 — initialise a 48-byte projection block at [r4].
//!
//! ```text
//! push r14/r13/pr
//! [r4] = 0.0f ; [r4+4] = 0.0f (jsr delay slot)
//! r4 += 24 ; jsr 0x8c056c04     ; copies two dwords from 0x8c23dd60
//! [r4+0x20] = 1.0f / [r4+0x18]
//! [r4+0x24] = 1.0f / [r4+0x1c]
//! [r4+0x08] = [r4+0x20] * 0.0f
//! [r4+0x0c] = [r4+0x20] * 640.0f
//! [r4+0x10] = [r4+0x24] * 480.0f
//! [r4+0x14] = [r4+0x24] * 0.0f
//! [r4+0x28] = [r4+0x0c] - [r4+0x08]
//! [r4+0x2c] = [r4+0x10] - [r4+0x14]
//! ```
//!
//! All arithmetic is single precision. The two 0.0f products are not foldable
//! to zero: the reciprocals can be inf/NaN, and the oracle multiplies exactly.
//! Side effects the C does not show: the callee leaves r0/r1, the fp0/fp1/fp2
//! residues matter, and r13/r14/pr are restored from the stack.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // mov.l r13,@-r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]); // fmov fr0,@r14
    cpu.r[1] = cpu.r[14].wrapping_add(4); // mov r14,r1 ; add #4,r1
    cpu.r[0] = mem.read_u32(0x8c082204).unwrap_or(0); // literal 0x8c056c04
    cpu.r[4] = cpu.r[14].wrapping_add(24); // add #24,r4
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // jsr delay slot: fmov fr0,@r1
    cpu.pr = 0x8c082190; // jsr @r0 sets pr to the return address
    crate::registry::p_8c056c04::run(cpu, mem);

    cpu.r[1] = cpu.r[14].wrapping_add(24);
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr2
    cpu.fr[1] = 1.0; // fldi1 fr1
    cpu.r[3] = cpu.r[14].wrapping_add(32);
    cpu.fr[0] = cpu.fr[1]; // fmov fr1,fr0
    cpu.fr[0] /= cpu.fr[2]; // fdiv fr2,fr0
    cpu.r[0] = cpu.r[14].wrapping_add(28);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
    cpu.r[13] = cpu.r[14].wrapping_add(36);
    cpu.fr[1] /= cpu.fr[0]; // fdiv fr0,fr1
    let _ = mem.write_f32(cpu.r[13], cpu.fr[1]); // fmov fr1,@r13

    cpu.r[7] = cpu.r[14].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr0
    cpu.fr[2] = 0.0; // fldi0 fr2
    cpu.fr[0] *= cpu.fr[2]; // fmul fr2,fr0
    let _ = mem.write_f32(cpu.r[7], cpu.fr[0]); // fmov fr0,@r7
    cpu.r[0] = 0x8c082208; // mova 0x8c082208,r0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 640.0f
    cpu.r[6] = cpu.r[14].wrapping_add(12);
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr1
    cpu.r[0] = 0x8c08220c; // mova 0x8c08220c,r0
    cpu.fr[1] *= cpu.fr[0]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[6], cpu.fr[1]); // fmov fr1,@r6
    cpu.r[5] = cpu.r[14].wrapping_add(16);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 480.0f
    cpu.fr[1] = mem.read_f32(cpu.r[13]).unwrap_or(0.0); // fmov @r13,fr1
    cpu.r[4] = cpu.r[14].wrapping_add(20);
    cpu.fr[1] *= cpu.fr[0]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[5], cpu.fr[1]); // fmov fr1,@r5

    cpu.r[3] = cpu.r[14].wrapping_add(40);
    cpu.fr[0] = mem.read_f32(cpu.r[13]).unwrap_or(0.0); // fmov @r13,fr0
    cpu.r[1] = cpu.r[14].wrapping_add(44);
    cpu.fr[0] *= cpu.fr[2]; // fmul fr2,fr0
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // fmov fr0,@r4
    cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // fmov @r6,fr1
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap_or(0.0); // fmov @r7,fr0
    cpu.fr[1] -= cpu.fr[0]; // fsub fr0,fr1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]); // fmov fr1,@r3
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr1
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // fmov @r4,fr0
    cpu.fr[1] -= cpu.fr[0]; // fsub fr0,fr1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
