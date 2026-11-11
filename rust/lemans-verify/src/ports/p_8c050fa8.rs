//! 0x8c050fa8 — walker reset/init for one lane.
//!
//! ABI: r4 = walker struct. Literal pool at 0x8c05106c/0x8c051070 gives
//! A = 0x8c1f6be8 (group structs) and B = 0x8c1f6bd8 (entry bases, stride
//! 0x50). Because *(p+0x20) is zeroed first, the group is always group[0]:
//! { count, entries* }.
//!
//! ```text
//! push r14; r14 = 0
//! *(p+0x20) = 0; *(p+0x24) = 0
//! *(p+0x28) = count-1; *(p+0x30) = 0; *(p+0x34) = 0
//! fldi0 fr0; fmov.s fr0,@(p+0x38)   // *(u32*)(p+0x38) = 0
//! *(p+8)  = B[sel] + entries[count-1]*0x50
//! *(p+0xc)= B[sel] + entries[0]*0x50
//! *(p+0x10)= B[sel] + entries[1]*0x50
//! *(p+0x14..0x1c) = *( *(p+8) +0x14..0x1c )
//! rts (delay: pop r14)
//! ```
//!
//! Side effects the decompiled C does not show: the prologue is a real
//! `mov.l r14,@-r15` / `mov.l @r15+,r14` pair (net identity on r14/r15, both
//! inside the excluded stack window); fr0 is clobbered to +0.0 by `fldi0`
//! (the decompiler's fr1/FPSCR_SZ double-store branch does not exist in the
//! machine code — the store is a 4-byte `fmov.s fr0,@r6`); r0 ends =
//! *(src+0x1c), r1 = *(src+0x14), r3 = src+0x14, r5 = p+0x14, r6 = entries[1]
//! (r6 held p+0x38 only for the fmov, then is reused as scratch at
//! 0x8c051044), r7 = sel*4; no instruction in the function touches T, so T
//! passes through unchanged.

use sh4_oracle::{Cpu, Memory};

const A_TABLE: u32 = 0x8c1f_6be8; // value in the literal pool at 0x8c05106c
const B_TABLE: u32 = 0x8c1f_6bd8; // value in the literal pool at 0x8c051070

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

    // r14 is pushed/popped around the body (stack window, net identity).
    wr(mem, p + 0x20, 0); // 0x8c050fac
    wr(mem, p + 0x24, 0); // 0x8c050fb6

    let sel = rd(mem, p + 4);
    let group = rd(mem, A_TABLE + sel * 4);
    let count = rd(mem, group); // group index is 0: *(p+0x20) was just zeroed
    let c28 = count.wrapping_sub(1);
    wr(mem, p + 0x28, c28); // 0x8c050fd0
    wr(mem, p + 0x30, 0); // 0x8c050fd2
    wr(mem, p + 0x34, 0); // 0x8c050fd6

    cpu.fr[0] = 0.0; // fldi0 fr0 at 0x8c050fbc
    wr(mem, p + 0x38, cpu.fr[0].to_bits()); // fmov.s fr0,@r6 at 0x8c050fd8

    let entries = rd(mem, group + 4);
    let base = rd(mem, B_TABLE + sel * 4);

    let idx8 = rd(mem, entries + c28 * 4);
    let e8 = base.wrapping_add(idx8.wrapping_mul(0x50));
    wr(mem, p + 8, e8); // 0x8c051000

    let idxc = rd(mem, entries); // *(p+0x24) == 0
    let ec = base.wrapping_add(idxc.wrapping_mul(0x50));
    wr(mem, p + 0xc, ec); // 0x8c05102a

    let idx10 = rd(mem, entries.wrapping_add(4)); // entries[1]
    let e10 = base.wrapping_add(idx10.wrapping_mul(0x50));
    wr(mem, p + 0x10, e10); // 0x8c051056

    let v14 = rd(mem, e8.wrapping_add(0x14));
    let v18 = rd(mem, e8.wrapping_add(0x18));
    let v1c = rd(mem, e8.wrapping_add(0x1c));
    wr(mem, p + 0x14, v14); // 0x8c051060
    wr(mem, p + 0x18, v18); // 0x8c051062
    wr(mem, p + 0x1c, v1c); // 0x8c051066

    cpu.r[0] = v1c;
    cpu.r[1] = v14;
    cpu.r[3] = e8.wrapping_add(0x14); // add #20,r3 at 0x8c05105a
    cpu.r[5] = p + 0x14; // add #20,r5 at 0x8c050fbe
    // r6 was p+0x38 for the fmov but is reused as scratch afterwards.
    cpu.r[7] = sel.wrapping_mul(4); // shll2 r7 at 0x8c051030
    cpu.r[6] = idx10; // final r6: `mov.l @(r0,r3),r6` = entries[1] at 0x8c051044
}
