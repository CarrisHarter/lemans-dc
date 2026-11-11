//! 0x8c050ea0 — "step forward" cursor advance; the pair of 0x8c050d90.
//!
//! ABI: r4 = walker struct. Literal pool at 0x8c050fa0/0x8c050fa4 gives the
//! same tables as 0x8c050d90: A = 0x8c1f6be8 (group structs), B = 0x8c1f6bd8
//! (entry bases, stride 0x50). Group struct = { count, entries* } at
//! group + *(p+0x20)*8.
//!
//! ```text
//! *(p+0x28) = cur (= *(p+0x24))
//! if (count > cur+1)        // cmp/gt r5,r0 at 0x8c050eb8, SIGNED
//!     *(p+0x24) = cur+1     // delay slot of `bra 0x8c050eda`
//! else {
//!     *(p+0x30)++
//!     *(p+0x24) = cur+1-count
//! }
//! *(p+8)  = B[sel] + entries[*(p+0x28)]*0x50
//! *(p+0xc)= B[sel] + entries[*(p+0x24)]*0x50
//! T = (count-1 == *(p+0x24))
//! *(p+0x10)= B[sel] + (T ? entries[0] : entries[cur+1])*0x50
//! *(p+0x14..0x1c) = *( *(p+8) +0x14..0x1c )
//! *(p+0x34)++
//! rts (delay: *(p+0x34) store)
//! ```
//!
//! Side effects the decompiled C does not show: the very first store is
//! *(p+0x28) = old cur (0x8c050ea6) — it clobbers +0x28 before it is read;
//! in the wrap branch *(p+0x30) is stored BEFORE *(p+0x24); r0 ends = r4;
//! r1 = *(p+0x34)+1; r3 = copied +0x14 field; r5 = *(p+8)+0x14, r6 = p+0x14;
//! r7 = cur on the T=1 branch, cur*4 on the T=0 branch; the comparison is
//! signed (`cmp/gt`); T on exit is the second compare's result, not the
//! first cmp/gt.

use sh4_oracle::{Cpu, Memory};

const A_TABLE: u32 = 0x8c1f_6be8; // value in the literal pool at 0x8c050fa0
const B_TABLE: u32 = 0x8c1f_6bd8; // value in the literal pool at 0x8c050fa4

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

    // Prologue (0x8c050ea0-0x8c050ed8).
    let cur0 = rd(mem, p + 0x24);
    wr(mem, p + 0x28, cur0); // 0x8c050ea6: save old cursor into +0x28
    let sel = rd(mem, p + 4);
    let group = rd(mem, A_TABLE + sel * 4);
    let count = rd(mem, group + rd(mem, p + 0x20) * 8);
    let next = cur0.wrapping_add(1);
    let fits = (count as i32) > (next as i32); // cmp/gt r5,r0 at 0x8c050eb8
    let cur = if fits {
        wr(mem, p + 0x24, next); // delay slot of `bra 0x8c050eda` (0x8c050ebe)
        next
    } else {
        let c30 = rd(mem, p + 0x30).wrapping_add(1);
        wr(mem, p + 0x30, c30); // 0x8c050ec6
        let wrapped = next.wrapping_sub(count);
        wr(mem, p + 0x24, wrapped); // 0x8c050ed8
        wrapped
    };

    // Tail (0x8c050eda onward) — same shape as 0x8c050d90's tail.
    let entries = rd(mem, group + rd(mem, p + 0x20) * 8 + 4);
    let base = rd(mem, B_TABLE + sel * 4);
    let c28 = rd(mem, p + 0x28);

    let idx8 = rd(mem, entries + c28 * 4);
    let e8 = base.wrapping_add(idx8.wrapping_mul(0x50));
    wr(mem, p + 8, e8);

    let idxc = rd(mem, entries + cur * 4);
    let ec = base.wrapping_add(idxc.wrapping_mul(0x50));
    wr(mem, p + 0xc, ec);

    let t = count.wrapping_sub(1) == cur; // cmp/eq r7,r3 at 0x8c050f46
    let e10 = if t {
        let idx0 = rd(mem, entries);
        base.wrapping_add(idx0.wrapping_mul(0x50))
    } else {
        let addr = entries.wrapping_add(4).wrapping_add(cur.wrapping_mul(4));
        let idxn = rd(mem, addr);
        base.wrapping_add(idxn.wrapping_mul(0x50))
    };
    wr(mem, p + 0x10, e10);

    let v14 = rd(mem, e8.wrapping_add(0x14));
    let v18 = rd(mem, e8.wrapping_add(0x18));
    let v1c = rd(mem, e8.wrapping_add(0x1c));
    wr(mem, p + 0x14, v14);
    wr(mem, p + 0x18, v18);
    wr(mem, p + 0x1c, v1c);

    // rts at 0x8c050f9c, delay slot stores the incremented *(p+0x34).
    let c34 = rd(mem, p + 0x34).wrapping_add(1);
    wr(mem, p + 0x34, c34);

    cpu.r[0] = p; // mov r4,r0 at 0x8c050f8c
    cpu.r[1] = c34;
    cpu.r[3] = v14;
    cpu.r[5] = e8.wrapping_add(0x14); // add #20,r5 at 0x8c050f88
    cpu.r[6] = p + 0x14; // add #20,r6 at 0x8c050f86
    cpu.r[7] = if t { cur } else { cur.wrapping_mul(4) };
    cpu.set_t(t);
}
