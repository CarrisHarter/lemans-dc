//! 0x8c0ed6b8 — scale and clamp a struct's float against a power-of-two limit.
//!
//! Calls 0x8c010d8c (which classifies the float it gets from 0x8c0148d0 /
//! 0x8c013e94, from the caller's fr4) and uses its fr0 as the scale:
//!
//! ```text
//! fr3 = f32[r4+4] * fr0
//! r1 = 1 shld r5            (negative r5 shifts right)
//! fpul = r1 ; fr2 = (float)r1
//! if r5 != 8: fr0 = 256.0 / fr2 ; fr3 = fr3 / fr0
//! if (r6 & 0xff) == 0: fr3 = (fr2 - 1.0) - fr3
//! fr0 = clamp(fr3, 0.0, fr2)  (0 if fr3 < 0, fr2 if fr3 > fr2, else fr3)
//! ```
//!
//! The clamp is the last thing that sets T: the `0.0 > fr3` arm leaves it set,
//! the `fr3 > fr2` arm leaves it equal to that comparison. Final r0 is the
//! zero-extended low byte of the incoming r6, r1 is the shifted word and fpul
//! holds the same word. r12-r14 and pr come back from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0ed6b8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0ed6ba: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0ed6bc: mov r6,r14
    cpu.r[14] = cpu.r[6];
    // 0x8c0ed6be: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0ed6c0: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c0ed6c2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0ed6c4: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 0x8c0ed6c6: mov.l 0x8c0ed718,r0 (0x8c010d8c) ; jsr @r0 (pr = 0x8c0ed6cc)
    cpu.r[0] = mem.read_u32(0x8c0ed718).unwrap_or(0x8c010d8c);
    cpu.pr = 0x8c0ed6cc;
    crate::registry::p_8c010d8c::run(cpu, mem);

    // 0x8c0ed6cc: add #4,r12
    cpu.r[12] = cpu.r[12].wrapping_add(4);
    // 0x8c0ed6ce: fmov @r12,fr3
    cpu.fr[3] = mem.read_f32(cpu.r[12]).unwrap_or(0.0);
    // 0x8c0ed6d0: mov #1,r1
    cpu.r[1] = 1;
    // 0x8c0ed6d2: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0ed6d4: shld r13,r1 — signed shift amount: negative shifts right
    let amount = cpu.r[13] as i32;
    cpu.r[1] = if amount >= 0 {
        cpu.r[1] << (amount as u32 & 31)
    } else {
        cpu.r[1] >> (amount.wrapping_neg() as u32 & 31)
    };
    // 0x8c0ed6d6: cmp/eq #8,r0
    cpu.set_t(cpu.r[0] == 8);
    // 0x8c0ed6d8: fmul fr0,fr3
    cpu.fr[3] *= cpu.fr[0];
    // 0x8c0ed6da: lds r1,fpul
    cpu.fpul = cpu.r[1];
    // 0x8c0ed6dc: bt.s 0x8c0ed6e8 ; 0x8c0ed6de: float fpul,fr2 (delay slot)
    let skip_div = cpu.t();
    cpu.fr[2] = cpu.fpul as i32 as f32;
    if !skip_div {
        // 0x8c0ed6e0: mova 0x8c0ed71c,r0
        cpu.r[0] = 0x8c0ed71c;
        // 0x8c0ed6e2: fmov @r0,fr0
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c0ed6e4: fdiv fr2,fr0
        cpu.fr[0] /= cpu.fr[2];
        // 0x8c0ed6e6: fdiv fr0,fr3
        cpu.fr[3] /= cpu.fr[0];
    }

    // 0x8c0ed6e8: extu.b r14,r0
    cpu.r[0] = cpu.r[14] & 0xff;
    // 0x8c0ed6ea: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c0ed6ec: bf 0x8c0ed6f8
    if cpu.t() {
        // 0x8c0ed6ee: fmov fr2,fr0
        cpu.fr[0] = cpu.fr[2];
        // 0x8c0ed6f0: fldi1 fr1
        cpu.fr[1] = 1.0;
        // 0x8c0ed6f2: fsub fr1,fr0
        cpu.fr[0] -= cpu.fr[1];
        // 0x8c0ed6f4: fsub fr3,fr0
        cpu.fr[0] -= cpu.fr[3];
        // 0x8c0ed6f6: fmov fr0,fr3
        cpu.fr[3] = cpu.fr[0];
    }

    // 0x8c0ed6f8: fldi0 fr0
    cpu.fr[0] = 0.0;
    // 0x8c0ed6fa: fcmp/gt fr3,fr0 — T = 0.0 > fr3
    let below = cpu.fr[0] > cpu.fr[3];
    cpu.set_t(below);
    if !below {
        // 0x8c0ed702: fcmp/gt fr2,fr3 — T = fr3 > fr2
        let above = cpu.fr[3] > cpu.fr[2];
        cpu.set_t(above);
        if above {
            // 0x8c0ed706: bra 0x8c0ed70c ; 0x8c0ed708: fmov fr2,fr0 (delay)
            cpu.fr[0] = cpu.fr[2];
        } else {
            // 0x8c0ed70a: fmov fr3,fr0
            cpu.fr[0] = cpu.fr[3];
        }
    }

    // 0x8c0ed70c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0ed70e: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0ed710: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0ed712: rts ; 0x8c0ed714: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
