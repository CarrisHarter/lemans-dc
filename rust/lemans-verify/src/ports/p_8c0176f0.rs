//! 0x8c0176f0 — two-string matcher with an optional `!digits!` capture.
//!
//! ABI: r4 = pattern, r5 = string, r6 = int* out. Returns 0/1 in r0.
//! The original stores 0 to *r6 on entry and, on the `!…!` capture path,
//! stores the parsed decimal there before returning 1.
//!
//! Side effects the decompiled C does not show: r5 is clobbered with -48
//! (delay slot `mov #-48,r5` at 0x8c0177778) on the capture path and with
//! sext8(*r4) in the tail loop (0x8c01782e); r1/r3/r4 are left pointing past
//! the consumed input; delay-slot `add #1` instructions fire even on taken
//! branches; the T bit is 1 on every return-1 and on the `*r4==0` mismatch
//! exit, 0 on the two compare-mismatch exits.

use sh4_oracle::{Cpu, Memory};

/// `mov.b @Rm,Rn` in this oracle sign-extends the byte into the register.
#[inline]
fn sx(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

/// Epilogue 0x8c01783c is a net identity on r14/r15 (`mov r14,r15` + `rts` +
/// `mov.l @r15+,r14`), so the port leaves them alone and just sets the
/// registers the body touched plus T.
#[inline]
fn finish(cpu: &mut Cpu, r0: u32, r1: u32, r3: u32, r4: u32, r5: u32, t: bool) {
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    cpu.set_t(t);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r6 = cpu.r[6];
    let mut r4 = cpu.r[4];
    let mut r5 = cpu.r[5];
    let mut r3 = cpu.r[3];
    let mut r1: u32;

    // 0x8c0176f4-0x8c0176f6: r0 = 0; *r6 = 0
    mem.write_u32(r6, 0).unwrap();

    // 0x8c0176f8: tst r5,r5; bf.s 0x8c017702 (delay: mov r5,r1)
    if r5 == 0 {
        // fall through: bra 0x8c01783c (delay: mov #1,r0)
        finish(cpu, 1, 0, r3, r4, r5, true);
        return;
    }
    r1 = r5;

    // 0x8c017702-0x8c017732: optional "P[C][V]v" prefix on the string.
    if sx(mem, r5) == 80 {
        // 0x8c01770a
        r1 = r1.wrapping_add(1);
        if sx(mem, r1) == 67 {
            // 0x8c017714
            r1 = r1.wrapping_add(1);
        }
        if sx(mem, r1) == 86 {
            // 0x8c01771e
            r1 = r1.wrapping_add(1);
        }
        if sx(mem, r1) == 118 {
            // 'v': accept when the pattern starts with 'P' or '*'
            let c4 = sx(mem, r4);
            if c4 == 80 || c4 == 42 {
                // 0x8c017734: bra 0x8c01783c (delay: mov #1,r0)
                finish(cpu, 1, r1, r3, r4, r5, true);
                return;
            }
        }
        // 0x8c017738: mov r5,r1
        r1 = r5;
    }

    // 0x8c01773a/0x8c01773c: merge point A, r0 = sext8(*r4)
    let c = sx(mem, r4);
    if c == 33 || c == 42 {
        // ---- '!'/'*' pattern path (0x8c01774a) ----
        r3 = sx(mem, r4); // 0x8c01774a
        let r0 = sx(mem, r1); // 0x8c01774c
        r4 = r4.wrapping_add(1); // 0x8c017750
        r1 = r1.wrapping_add(1); // delay slot of bt.s at 0x8c017756 (always runs)
        if r3 != r0 {
            // 0x8c01775a: bra epilogue (delay: mov #0,r0), T = 0 from the cmp
            finish(cpu, 0, r1, r3, r4, r5, false);
            return;
        }
        // LOOP_B at 0x8c01775e
        loop {
            let a = sx(mem, r4); // *r4
            let b = sx(mem, r1); // *r1 (via r3)
            r3 = b; // 0x8c017762: mov.b @r3,r3
            r1 = r1.wrapping_add(1); // delay of bf.s at 0x8c01776a (always runs)
            if a != b {
                // MISMATCH at 0x8c01779c: skip r4 past two '!'
                loop {
                    let c = sx(mem, r4);
                    r4 = r4.wrapping_add(1); // delay of bf.s (always runs)
                    if c == 33 {
                        break;
                    }
                }
                loop {
                    let c = sx(mem, r4);
                    r4 = r4.wrapping_add(1);
                    if c == 33 {
                        break;
                    }
                }
                let c = sx(mem, r4); // 0x8c0177b0
                if c == 0 {
                    // 0x8c0177b8: bra epilogue (delay: mov #0,r0); tst set T=1
                    finish(cpu, 0, r1, r3, r4, r5, true);
                    return;
                }
                // 0x8c0177bc + delay slot: r1 = r5 + 1, back to LOOP_B
                r1 = r5.wrapping_add(1);
                continue;
            }
            r4 = r4.wrapping_add(1); // delay of bf.s at 0x8c017770 (always runs)
            if a == 33 {
                // 0x8c017774: r3 = 0; delay of bra at 0x8c017776: r5 = -48
                r3 = 0;
                r5 = (-48i32) as u32;
                // LOOP_C at 0x8c01778e: parse decimal until '!'
                loop {
                    let c = sx(mem, r4);
                    if c == 33 {
                        // 0x8c017796: *r6 = r3; bra epilogue (delay: mov #1,r0)
                        mem.write_u32(r6, r3).unwrap();
                        finish(cpu, 1, r1, r3, r4, r5, true);
                        return;
                    }
                    // body 0x8c01777a: r1 = r3*10 + sext8(*r4) - 48; r3 = r1; r4++
                    let d = sx(mem, r4);
                    let mut v = r3;
                    v = v << 2; // shll2 r1
                    v = v.wrapping_add(r3); // add r3,r1
                    v = v << 1; // shll r1 (sets T; overwritten by the head cmp)
                    v = v.wrapping_add(d); // add r0,r1
                    v = v.wrapping_add(r5); // add r5,r1 (-48)
                    r1 = v; // mov r3,r1 ... mov r1,r3
                    r3 = v;
                    r4 = r4.wrapping_add(1);
                }
            }
        }
    } else {
        // ---- plain path: LOOP_D at 0x8c017806 ----
        'outer: loop {
            let p = sx(mem, r4); // 0x8c017806: r3 = *r4
            if p == 80 || p == 82 {
                // 'P' or 'R'
                let s = sx(mem, r1); // 0x8c017812
                if s == p {
                    // D2 at 0x8c0177c2
                    r1 = r1.wrapping_add(1); // 0x8c0177c2
                    let c1 = sx(mem, r1); // 0x8c0177c4
                    r4 = r4.wrapping_add(1); // delay of bf.s at 0x8c0177ca (always)
                    if c1 == 67 {
                        // 'C' on the string: maybe skip a 'C' on the pattern too
                        if sx(mem, r4) == 67 {
                            r4 = r4.wrapping_add(1); // 0x8c0177d6
                        }
                        r1 = r1.wrapping_add(1); // 0x8c0177d8
                    }
                    r3 = sx(mem, r4); // 0x8c0177da/0x8c0177dc
                    if r3 == 67 {
                        // 0x8c0177e4: bra epilogue (delay: mov #0,r0), T = 1
                        finish(cpu, 0, r1, r3, r4, r5, true);
                        return;
                    }
                    if sx(mem, r1) == 86 {
                        // 'V' on the string: maybe skip a 'V' on the pattern
                        if r3 == 86 {
                            r4 = r4.wrapping_add(1); // 0x8c0177f6
                        }
                        r1 = r1.wrapping_add(1); // 0x8c0177f8
                    }
                    if sx(mem, r4) == 86 {
                        // 0x8c017802: bra epilogue (delay: mov #0,r0), T = 1
                        finish(cpu, 0, r1, r3, r4, r5, true);
                        return;
                    }
                    continue 'outer; // 0x8c017800 -> 0x8c017806
                }
                // 0x8c01781c: bra 0x8c01782e
            }
            // LOOP_E at 0x8c01782e
            loop {
                r5 = sx(mem, r4); // 0x8c01782e: mov.b @r4,r5 (r5 clobbered)
                let s = sx(mem, r1); // 0x8c017830
                r3 = r5; // 0x8c017832: exts.b r5,r3
                if s == r3 {
                    // 0x8c017820: r0 = r5; tst r0,r0
                    if r5 == 0 {
                        // 0x8c017826: bra epilogue (delay: mov #1,r0), T = 1
                        finish(cpu, 1, r1, r3, r4, r5, true);
                        return;
                    }
                    r4 = r4.wrapping_add(1); // 0x8c01782a
                    r1 = r1.wrapping_add(1); // 0x8c01782c
                } else {
                    // 0x8c01783a: mov #0,r0; T = 0 from the cmp
                    finish(cpu, 0, r1, r3, r4, r5, false);
                    return;
                }
            }
        }
    }
}
