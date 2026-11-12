//! 0x8c18db84 — find the global delimiter byte in `buf = *r4`, length
//! `*(r4+4)`; on a hit store 1 at the found byte and return
//! `*(r4+8) + 44*index`, otherwise 0.
//!
//! Original bytes:
//! ```text
//! mov.l r14,@-r15 ; mov.l 0x8c18dbc8,r0 ; mov #0,r5 ; mov.l @r4,r7
//! mov.b @r0,r1 ; mov r7,r6 ; mov.l @(4,r4),r14 ; extu.b r1,r1
//! mov r7,r0 ; add r14,r6 ; bra 0x8c18db9e ; extu.b r1,r1
//! loop: add #1,r0 ; cmp/eq r6,r0 ; bt done ; mov.b @r0,r3
//!       extu.b r3,r3 ; extu.b r3,r3 ; cmp/eq r1,r3 ; bf loop
//! done: sub r7,r0 ; cmp/eq r14,r0 ; bt ret ; mov #1,r1
//!       mov.b r1,@(r0,r7) ; mov #44,r1 ; mul.l r1,r0 ; sts macl,r0
//!       mov.l @(8,r4),r5 ; add r0,r5
//! ret: mov.l @r15+,r14 ; rts ; mov r5,r0
//! ```
//!
//! The needle is loaded once from the literal 0x8c18dbc8 -> 0x8c20f018 and
//! zero-extended by both `extu.b` (the second is the `bra` delay slot). The
//! loop compares the cursor with `buf + len` (`cmp/eq` only, so a wrapped end
//! scans forward until equality). `sub r7,r0` leaves the index; `mul.l` writes
//! MACL *and* MACH with the signed 44*index product, and `sts macl,r0` feeds
//! the scaled index into `*(r4+8) + ...`. T ends 1 on the miss path (the
//! `cmp/eq r14,r0` that branched) and 0 on the hit path (nothing after that
//! comparison sets flags). r1 ends 44 on a hit, the needle byte on a miss;
//! r0 ends as the returned value from the `rts` delay slot; r3 is the last
//! zero-extended data byte compared (entry value when len == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r15_entry = cpu.r[15];
    let r15 = r15_entry.wrapping_sub(4);
    let _ = mem.write_u32(r15, cpu.r[14]); // mov.l r14,@-r15

    let needle_ptr = mem.read_u32(0x8c18dbc8).unwrap(); // mov.l 0x8c18dbc8,r0
    let mut r5 = 0u32; // mov #0,r5
    let r7 = mem.read_u32(r4).unwrap(); // mov.l @r4,r7
    let mut r1 = mem.read_u8(needle_ptr).unwrap() as i8 as i32 as u32; // mov.b @r0,r1
    let mut r6 = r7; // mov r7,r6
    let r14 = mem.read_u32(r4.wrapping_add(4)).unwrap(); // mov.l @(4,r4),r14
    r1 &= 0xff; // extu.b r1,r1
    let mut r0 = r7; // mov r7,r0
    r6 = r6.wrapping_add(r14); // add r14,r6
    r1 &= 0xff; // bra delay slot: extu.b r1,r1

    let mut r3 = cpu.r[3];
    loop {
        cpu.set_t(r0 == r6); // cmp/eq r6,r0
        if r0 == r6 {
            break; // bt done
        }
        r3 = mem.read_u8(r0).unwrap() as i8 as i32 as u32; // mov.b @r0,r3
        r3 &= 0xff; // extu.b r3,r3 (0x8c18dba4)
        r3 &= 0xff; // extu.b r3,r3 (0x8c18dba6)
        cpu.set_t(r3 == r1); // cmp/eq r1,r3
        if r3 != r1 {
            r0 = r0.wrapping_add(1); // 0x8c18db9c: add #1,r0
            continue; // bf 0x8c18db9c
        }
        break; // match: fall through to done
    }

    r0 = r0.wrapping_sub(r7); // sub r7,r0 -> index
    cpu.set_t(r0 == r14); // cmp/eq r14,r0
    let mut mach = cpu.mach;
    let mut macl = cpu.macl;
    if r0 != r14 {
        r1 = 1; // mov #1,r1
        let _ = mem.write_u8(r7.wrapping_add(r0), r1 as u8); // mov.b r1,@(r0,r7)
        r1 = 44; // mov #44,r1
        let p = (r0 as i32 as i64) * (r1 as i32 as i64); // mul.l r1,r0
        mach = (p >> 32) as u32;
        macl = p as u32;
        r0 = macl; // sts macl,r0
        r5 = mem.read_u32(r4.wrapping_add(8)).unwrap(); // mov.l @(8,r4),r5
        r5 = r5.wrapping_add(r0); // add r0,r5
    }

    cpu.r[14] = mem.read_u32(r15).unwrap(); // mov.l @r15+,r14
    cpu.r[0] = r5; // rts delay slot: mov r5,r0
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.mach = mach;
    cpu.macl = macl;
    cpu.r[15] = r15_entry;
}
