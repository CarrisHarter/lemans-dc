//! 0x8c0d8cfc — 1-D table lookup/interpolation over a struct at r4 with the
//! query float in fr4; returns a float in fr0.
//!
//! CFG:
//!   hi = [r4+120]; if (fr4 > hi) fr5 = 1.0, goto check;      // path A
//!   lo = [r4+124]; if (fr4 > lo) {                            // path C
//!       idx = [r4+76]; r3 = idx<<2; r5 = r4+88+r3;
//!       fr4 = (fr4 - lo) * [r4+128]; fr5 = [r5] + fr4; r0 = r4+0x80;
//!   } else fr5 = 0.0;                                         // path B
//! check: fr0 = 1.0; if (fr5 == 1.0) return 1.0;              // T=1
//!   fr0 = 0.0; if (fr5 == 0.0) { r4 += 76; r0 = [r4]-1;
//!       fpul = r0; return (float)r0; }                        // T=1
//!   r6 = [r4+76] (count); r1 = r4+92; r3 = 1; r5 = 4;
//!   loop: while (r6 > r3) { fr1 = [r1+r5]; if (fr5 > fr1) {
//!       i = r3; r0 = i<<2; r4 = r4+88+r0; r1 += r0;
//!       fr3 = [r4]; fpul = i; fr2 = [r1];
//!       return (fr3 - fr5) / (fr3 - fr2) + (float)i; }        // T=1, rts delay fadd
//!       r5 += 4; r3 += 1; }
//!   return 0.0;                                               // T=0
//!
//! Hidden side effects: fr4 is clobbered on path C only; r0 ends as
//! r4+120 / r4+124 / r4+0x80 / count-1 / r4+76 / 4*i per path; r1 = 0x80 on
//! the check paths but r4+92(+4i) on loop paths; r3 = idx<<2 on check paths,
//! the loop counter on loop paths; r5 = r4+88+4*idx on check paths, 4*i on
//! loop paths; fpul is written on the fr5==0 and found paths; fr1 holds the
//! last table float probed on the miss path; T is 1 except on the loop-miss
//! exit (0). No stack use; r15 untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let ld = |a: u32| f32::from_bits(mem.read_u32(a).unwrap());
    let fr4_in = cpu.fr[4];
    let hi = ld(base + 120);
    if fr4_in > hi {
        // fcmp/gt fr0,fr4 -> T = fr4 > fr0; bf not taken -> bra path
        // path A: bra delay slot fldi1 fr5; fr5 == 1.0 -> immediate rts
        cpu.r[0] = base + 120;
        cpu.fr[5] = 1.0;
        cpu.fr[0] = 0.0; // bf.s not taken, but its delay slot fldi0 fr0 still runs
        cpu.set_t(true); // fcmp/eq fr0,fr5 with fr0 = 1.0
        return;
    }
    cpu.r[0] = base + 124;
    let lo = ld(base + 124);
    cpu.fr[1] = lo; // fmov @r0,fr1 at 0x8c0d8d0e; survives on the check paths
    let mut fr4;
    let fr5: f32;
    if fr4_in > lo {
        // fcmp/gt fr1,fr4 -> T = fr4 > fr1; bf not taken -> path C
        let idx = mem.read_u32(base + 76).unwrap();
        let r3 = idx.wrapping_shl(2); // shll2 r3
        let r5 = base.wrapping_add(88).wrapping_add(r3);
        let fr0 = ld(base + 128);
        fr4 = fr4_in - lo; // fsub fr1,fr4
        fr4 = fr4 * fr0; // fmul fr0,fr4
        fr5 = ld(r5) + fr4; // fmov + fadd in the bra delay slot
        cpu.r[1] = 0x80; // mov.l 0x8c0d8d98,r1
        cpu.r[0] = base.wrapping_add(0x80); // add r1,r0 with r0 = r4
        cpu.r[3] = r3;
        cpu.r[5] = r5;
        cpu.fr[4] = fr4;
    } else {
        fr5 = 0.0; // fldi0 fr5
    }
    // 0x8c0d8d36: fldi1 fr0; fcmp/eq fr0,fr5
    if fr5 == 1.0 {
        cpu.fr[0] = 0.0; // bf.s delay slot fldi0 fr0 runs even though not taken
        cpu.fr[5] = fr5;
        cpu.set_t(true);
        return; // rts at 0x8c0d8d3e, delay slot nop
    }
    cpu.fr[0] = 0.0; // bf.s delay slot: fldi0 fr0
    cpu.fr[5] = fr5;
    if fr5 == 0.0 {
        // fcmp/eq fr0,fr5 true -> rts float(count-1)
        let r4 = base + 76;
        let r0 = mem.read_u32(r4).unwrap().wrapping_sub(1);
        cpu.r[4] = r4;
        cpu.r[0] = r0;
        cpu.fpul = r0; // lds r0,fpul
        cpu.fr[0] = r0 as i32 as f32; // float fpul,fr0 in the rts delay slot
        cpu.set_t(true);
        return;
    }
    // loop at 0x8c0d8d62
    let count = mem.read_u32(base + 76).unwrap();
    cpu.r[6] = count;
    cpu.r[1] = base + 92;
    cpu.r[0] = base + 76;
    let mut r3: u32 = 1;
    let mut r5: u32 = 4; // bra delay slot: mov #4,r5
    loop {
        // 0x8c0d8d8e: cmp/gt r3,r6 -> T = (r6 > r3), signed
        if !((count as i32) > (r3 as i32)) {
            cpu.r[3] = r3;
            cpu.r[5] = r5;
            cpu.set_t(false);
            return; // rts at 0x8c0d8d92, fr0 stays 0.0
        }
        cpu.r[0] = cpu.r[1].wrapping_add(r5); // mov r1,r0; add r5,r0 at 0x8c0d8d62
        let fr1 = ld(cpu.r[0]);
        cpu.fr[1] = fr1;
        // 0x8c0d8d68: fcmp/gt fr1,fr5 -> T = fr5 > fr1
        if fr5 > fr1 {
            cpu.set_t(r3 >> 30 & 1 != 0); // shll2 r0 sets T from bit 30
            let r0 = r3.wrapping_shl(2); // mov r3,r0; shll2 r0
            cpu.r[0] = r0;
            cpu.r[4] = base.wrapping_add(88).wrapping_add(r0);
            cpu.r[1] = cpu.r[1].wrapping_add(r0);
            let fr3 = ld(cpu.r[4]);
            cpu.fpul = r3; // lds r3,fpul
            let fr2 = ld(cpu.r[1]);
            cpu.fr[2] = fr2;
            let fr3b = fr3 - fr2; // fsub fr2,fr3
            cpu.fr[3] = fr3b;
            let mut fr0 = fr3 - fr5; // fmov fr3,fr0; fsub fr5,fr0
            cpu.fr[1] = r3 as i32 as f32; // float fpul,fr1
            fr0 = fr0 / fr3b; // fdiv fr3,fr0
            fr0 = fr0 + cpu.fr[1]; // rts delay slot: fadd fr1,fr0
            cpu.fr[0] = fr0;
            cpu.r[3] = r3;
            cpu.r[5] = r5;
            // T is the shll2's carry, not 1: nothing after it touches the flag.
            return;
        }
        r5 = r5.wrapping_add(4);
        r3 = r3.wrapping_add(1);
    }
}
