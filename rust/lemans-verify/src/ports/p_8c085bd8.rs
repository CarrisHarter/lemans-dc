//! 0x8c085bd8 — rebuild a transform block from its own lanes.
//!
//! Works on the block at r4: fr2 = [r4+8] - [r4+44]; two ratios
//! ([r4+16]/[r4+24] and [r4+20]/[r4+24]) are scaled by fr2 and, together with
//! -fr2, form a 3-float vector on the 48-byte stack frame. 0x8c0b4f94
//! (vector add, dst passed at [r15]) adds that vector to [r4+0..8]; the two
//! leading lanes are negated and the whole vector is scaled by 0.8 through
//! 0x8c0b4ef0 (fr4 = the literal at 0x8c085cb0); a second 0x8c0b4f94 call
//! combines the copies. The result replaces [r4+0..8] and [r4+24] is scaled
//! by the literal at 0x8c085cb8 (-0.8) in place.
//!
//! Both callees are ported; the oracle executes their real bytes, so the tree
//! must agree end to end.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c085bd8..8c085bdc — save r14/pr, open a 48-byte frame.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(48);

    let base = cpu.r[15];
    let p = cpu.r[4];
    cpu.r[14] = p;

    // 8c085be0..8c085bf6 — fr2 = [p+8] - [p+44].
    cpu.r[0] = p.wrapping_add(44);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[6] = p.wrapping_add(8);
    cpu.fr[2] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] - cpu.fr[0];

    // 8c085bf8..8c085c08 — [frame+36] = ([p+16] / [p+24]) * fr2.
    cpu.r[5] = p.wrapping_add(16);
    cpu.r[1] = p.wrapping_add(20);
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[3] = p.wrapping_add(24);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[2]; // fmul fr2,fr1
    cpu.r[2] = base.wrapping_add(36);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);

    // 8c085c0a..8c085c18 — [frame+40] = ([p+20] / [p+24]) * fr2.
    cpu.r[0] = base.wrapping_add(12);
    cpu.r[2] = base.wrapping_add(40);
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[2]; // fmul fr2,fr1
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);

    // 8c085c1a..8c085c20 — [frame+44] = -fr2.
    cpu.fr[2] = -cpu.fr[2]; // fneg fr2
    cpu.r[2] = base.wrapping_add(44);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[2]);

    // 8c085c22..8c085c2c — push dst = frame+12; 0x8c0b4f94(p, frame+36).
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[5] = cpu.r[15].wrapping_add(40); // delay slot of `jsr`
    cpu.r[0] = mem.read_u32(0x8c085cac).unwrap_or(0);
    cpu.pr = 0x8c085c2c;
    crate::registry::p_8c0b4f94::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // 8c085c2e..8c085c4a — flip frame[36]; frame[24] = frame[12].
    cpu.r[2] = base.wrapping_add(12);
    cpu.fr[0] = mem.read_f32(base.wrapping_add(36)).unwrap_or(0.0);
    cpu.r[4] = base;
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.fr[0] = -cpu.fr[0]; // fneg fr0
    cpu.r[0] = 0x8c085cb0; // mova
    let _ = mem.write_f32(base.wrapping_add(36), cpu.fr[0]);
    cpu.r[4] = base.wrapping_add(36);
    let _ = mem.write_f32(base.wrapping_add(24), cpu.fr[1]);

    // 8c085c4c..8c085c5e — frame[28] = frame[16].
    cpu.r[2] = base.wrapping_add(16);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[1] = mem.read_u32(0x8c085cb4).unwrap_or(0);
    cpu.r[2] = base.wrapping_add(40);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[2] = base.wrapping_add(28);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);

    // 8c085c60..8c085c6c — frame[40] = -frame[40].
    cpu.fr[0] = -cpu.fr[0]; // fneg fr0
    cpu.r[2] = base.wrapping_add(20);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[2] = base.wrapping_add(40);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);

    // 8c085c6e..8c085c72 — frame[32] = frame[20].
    cpu.r[2] = base.wrapping_add(32);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);

    // 8c085c74..8c085c76 — 0x8c0b4ef0(frame+36) with fr4 = [0x8c085cb0].
    cpu.pr = 0x8c085c78;
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // delay slot of `jsr`
    crate::registry::p_8c0b4ef0::run(cpu, mem);

    // 8c085c78..8c085c88 — push dst = frame; 0x8c0b4f94(frame+24, frame+36).
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[5] = cpu.r[15].wrapping_add(40);
    cpu.r[4] = cpu.r[15].wrapping_add(28); // delay slot of `jsr`
    cpu.r[0] = mem.read_u32(0x8c085cac).unwrap_or(0);
    cpu.pr = 0x8c085c88;
    crate::registry::p_8c0b4f94::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // 8c085c8a..8c085ca2 — copy frame[0..12] to p, then p[24] *= -0.8.
    cpu.r[4] = cpu.r[15];
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = 0x8c085cb8; // mova
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(p, cpu.r[3]);
    let _ = mem.write_u32(p.wrapping_add(4), cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(8), cpu.r[1]);
    cpu.r[14] = cpu.r[14].wrapping_add(24);
    cpu.fr[1] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[14], cpu.fr[1]);

    // 8c085ca4..8c085caa — drop the frame, restore pr/r14; `rts` delay slot.
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
