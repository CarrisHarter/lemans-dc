//! 0x8c18598c — chained float threshold check.
//!
//! All flag registers start at 1 (r9 via the bt.s delay slot). Each block is
//! guarded by `extu.b flag,r0; tst; bf skip`: it runs only when the flag is 0,
//! and on a failed comparison clears the NEXT flag with `mov #0,rX`. Pre-check:
//! r9 = 0 iff *(r4+40) >= 0.99 AND *(r4+44) >= 0.99 (fcmp/gt is T = fr[n] >
//! fr[m]). Blocks compare *(r4+48) >= 0.99; *(r4+56/60/64) <= 0.01;
//! *(r4+8/12/16) >= 0.99; *(r4+24/28/32) <= 0.01, the last one after r4 += 32.
//! Epilogue: r0 = extu.b r1, T = (r0 == 0), movt r0 → r0 = T = (r1 == 0).
//! r9..r14 are saved/restored; on success the flag register becomes r4+off.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let saved: Vec<u32> = (9..=14).map(|n| cpu.r[n]).collect();
    let c99 = f32::from_bits(mem.read_u32(0x8c185ab8).unwrap());
    let c01 = f32::from_bits(mem.read_u32(0x8c185abc).unwrap());
    let ld = |a: u32| f32::from_bits(mem.read_u32(a).unwrap());

    cpu.fr[0] = ld(r4 + 40); // fmov @r1,fr0 (r1 = r4+40)
    cpu.fr[1] = c99; // fmov @r0,fr1 (r0 = mova 0x8c185ab8)
    cpu.r[1] = 1;
    for n in [3usize, 5, 6, 7, 10, 11, 12, 13, 14] {
        cpu.r[n] = 1;
    }
    cpu.r[9] = 1; // bt.s delay slot: mov r1,r9

    // pre-check: T = c99 > val40, then maybe c99 > val44; r9 = 0 iff both fail
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 (05 f1): T = fr1 > fr0
    if !cpu.t() {
        // bt.s not taken: r0 = r4+44, reload fr0, compare again
        cpu.r[0] = r4 + 44;
        cpu.fr[0] = ld(r4 + 44);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if !cpu.t() {
            cpu.r[9] = 0; // bt not taken: mov #0,r9
        }
    }

    // block 1: runs iff r9 == 0; val *(r4+48), T = c99 > val; fail (T=0) clears r10
    cpu.set_t(cpu.r[9] & 0xff == 0); // extu.b r9,r0; tst r0,r0
    if cpu.t() {
        cpu.r[9] = r4 + 48;
        cpu.r[0] = 0x8c185ab8; // mova
        cpu.fr[1] = ld(r4 + 48);
        cpu.fr[0] = c99;
        cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0 (15 f0): T = fr0 > fr1
        if !cpu.t() {
            cpu.r[10] = 0;
        }
    }
    // block 2: runs iff r10 == 0; val *(r4+56), T = val > c01; fail clears r11
    cpu.set_t(cpu.r[10] & 0xff == 0);
    if cpu.t() {
        cpu.r[10] = r4 + 56;
        cpu.r[0] = 0x8c185abc;
        cpu.fr[1] = ld(r4 + 56);
        cpu.fr[0] = c01;
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 (05 f1): T = fr1 > fr0
        if !cpu.t() {
            cpu.r[11] = 0;
        }
    }
    // block 3: runs iff r11 == 0; val *(r4+60); fail clears r12
    cpu.set_t(cpu.r[11] & 0xff == 0);
    if cpu.t() {
        cpu.r[10] = r4 + 60;
        cpu.r[0] = 0x8c185abc;
        cpu.fr[1] = ld(r4 + 60);
        cpu.fr[0] = c01;
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if !cpu.t() {
            cpu.r[12] = 0;
        }
    }
    // block 4: runs iff r12 == 0; val *(r4+64); fail clears r13
    cpu.set_t(cpu.r[12] & 0xff == 0);
    if cpu.t() {
        cpu.r[10] = r4 + 64;
        cpu.r[0] = 0x8c185abc;
        cpu.fr[1] = ld(r4 + 64);
        cpu.fr[0] = c01;
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if !cpu.t() {
            cpu.r[13] = 0;
        }
    }
    // block 5: runs iff r13 == 0; val *(r4+8), T = c99 > val; fail clears r14
    cpu.set_t(cpu.r[13] & 0xff == 0);
    if cpu.t() {
        cpu.r[10] = r4 + 8;
        cpu.r[0] = 0x8c185ab8;
        cpu.fr[1] = ld(r4 + 8);
        cpu.fr[0] = c99;
        cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0 (15 f0)
        if !cpu.t() {
            cpu.r[14] = 0;
        }
    }
    // block 6: runs iff r14 == 0; val *(r4+12); fail clears r7
    cpu.set_t(cpu.r[14] & 0xff == 0);
    if cpu.t() {
        cpu.r[10] = r4 + 12;
        cpu.r[0] = 0x8c185ab8;
        cpu.fr[1] = ld(r4 + 12);
        cpu.fr[0] = c99;
        cpu.set_t(cpu.fr[0] > cpu.fr[1]);
        if !cpu.t() {
            cpu.r[7] = 0;
        }
    }
    // block 7: runs iff r7 == 0; val *(r4+16); fail clears r6
    cpu.set_t(cpu.r[7] & 0xff == 0);
    if cpu.t() {
        cpu.r[7] = r4 + 16;
        cpu.r[0] = 0x8c185ab8;
        cpu.fr[1] = ld(r4 + 16);
        cpu.fr[0] = c99;
        cpu.set_t(cpu.fr[0] > cpu.fr[1]);
        if !cpu.t() {
            cpu.r[6] = 0;
        }
    }
    // block 8: runs iff r6 == 0; val *(r4+24), T = val > c01; fail clears r5
    cpu.set_t(cpu.r[6] & 0xff == 0);
    if cpu.t() {
        cpu.r[6] = r4 + 24;
        cpu.r[0] = 0x8c185abc;
        cpu.fr[1] = ld(r4 + 24);
        cpu.fr[0] = c01;
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 (05 f1)
        if !cpu.t() {
            cpu.r[5] = 0;
        }
    }
    // block 9: runs iff r5 == 0; val *(r4+28); fail clears r3
    cpu.set_t(cpu.r[5] & 0xff == 0);
    if cpu.t() {
        cpu.r[5] = r4 + 28;
        cpu.r[0] = 0x8c185abc;
        cpu.fr[1] = ld(r4 + 28);
        cpu.fr[0] = c01;
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if !cpu.t() {
            cpu.r[3] = 0;
        }
    }
    // block 10: runs iff r3 == 0; r4 += 32, val *(r4+32); fail clears r1
    cpu.set_t(cpu.r[3] & 0xff == 0);
    if cpu.t() {
        let r4n = cpu.r[4].wrapping_add(32);
        cpu.r[4] = r4n; // add #32,r4
        cpu.r[0] = 0x8c185abc;
        cpu.fr[1] = ld(r4n);
        cpu.fr[0] = c01;
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 (05 f1)
        if !cpu.t() {
            cpu.r[1] = 0;
        }
    }

    // epilogue: pop r9; extu.b r1,r0; tst r0,r0; pop r10; movt r0; pops; rts
    for (i, n) in (9..=14).enumerate() {
        cpu.r[n] = saved[i];
    }
    cpu.r[0] = cpu.r[1] & 0xff; // extu.b r1,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    let t = cpu.t();
    cpu.r[0] = t as u32; // movt r0
}
