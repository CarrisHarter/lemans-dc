//! 0x8c050d90 — "step back" cursor advance for one lane of the table walker.
//!
//! ABI: r4 = walker struct. Uses two ROM-resident pointer tables loaded from
//! the literal pool at 0x8c050e98/0x8c050e9c: A = 0x8c1f6be8 (array of group
//! structs, indexed by *(p+4)*4) and B = 0x8c1f6bd8 (array of entry bases,
//! same index). A group struct is { count, entries* } at group + *(p+0x20)*8;
//! entries is a u32 index array; entry pointers are B[sel] + idx*0x50.
//!
//! ```text
//! cur = *(p+0x24); if (cur == 0) cur = count-1; else cur--; *(p+0x24) = cur
//! c28 = *(p+0x28); if (cur == 0) c28 = count-1; else c28--; *(p+0x28) = c28
//! *(p+8)  = B[sel] + entries[c28]*0x50
//! *(p+0xc)= B[sel] + entries[cur]*0x50
//! T = (count-1 == cur)
//! *(p+0x10)= B[sel] + (T ? entries[0] : entries[cur+1])*0x50
//! *(p+0x14..0x1c) = *( *(p+8) +0x14..0x1c )
//! *(p+0x34)--
//! rts (delay: *(p+0x34) store)
//! ```
//!
//! Side effects the decompiled C does not show: r0 is left = r4 (the struct
//! pointer, from `mov r4,r0` at 0x8c050e84); r1 ends as the decremented
//! *(p+0x34); r3 holds the copied +0x14 field; r5 = *(p+8)+0x14 and r6 = p+0x14
//! (the copy pointers); r7 is left as `cur` on the T=1 branch but `cur*4`
//! (`shll2 r7` at 0x8c050e60) on the T=0 branch; the delay-slot stores of
//! `bra` at 0x8c050da8/0x8c050dc8 run on both paths, and the `rts` delay slot
//! performs the *(p+0x34) decrement store.

use sh4_oracle::{Cpu, Memory};

const A_TABLE: u32 = 0x8c1f_6be8; // value in the literal pool at 0x8c050e98
const B_TABLE: u32 = 0x8c1f_6bd8; // value in the literal pool at 0x8c050e9c

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

#[inline]
fn wr(mem: &mut Memory, addr: u32, v: u32) {
    mem.write_u32(addr, v).unwrap();
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];

    // Block 1 (0x8c050d90-0x8c050dae): decrement or reload *(p+0x24).
    let mut cur = rd(mem, p + 0x24);
    if cur == 0 {
        let sel = rd(mem, p + 4);
        let group = rd(mem, A_TABLE + sel * 4);
        let count = rd(mem, group + rd(mem, p + 0x20) * 8);
        cur = count.wrapping_sub(1);
    } else {
        cur = cur.wrapping_sub(1);
    }
    wr(mem, p + 0x24, cur); // delay slot of `bra 0x8c050db0` / direct store

    // Block 2 (0x8c050db0-0x8c050dd0): same pattern for *(p+0x28), keyed on cur.
    let mut c28 = rd(mem, p + 0x28);
    if cur == 0 {
        let sel = rd(mem, p + 4);
        let group = rd(mem, A_TABLE + sel * 4);
        let count = rd(mem, group + rd(mem, p + 0x20) * 8);
        c28 = count.wrapping_sub(1);
    } else {
        c28 = c28.wrapping_sub(1);
    }
    wr(mem, p + 0x28, c28);

    // Tail (0x8c050dd2 onward).
    let sel = rd(mem, p + 4);
    let group = rd(mem, A_TABLE + sel * 4);
    let entries = rd(mem, group + rd(mem, p + 0x20) * 8 + 4);
    let base = rd(mem, B_TABLE + sel * 4);

    let idx8 = rd(mem, entries + c28 * 4);
    let e8 = base.wrapping_add(idx8.wrapping_mul(0x50));
    wr(mem, p + 8, e8);

    let idxc = rd(mem, entries + cur * 4);
    let ec = base.wrapping_add(idxc.wrapping_mul(0x50));
    wr(mem, p + 0xc, ec);

    let count = rd(mem, group + rd(mem, p + 0x20) * 8);
    let t = count.wrapping_sub(1) == cur; // cmp/eq r7,r3 at 0x8c050e3e
    let mut t_out = t;
    let e10 = if t {
        let idx0 = rd(mem, entries); // mov.l @r0,r3: entries[0]
        // Both arms end in three shll2 (v -> v*5 -> v*20 -> v*80), so the last
        // one's carry is bit 30 of v*20 and it replaces the cmp/eq T.
        t_out = idx0.wrapping_mul(20) >> 30 & 1 != 0;
        base.wrapping_add(idx0.wrapping_mul(0x50))
    } else {
        let addr = entries.wrapping_add(4).wrapping_add(cur.wrapping_mul(4));
        let idxn = rd(mem, addr); // entries[cur+1]
        t_out = idxn.wrapping_mul(20) >> 30 & 1 != 0;
        base.wrapping_add(idxn.wrapping_mul(0x50))
    };
    wr(mem, p + 0x10, e10);

    // Copy *(p+8)+0x14..0x1c into p+0x14..0x1c (0x8c050e7a-0x8c050e8e).
    let v14 = rd(mem, e8.wrapping_add(0x14));
    let v18 = rd(mem, e8.wrapping_add(0x18));
    let v1c = rd(mem, e8.wrapping_add(0x1c));
    wr(mem, p + 0x14, v14);
    wr(mem, p + 0x18, v18);
    wr(mem, p + 0x1c, v1c);

    // rts at 0x8c050e94, delay slot stores the decremented *(p+0x34).
    let c34 = rd(mem, p + 0x34).wrapping_sub(1);
    wr(mem, p + 0x34, c34);

    cpu.r[0] = p; // mov r4,r0 at 0x8c050e84
    cpu.r[1] = c34;
    cpu.r[3] = v14;
    cpu.r[5] = e8.wrapping_add(0x14); // add #20,r5 at 0x8c050e80
    cpu.r[6] = p + 0x14; // add #20,r6 at 0x8c050e7e
    cpu.r[7] = if t { cur } else { cur.wrapping_mul(4) };
    cpu.set_t(t_out);
}
