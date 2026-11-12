//! 0x8c07add4 — advance a ring-buffer cursor by `r5` four-byte elements.
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
//! The routine only seeks: the cursor is stored, never dereferenced. Plain
//! `bt`/`bf` are not delayed on the SH-4, so the instruction after each
//! conditional branch belongs to the fall-through path only; the delayed
//! branches here are `bra` and `rts`.
//!
//! `sts.l pr,@-r15` / `lds.l @r15+,pr` cancel and the synthetic stack window is
//! not compared, so the prologue and epilogue leave no traced state.
//!
//! `r5 == i32::MIN` is deliberately not modelled: `neg` leaves it negative and
//! the original `jsr`s the self-pointer at 0x8c07ae4c with unchanged arguments,
//! so it recurses forever and an oracle run only stops on its step budget. No
//! spec can compare that input.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];

    if (r5 as i32) >= 0 {
        // 0x8c07ae18: append path.
        let h = mem.read_u32(r4).unwrap();
        let s1 = mem.read_u32(r4.wrapping_add(4)).unwrap();
        let h2 = mem.read_u32(h.wrapping_add(8)).unwrap();
        let h0 = mem.read_u32(h).unwrap();
        let sum = h2.wrapping_add(s1).wrapping_add(r5);
        cpu.set_t(h0 > sum); // cmp/hi r1,r3 (plain, not delayed)
        if h0 <= sum {
            // 0x8c07ae34: wrapped cursor.
            let h3 = mem.read_u32(h.wrapping_add(12)).unwrap();
            // 0x8c07ae38: shll2 r1 is the last T writer on this arm.
            cpu.set_t(sum.wrapping_sub(h0) >> 30 & 1 != 0);
            let value = sum.wrapping_sub(h0).wrapping_shl(2).wrapping_add(h3);
            mem.write_u32(r4.wrapping_add(8), value).unwrap();
        } else {
            // 0x8c07ae28: cursor += r5 * 4.
            let s2 = mem.read_u32(r4.wrapping_add(8)).unwrap();
            // 0x8c07ae2c: shll2 r0 is the last T writer on this arm.
            cpu.set_t(r5 >> 30 & 1 != 0);
            let value = s2.wrapping_add(r5.wrapping_shl(2));
            mem.write_u32(r4.wrapping_add(8), value).unwrap();
        }
        // 0x8c07ae3e: index += r5.
        let s1 = mem.read_u32(r4.wrapping_add(4)).unwrap();
        let index = s1.wrapping_add(r5);
        mem.write_u32(r4.wrapping_add(4), index).unwrap();

        cpu.r[6] = h;
        cpu.r[3] = h0;
        cpu.r[0] = r4;
        cpu.r[1] = index;
    } else {
        // 0x8c07adea: rewind path, n = -r5.
        let n = r5.wrapping_neg();

        let h = mem.read_u32(r4).unwrap();
        let s1 = mem.read_u32(r4.wrapping_add(4)).unwrap();
        let h2 = mem.read_u32(h.wrapping_add(8)).unwrap();
        let x = h2.wrapping_add(s1);
        cpu.set_t(x >= n); // cmp/hs r6,r1 (plain, not delayed)
        if x < n {
            // 0x8c07ae00: wrapped cursor; r6 keeps n.
            let h0 = mem.read_u32(h).unwrap();
            let h3 = mem.read_u32(h.wrapping_add(12)).unwrap();
            // 0x8c07ae06: shll2 r1 is the last T writer on this arm.
            cpu.set_t(h0.wrapping_sub(n) >> 30 & 1 != 0);
            let value = h0.wrapping_sub(n).wrapping_shl(2).wrapping_add(h3);
            mem.write_u32(r4.wrapping_add(8), value).unwrap();
            cpu.r[6] = n;
        } else {
            // 0x8c07adf6: cursor -= n * 4; r6 becomes n * 4.
            let s2 = mem.read_u32(r4.wrapping_add(8)).unwrap();
            // 0x8c07adf8: shll2 r6 is the last T writer on this arm.
            cpu.set_t(n >> 30 & 1 != 0);
            let value = s2.wrapping_sub(n.wrapping_shl(2));
            mem.write_u32(r4.wrapping_add(8), value).unwrap();
            cpu.r[6] = n.wrapping_shl(2);
        }
        // 0x8c07ae0c: index += r5 (the delta stays negative).
        let s1 = mem.read_u32(r4.wrapping_add(4)).unwrap();
        let index = s1.wrapping_add(r5);
        mem.write_u32(r4.wrapping_add(4), index).unwrap();

        cpu.r[3] = h;
        cpu.r[0] = r4;
        cpu.r[1] = index;
    }
}
