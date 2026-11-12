//! 0x8c0c5e84 — step a clamped scalar toward a target, weighted by gate flags.
//!
//! ```text
//! push fr15, fr14, r14, r13, pr          (all restored at exit)
//! r13 = param (object); r14 = *[0x8c264da8]; r1 = *[0x8c23dab0] + 0xa4
//! 0x8c074ca0(r14, 0, 8)                  ; delay slot: fr14 = f32[r1]
//! if ((u8)ret == 0) fr15 += 0x8c074c54(r14, 0, 0)   ; returns float in fr0
//! if ((u8)0x8c074ca0(r14, 0, 2) != 0) fr15 -= 1.0
//! if ((u8)0x8c074ca0(r14, 0, 3) != 0) fr15 += 1.0
//! fr2 = (f32[obj+0x4c] - f32[obj+0x48]) * fr14 / 3.0 * fr15
//! f32[obj+0x50] += fr2
//! f32[obj+0x50] = max(f32[obj+0x50], f32[obj+0x48])
//! f32[obj+0x50] = min(f32[obj+0x50], f32[obj+0x4c])
//! ```
//!
//! Registers at exit: r0 = obj+0x4c, r1 = obj+0x50, r3 = obj+0x48,
//! r4 = obj+0x4c; r5/r6/r7 are whatever the last gate call left; r13, r14 and
//! the pushed FPU registers end at their entry values. T is the last
//! `fcmp/gt`'s (f32[obj+0x50] > f32[obj+0x4c] before the min clamp).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let obj = cpu.r[4]; // mov r4,r13 (r13 is pushed and restored)
    let saved_r14 = cpu.r[14];
    let saved_fr14 = cpu.fr[14];
    let saved_fr15 = cpu.fr[15];
    let saved_pr = cpu.pr;

    // mov.l 0x8c0c5f44,r0 ; mov.l @r0,r14 ; r7 = 0 ; r5 = 0 ; r6 = 8
    cpu.r[7] = 0;
    let pool_obj = mem.read_u32(0x8C0C5F44).unwrap_or(0); // 0x8c264da8
    let pool_base = mem.read_u32(0x8C0C5F48).unwrap_or(0); // 0x8c23dab0
    cpu.r[5] = cpu.r[7];
    cpu.r[14] = mem.read_u32(pool_obj).unwrap_or(0);
    cpu.r[6] = 8;
    cpu.r[1] = mem.read_u32(pool_base).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    let a4 = mem.read_u32(0x8C0C5F4C).unwrap_or(0); // 0xa4
    cpu.fr[15] = 0.0; // fldi0 fr15
    cpu.r[1] = cpu.r[1].wrapping_add(a4);
    cpu.r[0] = mem.read_u32(0x8C0C5F50).unwrap_or(0); // 0x8c074ca0
    // jsr delay slot: fmov @r1,fr14 — reads through the pre-call r1.
    cpu.fr[14] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0));
    cpu.pr = 0x8C0C5EAC;
    crate::registry::p_8c074ca0::run(cpu, mem);

    cpu.r[0] &= 0xFF; // extu.b
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        // bf not taken: 0x8c074c54(r14, 0, 0), float result in fr0.
        cpu.r[0] = mem.read_u32(0x8C0C5F54).unwrap_or(0); // 0x8c074c54
        cpu.r[6] = 0;
        cpu.r[5] = cpu.r[6];
        cpu.r[4] = cpu.r[14]; // jsr delay slot: mov r14,r4
        cpu.pr = 0x8C0C5EBC;
        crate::registry::p_8c074c54::run(cpu, mem);
        cpu.fr[15] = cpu.fr[15] + cpu.fr[0]; // fadd fr0,fr15
    }

    // Gate 2: 0x8c074ca0(r14, 0, 2)
    cpu.r[0] = mem.read_u32(0x8C0C5F50).unwrap_or(0); // 0x8c074ca0
    cpu.r[7] = 0;
    cpu.r[5] = cpu.r[7];
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = 2; // jsr delay slot: mov #2,r6
    cpu.pr = 0x8C0C5ECA;
    crate::registry::p_8c074ca0::run(cpu, mem);
    cpu.r[0] &= 0xFF; // extu.b
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.fr[15] = cpu.fr[15] - cpu.fr[0]; // fsub fr0,fr15
    }

    // Gate 3: 0x8c074ca0(r14, 0, 3)
    cpu.r[0] = mem.read_u32(0x8C0C5F50).unwrap_or(0); // 0x8c074ca0
    cpu.r[7] = 0;
    cpu.r[5] = cpu.r[7];
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = 3; // jsr delay slot: mov #3,r6
    cpu.pr = 0x8C0C5EDE;
    crate::registry::p_8c074ca0::run(cpu, mem);
    cpu.r[0] &= 0xFF; // extu.b
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.fr[15] = cpu.fr[15] + cpu.fr[0]; // fadd fr0,fr15
    }

    // mov r13,r4 ; add #76 ; mov r13,r3 ; add #72 ; mova 0x8c0c5f58
    cpu.r[4] = obj.wrapping_add(76); // obj+0x4c
    cpu.r[3] = obj.wrapping_add(72); // obj+0x48
    cpu.r[0] = 0x8C0C5F58;
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0)); // fmov @r4,fr2
    cpu.r[1] = obj;
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap_or(0)); // fmov @r3,fr1
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // 3.0
    cpu.fr[2] = cpu.fr[2] - cpu.fr[1]; // fsub fr1,fr2
    cpu.fr[2] = cpu.fr[2] * cpu.fr[14]; // fmul fr14,fr2
    cpu.fr[2] = cpu.fr[2] / cpu.fr[0]; // fdiv fr0,fr2
    cpu.r[1] = cpu.r[1].wrapping_add(80); // obj+0x50
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0)); // fmov @r1,fr0
    cpu.fr[2] = cpu.fr[2] * cpu.fr[15]; // fmul fr15,fr2
    cpu.fr[0] = cpu.fr[0] + cpu.fr[2]; // fadd fr2,fr0
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits()); // fmov fr0,@r1
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap_or(0)); // fmov @r3,fr0
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0)); // fmov @r1,fr1
    cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
    if cpu.fr[0] > cpu.fr[1] {
        cpu.fr[1] = cpu.fr[0]; // bt: fmov fr0,fr1
    }

    // mov r13,r1 ; add #80 ; mov r13,r0 ; fmov fr1,@r1 ; add #76,r0
    cpu.r[1] = obj.wrapping_add(80);
    cpu.r[0] = obj;
    let _ = mem.write_u32(cpu.r[1], cpu.fr[1].to_bits()); // fmov fr1,@r1
    cpu.r[0] = cpu.r[0].wrapping_add(76); // r0 = obj+0x4c (final)
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // fmov @r0,fr0
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0)); // fmov @r1,fr1
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
    if cpu.fr[1] > cpu.fr[0] {
        cpu.fr[1] = cpu.fr[0]; // bt: fmov fr0,fr1
    }
    // add #80,r13 ; fmov fr1,@r13 — r13 is restored by the epilogue.
    let _ = mem.write_u32(obj.wrapping_add(80), cpu.fr[1].to_bits());

    // Epilogue: lds.l @r15+,pr ; mov.l @r15+,r13 ; mov.l @r15+,r14 ;
    // fmov @r15+,fr14 ; rts ; fmov @r15+,fr15
    cpu.r[14] = saved_r14;
    cpu.fr[14] = saved_fr14;
    cpu.fr[15] = saved_fr15;
    cpu.pr = saved_pr;
}
