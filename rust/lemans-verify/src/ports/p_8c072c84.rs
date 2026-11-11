//! 0x8c072c84 — FUN_8c072c84: insert a 3-short triplet into a counted short
//! array (240 instructions). ABI: r4 = triplet (short[3]), r5 = array base,
//! r6 = &count (u32), r7 = flag byte.
//!
//! CFG sketch:
//!
//! ```text
//! entry: r9 = *r6 (count); arr[count] = 3; r9 = count-1; r13 = count+1;
//!        r3 = count-1; T = (r9 >= 0)            (cmp/pz r9)
//!        delay slot of bt.s always runs: r14 = 0
//!   ├─ T=0 (count==0) ──────────────► path A @0x8c072e26:
//!   │     arr[count+1..3] = p0,p1,p2; *r6 = count+4; r0 = 0
//!   └─ T=1 ─► 0x8c072cb4: s = arr[count-1] (zero-extended), r8 = p0, r1 = s
//!        ├─ p0 == s ─────────────────► path B @0x8c072cc4:
//!        │     if (r7&0xff) { r13--; r14 = 1 }
//!        │     arr[r13..r13+2] = p0,p1,p2; *r6 = r13+3; r0 = r14
//!        ├─ p1 == s ─────────────────► path C @0x8c072cf6 (delay slot spills
//!        │     r9=p1 to sp+4): same shape, order p1,p2,p0
//!        ├─ p2 == s ─────────────────► path D @0x8c072d32 (delay spills r9=p2
//!        │     to sp+0): same shape, order p2,p0,p1
//!        └─ else ─► path E @0x8c072d6e: r7 = -2 (pool 0x8c072e64);
//!              s2 = arr[count-2]
//!              ├─ s2 == p0 ─► E1 @0x8c072d7e:  order p0,p1,p2
//!              ├─ s2 == p1 ─► E2 @0x8c072dac:  order p1,p2,p0 (reads sp+4)
//!              ├─ s2 == p2 ─► E3 @0x8c072dd8:  order p2,p0,p1 (r9 = p2)
//!              └─ else ─────► E4 @0x8c072dfe:  order p0,p1,p2
//!              (E never decrements r13, never sets r14: *r6 = count+4, r0 = 0)
//! all paths converge at 0x8c072e4e: *r6 = r13; r0 = r14; epilogue restores
//! r8..r14 and r15 (7 pushes + add #-8 fully undone).
//! ```
//!
//! Side effects the decompiled C does not show: the `bt.s` delay slot
//! `mov #0,r14` runs on BOTH entry paths, so the "bool" return r0 is 0 unless a
//! match branch sets r14=1 (only when r7&0xff != 0); on path E r7 is clobbered
//! to 0xfffffffe (pool constant) before being reused as index; final r1/r3/r7
//! hold the last triplet word and doubled indices. T is NOT the last cmp/tst:
//! every `shll` sets T to the operand's old MSB, so the final T is the MSB of
//! r13-1 (the operand of the last `shll r3` on every path). r8..r14 and r15 are
//! restored by the epilogue pops, so they keep their input values.

use sh4_oracle::{Cpu, Memory};

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).expect("8c072c84: load out of range")
}

#[inline]
fn wr(mem: &mut Memory, addr: u32, v: u32) {
    mem.write_u32(addr, v).expect("8c072c84: store out of range")
}

/// mov.w load: sign-extended into the 32-bit register.
#[inline]
fn rdw(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).expect("8c072c84: word load out of range") as i16 as i32 as u32
}

/// mov.w store: low 16 bits.
#[inline]
fn wrw(mem: &mut Memory, addr: u32, v: u32) {
    mem.write_u16(addr, v as u16).expect("8c072c84: word store out of range");
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r = cpu.r;
    // Body r15 = r15_in - 36 (7 pushes + add #-8); the epilogue undoes it
    // fully, so the final r15 equals the input r15. Spill slots: sp+0, sp+4.
    let sp = r[15].wrapping_sub(36);

    let count = rd(mem, r[6]);
    r[9] = count;
    r[0] = count;
    r[3] = count;
    let mut r13 = count.wrapping_add(1); // r13
    r[0] = r[0] << 1; // shll r0: count*2
    r[9] = r[9].wrapping_sub(1); // count-1
    wrw(mem, r[5].wrapping_add(r[0]), 3); // arr[count] = 3 (r1 = 3)
    let t_entry = (r[9] as i32) >= 0; // cmp/pz r9
    r[3] = r[3].wrapping_add(0xffff_ffff); // add r8,r3 (r8 = -1): count-1
    let mut r14 = 0u32; // mov #0,r14 — delay slot of bt.s, runs on both paths

    if !t_entry {
        // Path A (count == 0): 0x8c072cae → bra 0x8c072e26 (delay nop).
        r[7] = r13;
        r13 = r13 << 1;
        r[1] = rdw(mem, r[4]); // p0
        r[0] = r13;
        r[7] = r[7].wrapping_add(1);
        wrw(mem, r[5].wrapping_add(r[0]), r[1]); // arr[count+1] = p0
        r[3] = r[7];
        r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
        r[7] = r[7] << 1;
        r[3] = r[3].wrapping_add(1);
        r[1] = r[0];
        r[0] = r[7];
        wrw(mem, r[5].wrapping_add(r[0]), r[1]); // arr[count+2] = p1
        r13 = r[3];
        r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
        r[3] = r[3] << 1;
        r13 = r13.wrapping_add(1);
        r[1] = r[0];
        r[0] = r[3];
        wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay slot: arr[count+3] = p2
    } else {
        // count >= 1: 0x8c072cb4..
        r[3] = r[3] << 1; // (count-1)*2
        r[0] = r[3];
        r[12] = rdw(mem, r[4]); // p0 (sign-extended)
        r[0] = rdw(mem, r[5].wrapping_add(r[0])); // s = arr[count-1] (sext)
        r[8] = r[12] & 0xffff; // extu.w r12,r8
        r[1] = r[0] & 0xffff; // extu.w r0,r1

        if r[1] == r[8] {
            // Path B @0x8c072cc4: p0 == arr[count-1]
            if r[7] & 0xff != 0 {
                r13 = r13.wrapping_sub(1);
                r14 = 1;
            }
            r[7] = r13;
            r13 = r13 << 1;
            r[0] = r13;
            wrw(mem, r[5].wrapping_add(r[0]), r[12]); // arr[r13] = p0
            r[7] = r[7].wrapping_add(1);
            r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
            r[3] = r[7];
            r[3] = r[3].wrapping_add(1);
            r[1] = r[0];
            r[7] = r[7] << 1;
            r[0] = r[7];
            wrw(mem, r[5].wrapping_add(r[0]), r[1]); // arr[r7] = p1
            r13 = r[3];
            r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
            r[3] = r[3] << 1;
            r13 = r13.wrapping_add(1);
            r[1] = r[0];
            r[0] = r[3];
            wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay slot: arr[r3] = p2
        } else if {
            // Path C @0x8c072cf6: p1 == arr[count-1]
            r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
            let r11 = r[0];
            r[9] = r11 & 0xffff; // extu.w r11,r9
            wr(mem, sp.wrapping_add(4), r[9]); // delay slot of bf.s: spill p1
            r[1] == r[9]
        } {
            let r11 = rdw(mem, r[4].wrapping_add(2));
            if r[7] & 0xff != 0 {
                r13 = r13.wrapping_sub(1);
                r14 = 1;
            }
            r[7] = r13;
            r13 = r13 << 1;
            r[0] = r13;
            wrw(mem, r[5].wrapping_add(r[0]), r11); // arr[r13] = p1
            r[7] = r[7].wrapping_add(1);
            r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
            r[3] = r[7];
            r[3] = r[3].wrapping_add(1);
            r[1] = r[0];
            r[7] = r[7] << 1;
            r[0] = r[7];
            wrw(mem, r[5].wrapping_add(r[0]), r[1]); // arr[r7] = p2
            r13 = r[3];
            r[0] = rdw(mem, r[4]); // p0
            r[3] = r[3] << 1;
            r13 = r13.wrapping_add(1);
            r[1] = r[0];
            r[0] = r[3];
            wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay slot: arr[r3] = p0
        } else if {
            // Path D @0x8c072d32: p2 == arr[count-1]
            r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
            let r10 = r[0];
            r[9] = r10 & 0xffff; // extu.w r10,r9
            wr(mem, sp, r[9]); // delay slot of bf.s: spill p2
            r[1] == r[9]
        } {
            let r10 = rdw(mem, r[4].wrapping_add(4));
            if r[7] & 0xff != 0 {
                r13 = r13.wrapping_sub(1);
                r14 = 1;
            }
            r[7] = r13;
            r13 = r13 << 1;
            r[0] = r13;
            wrw(mem, r[5].wrapping_add(r[0]), r10); // arr[r13] = p2
            r[7] = r[7].wrapping_add(1);
            r[3] = r[7];
            r[7] = r[7] << 1;
            r[0] = rdw(mem, r[4]); // p0
            r[3] = r[3].wrapping_add(1);
            r[1] = r[0];
            r[0] = r[7];
            wrw(mem, r[5].wrapping_add(r[0]), r[1]); // arr[r7] = p0
            r13 = r[3];
            r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
            r[3] = r[3] << 1;
            r13 = r13.wrapping_add(1);
            r[1] = r[0];
            r[0] = r[3];
            wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay slot: arr[r3] = p1
        } else {
            // Path E @0x8c072d6e: compare against arr[count-2].
            r[7] = rd(mem, 0x8c07_2e64); // 0xfffffffe
            let s2 = rdw(mem, r[5].wrapping_add(r[3]).wrapping_add(r[7])) & 0xffff;
            r[0] = s2; // extu.w r0,r0
            if r[0] == r[8] {
                // E1 @0x8c072d7e: s2 == p0, order p0,p1,p2
                r[7] = r13;
                r13 = r13 << 1;
                r[0] = r13;
                wrw(mem, r[5].wrapping_add(r[0]), r[12]); // p0
                r[7] = r[7].wrapping_add(1);
                r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
                r[3] = r[7];
                r[3] = r[3].wrapping_add(1);
                r[1] = r[0];
                r[7] = r[7] << 1;
                r[0] = r[7];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // p1
                r13 = r[3];
                r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
                r[3] = r[3] << 1;
                r13 = r13.wrapping_add(1);
                r[1] = r[0];
                r[0] = r[3];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay: p2
            } else if r[0] == rd(mem, sp.wrapping_add(4)) {
                // E2 @0x8c072dac: s2 == p1 (spilled), order p1,p2,p0
                let r11 = rdw(mem, r[4].wrapping_add(2));
                r[7] = r13;
                r13 = r13 << 1;
                r[0] = r13;
                wrw(mem, r[5].wrapping_add(r[0]), r11); // p1
                r[7] = r[7].wrapping_add(1);
                r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
                r[3] = r[7];
                r[3] = r[3].wrapping_add(1);
                r[1] = r[0];
                r[7] = r[7] << 1;
                r[0] = r[7];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // p2
                r13 = r[3];
                r[0] = rdw(mem, r[4]); // p0
                r[3] = r[3] << 1;
                r13 = r13.wrapping_add(1);
                r[1] = r[0];
                r[0] = r[3];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay: p0
            } else if r[0] == r[9] {
                // E3 @0x8c072dd8: s2 == p2 (r9 still holds it), order p2,p0,p1
                let r10 = rdw(mem, r[4].wrapping_add(4));
                r[7] = r13;
                r13 = r13 << 1;
                r[0] = r13;
                wrw(mem, r[5].wrapping_add(r[0]), r10); // p2
                r[7] = r[7].wrapping_add(1);
                r[3] = r[7];
                r[7] = r[7] << 1;
                r[0] = rdw(mem, r[4]); // p0
                r[3] = r[3].wrapping_add(1);
                r[1] = r[0];
                r[0] = r[7];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // p0
                r13 = r[3];
                r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
                r[3] = r[3] << 1;
                r13 = r13.wrapping_add(1);
                r[1] = r[0];
                r[0] = r[3];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay: p1
            } else {
                // E4 @0x8c072dfe: no match, order p0,p1,p2
                r[7] = r13;
                r13 = r13 << 1;
                r[0] = r13;
                wrw(mem, r[5].wrapping_add(r[0]), r[12]); // p0
                r[7] = r[7].wrapping_add(1);
                r[0] = rdw(mem, r[4].wrapping_add(2)); // p1
                r[3] = r[7];
                r[3] = r[3].wrapping_add(1);
                r[1] = r[0];
                r[7] = r[7] << 1;
                r[0] = r[7];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // p1
                r13 = r[3];
                r[0] = rdw(mem, r[4].wrapping_add(4)); // p2
                r[3] = r[3] << 1;
                r13 = r13.wrapping_add(1);
                r[1] = r[0];
                r[0] = r[3];
                wrw(mem, r[5].wrapping_add(r[0]), r[1]); // delay: p2
            }
        }
    }

    // Convergence 0x8c072e4e: *r6 = r13; r0 = r14. Epilogue: r15 net
    // unchanged; r8..r14 popped back to their input values, so only r0..r7
    // are written back. Final T = MSB of the last `shll r3` operand = MSB(r13-1).
    wr(mem, r[6], r13);
    r[0] = r14;
    let t_last = (r13.wrapping_sub(1) >> 31) != 0;
    for i in 0..8 {
        cpu.r[i] = r[i];
    }
    cpu.set_t(t_last);
}
