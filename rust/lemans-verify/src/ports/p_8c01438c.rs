//! 0x8c01438c — single-precision arcsin: classifies fr4, folds |x| into
//! tan(asin(x)/2) with the reciprocal/square-root identities, calls the
//! atan kernel 0x8c014480, then doubles via 0x8c014810 (r4 = 1).
//!
//! ```text
//! push fr15, pr ; add #-8,r15
//! r0=4 ; fr5=fr4 ; [r15+4]=fr5 ; r3=[r15+4]
//! cmp/pz r3 ; bt.s +2 ; mov #0,r4 ; (not taken) mov #8,r4   ; r4 = sign flag
//! fr3=0.0 ; fcmp/eq fr3,fr5 ; r7=4 ; bf.s +4 ; mov #1,r6
//! ; fr4==0: bra cls ; or r7,r4 (r4 |= 4 zero flag)
//! ; else: r2=bits&0x7fffffff, r3=0x7fffffff, r1=0x7f800000
//! ;       cmp/eq inf -> r0=2, r4|=2 ; else fcmp/eq fr5,fr5 -> r4|=1 NaN
//! cls: r5=0x8c1ccf98 ; tst r4,r6 ; bt sign ; fr0=NaN ; epilogue
//! ;     sign: r3=r15, r2=0x80000000, [r15]=fr4, r1=[r3], tst sign
//! ;           if negative, [r15] = -[r15]
//! ;     fr3=[r15], fr5=1.0, fcmp/gt fr5,fr3 (|x|>1) -> bra out with fr5=NaN
//! ;     fcmp/eq fr5,fr3 (|x|==1) -> fr5=±π/2 by r4&8, bra out
//! ;     tst r7,r4 (zero) -> bra out with fr5=fr4
//! ;     r2=0.5 ; fcmp/gt fr3,fr2 (0.5>|x|) ->
//! ;        small: t=x/(1+sqrt(1-x*x)) ; large: t=(1-sqrt(1-x*x))/x
//! ;     jsr 0x8c014480 (fr4=t) ; jsr 0x8c014810 (fr4=fr0, r4=1)
//! ; out: fr0=fr5 ; add #8,r15 ; lds.l pr ; rts ; fmov @r15+,fr15
//! ```
//!
//! Side effects the C does not show: r4 is the class/sign bitmap (1 NaN,
//! 2 Inf, 4 zero, 8 negative); the sign test works on the raw bits stored at
//! [r15], so -0.0 keeps its sign through the zero arm; r5 ends as the NaN
//! literal pointer on every arm; r6=1 and r7=4 are set before the class
//! branch; fr5 carries the result on every exit except the NaN arm, which
//! loads fr0 directly; T is left by the last flag instruction of the taken
//! arm (or of the called kernel chain on the two polynomial arms).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01438c: fmov fr15,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    // 0x8c01438e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c014390: add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // 0x8c014392: mov #4,r0
    cpu.r[0] = 4;
    // 0x8c014394: fmov fr4,fr5
    cpu.fr[5] = cpu.fr[4];
    // 0x8c014396: fmov fr5,@(r0,r15)
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[5]);
    // 0x8c014398: mov.l @(4,r15),r3
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    // 0x8c01439a: cmp/pz r3 ; 0x8c01439e: mov #0,r4 (delay)
    cpu.set_t((cpu.r[3] as i32) >= 0);
    cpu.r[4] = 0;
    if !cpu.t() {
        // 0x8c0143a0: mov #8,r4
        cpu.r[4] = 8;
    }
    // 0x8c0143a2: fldi0 fr3
    cpu.fr[3] = 0.0;
    // 0x8c0143a4: fcmp/eq fr3,fr5
    cpu.set_t(cpu.fr[3] == cpu.fr[5]);
    // 0x8c0143a6: mov #4,r7 ; 0x8c0143aa: mov #1,r6 (bf.s delay)
    cpu.r[7] = 4;
    cpu.r[6] = 1;
    if cpu.t() {
        // 0x8c0143ac: bra cls ; 0x8c0143ae: or r7,r4 (delay)
        cpu.r[4] |= cpu.r[7];
        class_arm(cpu, mem);
        return;
    }
    // 0x8c0143b0: mov.l @(4,r15),r2
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    // 0x8c0143b2: mov.l 0x8c01445c,r3 (0x7fffffff)
    cpu.r[3] = mem.read_u32(0x8c01445c).unwrap_or(0);
    // 0x8c0143b4: mov.l 0x8c014460,r1 (0x7f800000)
    cpu.r[1] = mem.read_u32(0x8c014460).unwrap_or(0);
    // 0x8c0143b6: and r3,r2 ; 0x8c0143b8: cmp/eq r1,r2
    cpu.r[2] &= cpu.r[3];
    cpu.set_t(cpu.r[2] == cpu.r[1]);
    // 0x8c0143ba: bf 0x8c0143c2
    if cpu.t() {
        // 0x8c0143bc: mov #2,r0 ; 0x8c0143be: bra cls ; 0x8c0143c0: or r0,r4
        cpu.r[0] = 2;
        cpu.r[4] |= cpu.r[0];
        class_arm(cpu, mem);
        return;
    }
    // 0x8c0143c2: fcmp/eq fr5,fr5
    cpu.set_t(cpu.fr[5] == cpu.fr[5]);
    // 0x8c0143c4: bt cls
    if !cpu.t() {
        // 0x8c0143c6: or r6,r4 (NaN flag)
        cpu.r[4] |= cpu.r[6];
    }
    class_arm(cpu, mem);
}

/// 0x8c0143c8: NaN early-out, then the sign fold and the |x| range tests.
fn class_arm(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0143c8: mov.l 0x8c014464,r5 (0x8c1ccf98)
    cpu.r[5] = mem.read_u32(0x8c014464).unwrap_or(0);
    // 0x8c0143ca: tst r4,r6 ; 0x8c0143cc: bt 0x8c0143d8
    cpu.set_t(cpu.r[4] & cpu.r[6] == 0);
    if !cpu.t() {
        // 0x8c0143ce: fmov @r5,fr0 (canonical NaN)
        cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
        epilogue(cpu, mem);
        return;
    }
    // 0x8c0143d8: mov r15,r3
    cpu.r[3] = cpu.r[15];
    // 0x8c0143da: mov.l 0x8c014468,r2 (0x80000000)
    cpu.r[2] = mem.read_u32(0x8c014468).unwrap_or(0);
    // 0x8c0143dc: fmov fr4,@r15
    let _ = mem.write_f32(cpu.r[15], cpu.fr[4]);
    // 0x8c0143de: mov.l @r3,r1 ; 0x8c0143e0: tst r2,r1
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.set_t(cpu.r[2] & cpu.r[1] == 0);
    // 0x8c0143e2: bt 0x8c0143ea
    if !cpu.t() {
        // 0x8c0143e4: fmov @r15,fr3 ; 0x8c0143e6: fneg fr3 ; 0x8c0143e8: fmov fr3,@r15
        cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
        cpu.fr[3] = -cpu.fr[3];
        let _ = mem.write_f32(cpu.r[15], cpu.fr[3]);
    }
    // 0x8c0143ea: mov.l 0x8c01446c,r3 (1.0)
    cpu.r[3] = mem.read_u32(0x8c01446c).unwrap_or(0);
    // 0x8c0143ec: fmov @r15,fr3 (|x|)
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    // 0x8c0143ee: fmov @r3,fr5 (1.0)
    cpu.fr[5] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    // 0x8c0143f0: fcmp/gt fr5,fr3 (fr3 > fr5)
    cpu.set_t(cpu.fr[3] > cpu.fr[5]);
    // 0x8c0143f2: bf 0x8c0143f8
    if cpu.t() {
        // 0x8c0143f4: bra out ; 0x8c0143f6: fmov @r5,fr5 (delay, NaN)
        cpu.fr[5] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
        out(cpu, mem);
        return;
    }
    // 0x8c0143f8: fcmp/eq fr5,fr3 (|x| == 1)
    cpu.set_t(cpu.fr[3] == cpu.fr[5]);
    // 0x8c0143fa: bf 0x8c01440a
    if cpu.t() {
        // 0x8c0143fc: mov #8,r2 ; 0x8c0143fe: mov.l 0x8c014470,r1 (π/2)
        cpu.r[2] = 8;
        cpu.r[1] = mem.read_u32(0x8c014470).unwrap_or(0);
        // 0x8c014400: tst r2,r4 ; 0x8c014402: bt.s out ; 0x8c014404: fmov @r1,fr5
        cpu.set_t(cpu.r[2] & cpu.r[4] == 0);
        cpu.fr[5] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        if !cpu.t() {
            // 0x8c014406: bra out ; 0x8c014408: fneg fr5 (delay)
            cpu.fr[5] = -cpu.fr[5];
        }
        out(cpu, mem);
        return;
    }
    // 0x8c01440a: tst r7,r4 (zero flag)
    cpu.set_t(cpu.r[7] & cpu.r[4] == 0);
    // 0x8c01440c: bt 0x8c014412
    if !cpu.t() {
        // 0x8c01440e: bra out ; 0x8c014410: fmov fr4,fr5 (delay, keeps ±0)
        cpu.fr[5] = cpu.fr[4];
        out(cpu, mem);
        return;
    }
    // 0x8c014412: mov.l 0x8c014474,r2 (0.5)
    cpu.r[2] = mem.read_u32(0x8c014474).unwrap_or(0);
    // 0x8c014414: fmov @r15,fr3
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    // 0x8c014416: fmov @r2,fr2 (0.5)
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    // 0x8c014418: fcmp/gt fr3,fr2 (fr2 > fr3)
    cpu.set_t(cpu.fr[2] > cpu.fr[3]);
    // 0x8c01441a: bf 0x8c014430
    if cpu.t() {
        // |x| < 0.5: t = x / (1 + sqrt(1 - x*x))
        // 0x8c01441c: fmov fr4,fr1 ; 0x8c01441e: fmul fr4,fr1
        cpu.fr[1] = cpu.fr[4];
        cpu.fr[1] = cpu.fr[1] * cpu.fr[4];
        // 0x8c014420: fmov fr5,fr0 ; 0x8c014422: fmov fr4,fr15
        cpu.fr[0] = cpu.fr[5];
        cpu.fr[15] = cpu.fr[4];
        // 0x8c014424: fsub fr1,fr0
        cpu.fr[0] = cpu.fr[0] - cpu.fr[1];
        // 0x8c014426: fmov fr5,fr1 ; 0x8c014428: fsqrt fr0 ; 0x8c01442a: fadd fr0,fr1
        cpu.fr[1] = cpu.fr[5];
        cpu.fr[0] = cpu.fr[0].sqrt();
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
        // 0x8c01442c: bra 0x8c014442 ; 0x8c01442e: fdiv fr1,fr15 (delay)
        cpu.fr[15] = cpu.fr[15] / cpu.fr[1];
    } else {
        // |x| >= 0.5 (incl. 0.5): t = (1 - sqrt(1 - x*x)) / x
        // 0x8c014430: fmov fr4,fr1 ; 0x8c014432: fmul fr4,fr1
        cpu.fr[1] = cpu.fr[4];
        cpu.fr[1] = cpu.fr[1] * cpu.fr[4];
        // 0x8c014434: fmov fr5,fr0 ; 0x8c014436: fsub fr1,fr0
        cpu.fr[0] = cpu.fr[5];
        cpu.fr[0] = cpu.fr[0] - cpu.fr[1];
        // 0x8c014438: fmov fr5,fr1 ; 0x8c01443a: fsqrt fr0 ; 0x8c01443c: fsub fr0,fr1
        cpu.fr[1] = cpu.fr[5];
        cpu.fr[0] = cpu.fr[0].sqrt();
        cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
        // 0x8c01443e: fmov fr1,fr15 ; 0x8c014440: fdiv fr4,fr15
        cpu.fr[15] = cpu.fr[1];
        cpu.fr[15] = cpu.fr[15] / cpu.fr[4];
    }
    call_chain(cpu, mem);
}

/// 0x8c014442: 2*atan(tan(asin(|x|)/2)) via the atan kernel and the scaler.
fn call_chain(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014442: mov.l 0x8c014478,r3 (0x8c014480)
    cpu.r[3] = mem.read_u32(0x8c014478).unwrap_or(0);
    // 0x8c014444: jsr @r3 ; 0x8c014446: fmov fr15,fr4 (delay)
    cpu.fr[4] = cpu.fr[15];
    cpu.pr = 0x8c014448;
    crate::registry::p_8c014480::run(cpu, mem);
    // 0x8c014448: mov.l 0x8c01447c,r2 (0x8c014810)
    cpu.r[2] = mem.read_u32(0x8c01447c).unwrap_or(0);
    // 0x8c01444a: fmov fr0,fr4
    cpu.fr[4] = cpu.fr[0];
    // 0x8c01444c: jsr @r2 ; 0x8c01444e: mov #1,r4 (delay)
    cpu.r[4] = 1;
    cpu.pr = 0x8c014450;
    crate::registry::p_8c014810::run(cpu, mem);
    // 0x8c014450: fmov fr0,fr5
    cpu.fr[5] = cpu.fr[0];
    out(cpu, mem);
}

/// 0x8c014452: common exit.
fn out(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014452: fmov fr5,fr0
    cpu.fr[0] = cpu.fr[5];
    epilogue(cpu, mem);
}

/// 0x8c014454..0x8c01445a: unwind and return.
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014454: add #8,r15 ; 0x8c014456: lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c014458: rts ; 0x8c01445a: fmov @r15+,fr15 (delay)
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
