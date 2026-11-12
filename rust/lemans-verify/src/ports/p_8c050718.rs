//! 0x8c050718 — resolve table entries for a struct at r4, param_2 in r5.
//!
//! `[r4+0x24]` is stored from the `bf.s` delay slot on both branches. For
//! r5 != 0, `[r4+0x28] = r5 - 1`; for r5 == 0 it is `[T1+8*idx2] - 1`. The
//! entry pointer `E = [T1 + 4 + 8*idx2]` indexes a slot array: slotA =
//! `E[ [r4+0x28] ]`, slotB = `E[r5]`, and `[r4+8]/[r4+0xc]` become
//! `T2 + 0x50*slot`. If `[T1+8*idx2]-1 == r5`, `[r4+0x10]` uses `T2 +
//! 0x50*E[0]` (one extra deref level), else `T2 + 0x50*E[1 + r5]`. Finally
//! 12 bytes are copied from `[[r4+8] + 0x14]` to `[r4+0x14]` and `[r4+0x34]`
//! gets r5 in the `rts` delay slot.
//!
//! Hidden side effects: T is not the cmp/eq at all - both arms end in three
//! `shll2` on the slot value, so T is bit 30 of slot*20; r5 ends as r4+0x14, r3 as
//! `[r4+8]+0x14`, r6 as 4*idx1, r7 as 4*r5 only on the else path, r0/r1
//! from the tail loads/stores.

use sh4_oracle::{Cpu, Memory};

const TABLE_PTR_1: u32 = 0x8c1f6be8; // per-idx1 -> row pointer table
const TABLE_PTR_2: u32 = 0x8c1f6bd8; // per-idx1 -> 0x50-byte record table

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    let p2 = cpu.r[5];

    cpu.set_t(p2 == 0); // tst r5,r5
    let _ = mem.write_u32(s.wrapping_add(0x24), p2); // bf.s delay slot, always

    let idx1 = mem.read_u32(s.wrapping_add(4)).unwrap_or(0);
    let idx2 = mem.read_u32(s.wrapping_add(0x20)).unwrap_or(0);
    let t1 = mem
        .read_u32(TABLE_PTR_1.wrapping_add(idx1.wrapping_mul(4)))
        .unwrap_or(0);
    let t2 = mem
        .read_u32(TABLE_PTR_2.wrapping_add(idx1.wrapping_mul(4)))
        .unwrap_or(0);
    let count = mem.read_u32(t1.wrapping_add(idx2.wrapping_mul(8))).unwrap_or(0);

    let v28 = if p2 == 0 {
        count.wrapping_sub(1) // [T1+8*idx2] - 1
    } else {
        p2.wrapping_sub(1) // [r4+0x24] - 1, and [r4+0x24] == p2 here
    };
    let _ = mem.write_u32(s.wrapping_add(0x28), v28);

    let entry = mem
        .read_u32(t1.wrapping_add(4).wrapping_add(idx2.wrapping_mul(8)))
        .unwrap_or(0);
    let slot_a = mem.read_u32(entry.wrapping_add(v28.wrapping_mul(4))).unwrap_or(0);
    let _ = mem.write_u32(s.wrapping_add(8), t2.wrapping_add(slot_a.wrapping_mul(0x50)));
    let slot_b = mem.read_u32(entry.wrapping_add(p2.wrapping_mul(4))).unwrap_or(0);
    let _ = mem.write_u32(s.wrapping_add(0xc), t2.wrapping_add(slot_b.wrapping_mul(0x50)));

    let eq = count.wrapping_sub(1) == p2;
    cpu.set_t(eq); // cmp/eq r7,r3
    let v10 = if eq {
        let inner = mem.read_u32(entry).unwrap_or(0); // extra deref: [E]
        // Both arms end in three shll2 (v -> v*5 -> v*20 -> v*80), so the last
        // one's carry is bit 30 of v*20 and it replaces the cmp/eq T.
        cpu.set_t(inner.wrapping_mul(20) >> 30 & 1 != 0);
        t2.wrapping_add(inner.wrapping_mul(0x50))
    } else {
        let v = mem
            .read_u32(entry.wrapping_add(4).wrapping_add(p2.wrapping_mul(4)))
            .unwrap_or(0);
        // The else path ends in three shll2 on r3 (r5 -> r5*5 -> r5*20 -> r5*80),
        // so the last one's carry is bit 30 of r5*20 and it replaces the cmp/eq T.
        cpu.set_t(v.wrapping_mul(20) >> 30 & 1 != 0);
        t2.wrapping_add(v.wrapping_mul(0x50))
    };
    let _ = mem.write_u32(s.wrapping_add(0x10), v10);

    let src = mem
        .read_u32(s.wrapping_add(8))
        .unwrap_or(0)
        .wrapping_add(0x14);
    let a = mem.read_u32(src).unwrap_or(0);
    let b = mem.read_u32(src.wrapping_add(4)).unwrap_or(0);
    let c = mem.read_u32(src.wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(s.wrapping_add(0x14), a);
    let _ = mem.write_u32(s.wrapping_add(0x18), b);
    let _ = mem.write_u32(s.wrapping_add(0x1c), c);
    let _ = mem.write_u32(s.wrapping_add(0x34), p2); // rts delay slot

    cpu.r[0] = p2; // mov.l @(36,r4),r0 before rts
    cpu.r[1] = a;
    cpu.r[3] = src;
    cpu.r[5] = s.wrapping_add(0x14);
    cpu.r[6] = idx1.wrapping_mul(4);
    cpu.r[7] = if eq { p2 } else { p2.wrapping_mul(4) };
}
