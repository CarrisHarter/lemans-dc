//! 0x8c024f58 — assemble two half-angle quads into a four-word result.
//!
//! `r4` points at a record whose f32 fields at +4 and +8 are the two angles;
//! the first stack argument (`[r15]` at entry) is the four-float destination.
//!
//! The frame (after `add #-80,r15`) is laid out as: 0 = 0x8c0b4000's output,
//! 16 = quad for the +8 angle, 32 = quad for the +4 angle, 48/64 = copies of
//! those quads handed to 0x8c0b4000 as its `a`/`b` operand arrays.
//!
//! 0x8c025038 is called twice: selector 1 with the +4 angle writes the quad at
//! frame+32, selector 2 with the +8 angle writes the quad at frame+16. Then
//! 0x8c0b4000(r4 = frame+48, r5 = frame+64) writes its four products back to
//! frame+0, and the routine copies those four words to the destination.
//!
//! Side effects: exactly sixteen bytes stored at the destination pointer plus
//! the stack frame traffic (synthetic stack window). r13/r14/pr and the four
//! call-preserved FP registers are restored; the callees own their leftovers
//! (r0-r7 and fr0-fr11 are clobbered, and the T bit ends up as 0x8c025038 left
//! it).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov #92,r0 ; mov.l r13,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[0] = 92;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(80);
    // mov.l @(r0,r15),r14 — first stack argument is the destination pointer.
    cpu.r[14] = mem.read_u32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[13] = cpu.r[4];

    // Push frame+32 as the destination of the first (selector 1) quad.
    cpu.r[0] = cpu.r[15].wrapping_add(32);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = cpu.r[13].wrapping_add(4);
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = mem.read_u32(0x8c025030).unwrap_or(0); // 0x8c025038
    cpu.pr = 0x8c024f78;
    cpu.r[4] = 1; // delay slot of `jsr`
    crate::registry::p_8c025038::run(cpu, mem); // half-angle quad, selector 1
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Copy that quad into the `b` operand array at frame+64, reordering none.
    cpu.r[2] = 36;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // quad[1]
    cpu.r[0] = 32;
    cpu.fr[1] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0); // quad[0]
    cpu.r[2] = 64;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // b[0]
    cpu.r[0] = cpu.r[15];
    cpu.r[2] = 40;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // quad[2]
    cpu.r[0] = cpu.r[0].wrapping_add(16);
    cpu.r[2] = 68;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // b[1]
    cpu.r[2] = 44;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // quad[3]
    cpu.r[2] = 72;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // b[2]
    cpu.r[2] = 76;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // b[3]
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // frame+16 for the next quad

    cpu.r[13] = cpu.r[13].wrapping_add(8);
    cpu.fr[4] = mem.read_f32(cpu.r[13]).unwrap_or(0.0);
    cpu.r[0] = mem.read_u32(0x8c025030).unwrap_or(0); // 0x8c025038
    cpu.pr = 0x8c024fb8;
    cpu.r[4] = 2; // delay slot of `jsr`
    crate::registry::p_8c025038::run(cpu, mem); // half-angle quad, selector 2
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Copy the second quad into the `a` operand array at frame+48.
    cpu.r[2] = 20;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // quad[1]
    cpu.r[0] = 16;
    cpu.fr[1] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0); // quad[0]
    cpu.r[2] = 48;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // a[0]
    cpu.r[0] = cpu.r[15];
    cpu.r[2] = 24;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // quad[2]
    cpu.r[2] = 52;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // a[1]
    cpu.r[2] = 28;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // quad[3]
    cpu.r[2] = 56;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // a[2]
    cpu.r[2] = 60;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // a[3]
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // frame+0 destination

    cpu.r[5] = cpu.r[15];
    cpu.r[4] = cpu.r[15];
    cpu.r[0] = mem.read_u32(0x8c025034).unwrap_or(0); // 0x8c0b4000
    cpu.r[5] = cpu.r[5].wrapping_add(68); // delay slot: b = frame+64
    cpu.pr = 0x8c024ff8;
    cpu.r[4] = cpu.r[4].wrapping_add(52); // delay slot: a = frame+48
    crate::registry::p_8c0b4000::run(cpu, mem); // a x b -> frame+0
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Copy the four products to the destination pointer.
    cpu.r[2] = 4;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[3] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // dst[1]
    cpu.r[1] = cpu.r[14];
    cpu.r[0] = cpu.r[14];
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // dst[2]
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.r[2] = 12;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // dst[3]
    cpu.r[2] = 0;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // dst[0]
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[3]);
    cpu.r[14] = cpu.r[14].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[2]);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[1]);

    cpu.r[15] = cpu.r[15].wrapping_add(80);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
