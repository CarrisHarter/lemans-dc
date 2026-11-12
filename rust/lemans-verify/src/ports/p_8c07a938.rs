//! 0x8c07a938 — advance a ring-buffer cursor by `r5` four-byte elements.
//!
//! `r4` points at a four-word object:
//!   +0  H, a four-word descriptor: H[0] element count, H[2] base index,
//!       H[3] wrap-around base address (H[1] is unused here)
//!   +4  running element index
//!   +8  cursor address
//! `r5` is a signed delta. A positive delta appends, a negative one rewinds;
//! either way the index wraps modulo H[0], and the wrapped cursor is
//! `(index - H[0]) * 4 + H[3]`.
//!
//! Plain `bt`/`bf` are not delayed on the SH-4, so the instruction after each
//! conditional branch belongs to the fall-through path only. The only delayed
//! branches here are `bra` and `rts`, whose delay slots store the new cursor /
//! do nothing.
//!
//! `r5 == i32::MIN` is deliberately not modelled: `neg` leaves it negative and
//! the original then `jsr`s through the self-pointer at 0x8c07a9b0, recursing
//! until the stack window is exhausted. No spec seeds that value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];

    // sts.l pr,@-r15 (the synthetic stack window is not compared)

    if (r5 as i32) >= 0 {
        // 0x8c07a97c: append path.
        let h = mem.read_u32(r4).unwrap();
        let s1 = mem.read_u32(r4 + 4).unwrap();
        let h2 = mem.read_u32(h + 8).unwrap();
        let h0 = mem.read_u32(h).unwrap();
        let sum = h2.wrapping_add(s1).wrapping_add(r5);
        cpu.set_t(h0 > sum); // cmp/hi r1,r3 (plain, not delayed)
        if h0 <= sum {
            // 0x8c07a998: wrapped cursor.
            let h3 = mem.read_u32(h + 12).unwrap();
            let value = sum.wrapping_sub(h0).wrapping_shl(2).wrapping_add(h3);
            mem.write_u32(r4 + 8, value).unwrap();
        } else {
            // 0x8c07a98c: cursor += r5 * 4.
            let s2 = mem.read_u32(r4 + 8).unwrap();
            let value = s2.wrapping_add(r5.wrapping_shl(2));
            mem.write_u32(r4 + 8, value).unwrap();
        }
        // 0x8c07a9a2: index += r5.
        let s1 = mem.read_u32(r4 + 4).unwrap();
        let index = s1.wrapping_add(r5);
        mem.write_u32(r4 + 4, index).unwrap();

        cpu.r[6] = h;
        cpu.r[3] = h0;
        cpu.r[0] = r4;
        cpu.r[1] = index;
    } else {
        // 0x8c07a93e: rewind path, n = -r5.
        let n = r5.wrapping_neg();
        // cmp/pz r6 + bt 0x8c07a94e: every negative r5 except i32::MIN has
        // n > 0 and takes the path below.

        let h = mem.read_u32(r4).unwrap();
        let s1 = mem.read_u32(r4 + 4).unwrap();
        let h2 = mem.read_u32(h + 8).unwrap();
        let x = h2.wrapping_add(s1);
        cpu.set_t(x >= n); // cmp/hs r6,r1 (plain, not delayed)
        if x < n {
            // 0x8c07a964: wrapped cursor; r6 stays n.
            let h0 = mem.read_u32(h).unwrap();
            let h3 = mem.read_u32(h + 12).unwrap();
            let value = h0.wrapping_sub(n).wrapping_shl(2).wrapping_add(h3);
            mem.write_u32(r4 + 8, value).unwrap();
            cpu.r[6] = n;
        } else {
            // 0x8c07a95a: cursor -= n * 4; r6 becomes n * 4.
            let s2 = mem.read_u32(r4 + 8).unwrap();
            let value = s2.wrapping_sub(n.wrapping_shl(2));
            mem.write_u32(r4 + 8, value).unwrap();
            cpu.r[6] = n.wrapping_shl(2);
        }
        // 0x8c07a970: index += r5 (the delta stays negative).
        let s1 = mem.read_u32(r4 + 4).unwrap();
        let index = s1.wrapping_add(r5);
        mem.write_u32(r4 + 4, index).unwrap();

        cpu.r[3] = h;
        cpu.r[0] = r4;
        cpu.r[1] = index;
    }

    // 0x8c07a9aa: lds.l @r15+,pr ; rts ; nop
}
