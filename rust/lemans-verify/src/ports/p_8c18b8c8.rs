//! 0x8c18b8c8 — linear search in a table of 20-byte records.
//!
//! Original: `count = *r4; base = *(r4+4); for (i = 0; count > i; i++) {`
//! `  if (r5 == *(u32*)(base + 16 + 20*i)) return base + 20*i; }` `return 0;`
//!
//! Register details the C hides: the search index is carried in r0 as 20*i
//! (the `bra` delay slot seeds r0 = r6 = 0, and each miss does `add #20,r0`),
//! so the load is `@(r0,r3+16)`. On a match r0 is recomputed as 20*i and the
//! `rts` delay slot adds the base. On exhaustion the final `rts` delay slot
//! zeroes r0; with count <= 0 the body never runs, so r1/r3 keep their entry
//! values. Final T is the deciding compare on exhaustion, but on a match it is
//! the second `shll2`'s carry (bit 30 of r6*5).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let mut r7 = mem.read_u32(r4).unwrap_or(0);
    let mut r6: u32 = 0;
    let mut r0 = r6; // bra delay slot: mov r6,r0
    let mut r1 = cpu.r[1];
    let mut r3 = cpu.r[3];
    loop {
        let t = (r7 as i32) > (r6 as i32); // cmp/gt r6,r7
        cpu.set_t(t);
        if !t {
            r0 = 0; // rts delay slot: mov #0,r0
            break;
        }
        r3 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        r1 = mem
            .read_u32(r0.wrapping_add(r3.wrapping_add(16)))
            .unwrap_or(0);
        cpu.set_t(r5 == r1); // cmp/eq r5,r1
        if r5 == r1 {
            r0 = r6;
            r0 <<= 2; // shll2 r0
            r0 = r0.wrapping_add(r6);
            // The second shll2's carry (bit 30 of r6*5) is the final T.
            cpu.set_t(r0 >> 30 & 1 != 0);
            r0 <<= 2; // shll2 r0
            r0 = r0.wrapping_add(r3); // rts delay slot: add r3,r0
            break;
        }
        r0 = r0.wrapping_add(20); // add #20,r0
        r6 = r6.wrapping_add(1); // add #1,r6
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
