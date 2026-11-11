//! 0x8c18da2c — search a byte buffer for the global byte at 0x8c20f010.
//!
//! Original: `base = *r4; n = *(r4+4); for (i = 0; i != n; i++)` if
//! `buf[i] == needle { buf[i] = 1; return *(r4+8) + 48*i; }` `return 0;`
//!
//! Details the C hides: the needle is loaded once from the literal
//! 0x8c18da74 -> 0x8c20f010 and zero-extended twice (8c18da3a and the `bra`
//! delay slot 8c18da42). r6 = base + n is the end sentinel compared with
//! `cmp/eq r6,r0`. On a hit the found byte is overwritten with 1 and r5 is
//! built as 48*i via `shll`/`add`/`shll2`/`shll2` (the `shll` sets T from
//! bit 31 of i); the `rts` delay slot moves r5 into r0. On a miss r5 stays 0,
//! r0 = 0 and T ends 1 from `cmp/eq r14,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r15_entry = cpu.r[15];
    let r15 = r15_entry.wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(r15, cpu.r[14]);
    let needle_ptr = mem.read_u32(0x8c18da74).unwrap_or(0); // -> 0x8c20f010
    let mut r5 = 0u32; // mov #0,r5
    let r7 = mem.read_u32(r4).unwrap_or(0);
    let mut r1 = mem.read_u8(needle_ptr).unwrap_or(0) as i8 as i32 as u32; // mov.b @r0,r1
    let mut r6 = r7; // mov r7,r6
    let r14 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
    r1 &= 0xff; // extu.b r1,r1 (0x8c18da3a)
    let mut r0 = r7; // mov r7,r0
    r6 = r6.wrapping_add(r14); // add r14,r6
    r1 &= 0xff; // bra delay slot: extu.b r1,r1 (0x8c18da42)
    let mut r3 = cpu.r[3];
    loop {
        // 0x8c18da46
        cpu.set_t(r0 == r6); // cmp/eq r6,r0
        if r0 == r6 {
            break; // bt 0x8c18da54 (plain bt, no delay slot)
        }
        r3 = mem.read_u8(r0).unwrap_or(0) as i8 as i32 as u32; // mov.b @r0,r3
        r3 &= 0xff; // extu.b r3,r3 (0x8c18da4c, repeated at 0x8c18da4e)
        cpu.set_t(r3 == r1); // cmp/eq r1,r3
        if r3 != r1 {
            r0 = r0.wrapping_add(1); // 0x8c18da44: add #1,r0
            continue; // bf 0x8c18da44
        }
        break; // match: fall through to 0x8c18da54
    }
    // 0x8c18da54
    r0 = r0.wrapping_sub(r7); // sub r7,r0 -> index
    cpu.set_t(r0 == r14); // cmp/eq r14,r0
    if r0 != r14 {
        r1 = 1; // mov #1,r1
        r5 = r0;
        let t = (r5 >> 31) != 0;
        r5 <<= 1; // shll r5
        cpu.set_t(t);
        r5 = r5.wrapping_add(r0); // add r0,r5 -> 3*i
        let _ = mem.write_u8(r7.wrapping_add(r0), r1 as u8); // mov.b r1,@(r0,r7)
        r5 <<= 2; // shll2 r5 -> 12*i
        let base2 = mem.read_u32(r4.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r0
        r5 <<= 2; // shll2 r5 -> 48*i
        r5 = r5.wrapping_add(base2); // add r0,r5
    }
    // 0x8c18da6c: pop r14; rts delay slot: mov r5,r0
    cpu.r[14] = mem.read_u32(r15).unwrap_or(cpu.r[14]);
    cpu.r[0] = r5;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[15] = r15_entry;
}
