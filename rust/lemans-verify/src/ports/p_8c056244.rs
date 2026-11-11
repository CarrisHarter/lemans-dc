//! 0x8c056244 — 2-D line intersection. r4 points at three floats
//! (fr3 = [r4+4], fr4 = [r4+8], fr5 = [r4+12]); [r15] is the output triple.
//! Writes [out] = t-ish value (fr2), [out+4] = fr8, [out+8] = fr6 (max).
//!
//! ```text
//! r3 = [r15]; r1 = r4+4; r0 = r4+8; r4 += 12
//! fr3 = [r1]; fr2 = 0; fr4 = [r0]; fr5 = [r4]        (fr5 load is a bf.s delay slot)
//! fr6 = fr4 > fr3 ? fr4 : fr3; if fr5 > fr6 { fr6 = fr5 }   -> max
//! fr7 = fr4 > fr3 ? fr3 : fr4; if !(fr5 > fr7) { fr7 = fr5 } -> min
//! fr8 = 0; if fr6 != 0 { fr8 = (fr6 - fr7) / fr6 }
//! if fr8 == 0 { fr2 = 0; goto store }               (bra, delay fmov fr0,fr2)
//! fr1 = fr6 - fr7                                    (bf.s delay slot, always)
//! if fr3 == fr6 { fr4 = (fr4 - fr5)/fr1; fr2 = fr4 }
//! else if fr4 == fr6 { fr2 = (fr5 - fr3)/fr1 + [0x8c0562e4] (=2.0) }
//! else if fr5 == fr6 { fr2 = (fr3 - fr4)/fr1 + [0x8c0562e8] (=4.0) }
//! fr1 = [0x8c0562ec] (=60.0); fr0 = 0; fr2 *= fr1
//! if 0 > fr2 { fr0 = [0x8c0562f0] (=360.0); fr2 += fr0 }   T = 0 > fr2
//! store: [r3]=fr2; [r3+4]=fr8; rts; [r3+8]=fr6 (delay slot)
//! ```
//! Side effects the C does not show: r0 = out+4, r3 = out+8, r1 = r4o+4,
//! r4 = r4o+12; fr0/fr1/fr2..fr8 all keep their last computed values (fr1 is
//! *not* written on the fr8==0 early exit); T ends as the final fcmp/gt
//! (0 > fr2), or 1 on the early exit; the last store is the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4o = cpu.r[4];
    let out = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[3] = out;
    cpu.r[1] = r4o.wrapping_add(4); // mov r4,r1 ; add #4,r1
    cpu.r[0] = r4o.wrapping_add(8); // mov r4,r0 ; add #8,r0
    cpu.r[4] = r4o.wrapping_add(12); // add #12,r4
    let fr3 = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0)); // fmov @r1,fr3
    cpu.fr[3] = fr3;
    cpu.fr[2] = 0.0; // fldi0 fr2
    let fr4 = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // fmov @r0,fr4
    cpu.fr[4] = fr4;
    let fr5 = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0)); // fmov @r4,fr5 (bf.s delay slot)
    cpu.fr[5] = fr5;

    cpu.fr[6] = if fr4 > fr3 { fr4 } else { fr3 }; // fcmp/gt fr3,fr4 + fmovs
    if fr5 > cpu.fr[6] {
        cpu.fr[6] = fr5; // fcmp/gt fr6,fr5 ; fmov fr5,fr6
    }
    cpu.fr[7] = if fr4 > fr3 { fr3 } else { fr4 };
    if !(fr5 > cpu.fr[7]) {
        cpu.fr[7] = fr5; // fcmp/gt fr7,fr5 ; bf -> fmov fr5,fr7
    }
    let fr6 = cpu.fr[6];
    let fr7 = cpu.fr[7];

    cpu.fr[8] = 0.0; // fldi0 fr8
    if fr6 != 0.0 {
        // fcmp/eq fr8,fr6 false: fr8 = fr6; fr8 -= fr7; fr8 /= fr6
        cpu.fr[8] = fr6 - fr7;
        cpu.fr[8] /= fr6;
    }
    cpu.fr[0] = 0.0; // fldi0 fr0
    if cpu.fr[8] == 0.0 {
        // fcmp/eq fr0,fr8 true: bra to the store, delay fmov fr0,fr2
        cpu.fr[2] = 0.0;
        cpu.set_t(true);
        let _ = mem.write_u32(out, cpu.fr[2].to_bits()); // fmov fr2,@r3
        let _ = mem.write_u32(out.wrapping_add(4), cpu.fr[8].to_bits()); // fmov fr8,@r0
        cpu.r[0] = out.wrapping_add(4); // mov r3,r0 ; add #4,r0
        cpu.r[3] = out.wrapping_add(8); // add #8,r3
        let _ = mem.write_u32(cpu.r[3], cpu.fr[6].to_bits()); // rts delay slot: fmov fr6,@r3
        return;
    }
    cpu.set_t(false); // fcmp/eq fr0,fr8 false

    cpu.fr[1] = fr6; // fmov fr6,fr1
    cpu.fr[1] = cpu.fr[1] - fr7; // fsub fr7,fr1 (bf.s delay slot, always runs)
    let fr1 = cpu.fr[1];
    if fr3 == fr6 {
        // fcmp/eq fr6,fr3 true
        cpu.fr[4] = fr4 - fr5; // fsub fr5,fr4
        cpu.fr[4] /= fr1; // fdiv fr1,fr4
        cpu.fr[2] = cpu.fr[4]; // bra delay: fmov fr4,fr2
    } else if fr4 == fr6 {
        // fcmp/eq fr6,fr4 true
        cpu.fr[0] = f32::from_bits(mem.read_u32(0x8c0562e4).unwrap_or(0)); // mova + fmov @r0,fr0 = 2.0
        cpu.fr[5] = fr5 - fr3; // fsub fr3,fr5
        cpu.fr[5] /= fr1; // fdiv fr1,fr5
        cpu.fr[5] += cpu.fr[0]; // fadd fr0,fr5
        cpu.fr[2] = cpu.fr[5]; // bra delay: fmov fr5,fr2
    } else if fr5 == fr6 {
        // fcmp/eq fr6,fr5 true
        cpu.fr[0] = f32::from_bits(mem.read_u32(0x8c0562e8).unwrap_or(0)); // = 4.0
        cpu.fr[3] = fr3 - fr4; // fsub fr4,fr3
        cpu.fr[3] /= fr1; // fdiv fr1,fr3
        cpu.fr[3] += cpu.fr[0]; // fadd fr0,fr3
        cpu.fr[2] = cpu.fr[3]; // fmov fr3,fr2
    }
    // else: fr2 keeps 0.0 from the fldi0

    cpu.r[0] = 0x8c0562ec; // mova 0x8c0562ec,r0
    cpu.fr[1] = f32::from_bits(mem.read_u32(0x8c0562ec).unwrap_or(0)); // fmov @r0,fr1 = 60.0
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.fr[2] = cpu.fr[2] * cpu.fr[1]; // fmul fr1,fr2
    let t = 0.0f32 > cpu.fr[2]; // fcmp/gt fr2,fr0 : T = fr0 > fr2
    if t {
        cpu.fr[0] = f32::from_bits(mem.read_u32(0x8c0562f0).unwrap_or(0)); // mova + fmov = 360.0
        cpu.fr[2] += cpu.fr[0]; // fadd fr0,fr2
    }
    cpu.set_t(t);

    cpu.r[0] = out; // mov r3,r0
    let _ = mem.write_u32(out, cpu.fr[2].to_bits()); // fmov fr2,@r3
    cpu.r[0] = out.wrapping_add(4); // add #4,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[8].to_bits()); // fmov fr8,@r0
    cpu.r[3] = out.wrapping_add(8); // add #8,r3
    let _ = mem.write_u32(cpu.r[3], cpu.fr[6].to_bits()); // rts delay slot: fmov fr6,@r3
}
