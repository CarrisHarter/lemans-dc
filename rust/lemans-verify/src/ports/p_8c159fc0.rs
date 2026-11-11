//! 0x8c159fc0 — free a block (dlmalloc-style free).
//!
//! r4 = block header pointer. If r4 == 0, return immediately (T=1).
//! Otherwise: negate the size at r4-4, shrink the top chunk at *0x8c2b1ef8,
//! maybe insert into a free list, maybe coalesce with the previous block,
//! compute a bin index from the size, then walk the bins at
//! mem[0x8c2b1ef8]+4+4*k downwards, inserting r4 wherever the bin entry
//! is greater (unsigned) than r4; return when a bin entry <= r4.
//!
//! Plain `bt`/`bf` have no delay slot here; only `bf.s`/`bt.s` do.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c15a060).unwrap_or(0); // 0x8c2b1ef8
    let mut r4 = cpu.r[4];
    if r4 == 0 {
        cpu.set_t(true); // tst r4,r4 ; bt rts (delay nop)
        return;
    }
    cpu.set_t(false);
    r4 = r4.wrapping_sub(8);
    let mut r1 = r4; // mov r4,r1
    let r3n = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0).wrapping_neg();
    let _ = mem.write_u32(r4.wrapping_add(4), r3n); // neg r3,r3 ; mov.l r3,@(4,r4)
    let r5 = mem.read_u32(g).unwrap_or(0); // mov.l @r0,r5 (top ptr)
    let r0 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
    let top = mem.read_u32(r5).unwrap_or(0).wrapping_add(r0);
    let _ = mem.write_u32(r5, top); // mov.l r3,@r5
    let r0 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
    r1 = r1.wrapping_add(r0).wrapping_add(8); // add r0,r1 ; add #8,r1
    let mut r3 = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r1),r3
    let r0 = r3.wrapping_add(1);
    if (r0 as i32) >= 0 {
        // cmp/pz passed
        if r3 != 0x7fffffff {
            // cmp/eq 0x7fffffff,r3 failed: insert into free list
            r1 = r1.wrapping_add(r3);
            let _ = mem.write_u32(r1.wrapping_add(8), r4); // mov.l r4,@(8,r1)
        }
        let r1b = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        r3 = r3.wrapping_add(8);
        let r1c = r1b.wrapping_add(r3);
        let _ = mem.write_u32(r4.wrapping_add(4), r1c); // mov.l r1,@(4,r4)
        let h = mem.read_u32(g).unwrap_or(0);
        let v = mem.read_u32(h).unwrap_or(0).wrapping_add(8);
        let _ = mem.write_u32(h, v); // mov.l r0,@r1 with r0 = mem[h]+8
    }
    // 8c15a006: coalesce check
    let r5 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r5
    let r0 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0).wrapping_add(1);
    if (r0 as i32) >= 0 {
        // cmp/pz passed: coalesce with previous block
        let r3 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        let r1 = r4.wrapping_add(r3);
        let _ = mem.write_u32(r1.wrapping_add(8), r5); // mov.l r5,@(8,r1)
        let r3 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        r4 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r4
        let r3 = r3.wrapping_add(8);
        let r1 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0).wrapping_add(r3);
        let _ = mem.write_u32(r4.wrapping_add(4), r1);
        let h = mem.read_u32(g).unwrap_or(0);
        let v = mem.read_u32(h).unwrap_or(0).wrapping_add(8);
        let _ = mem.write_u32(h, v);
    }
    // bin index: r0 = 16, r5 = 1; while (size >= r0) { r0 <<= 1; r5 += 1 }
    let r1 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
    let mut r0: u32 = 16;
    let mut r5: u32 = 1; // bf.s delay slot: mov #1,r5
    if (r1 as i32) >= (r0 as i32) {
        loop {
            r0 = r0 << 1; // shll r0
            r5 = r5.wrapping_add(1); // bt.s delay slot: add #1,r5 (always runs)
            if (r1 as i32) >= (r0 as i32) {
                continue;
            }
            break;
        }
    }
    cpu.set_t(false); // last cmp/ge at 8c15a03a/8c15a032 was false
    // bin walk
    let r3 = g; // mov.l 0x8c15a060,r3
    let mut r0 = r5 << 2; // bra delay: shll2 r0
    let mut r6;
    let mut r1;
    loop {
        r6 = mem.read_u32(r3).unwrap_or(0); // mov.l @r3,r6
        r1 = mem.read_u32(r0.wrapping_add(r6.wrapping_add(4))).unwrap_or(0); // mov.l @(r0,r1),r1
        let t = r1 > r4; // cmp/hi r4,r1
        cpu.set_t(t);
        if !t {
            break; // fall through to rts
        }
        r6 = r6.wrapping_add(4); // add #4,r6
        let _ = mem.write_u32(r0.wrapping_add(r6), r4); // mov.l r4,@(r0,r6)
        r5 = r5.wrapping_sub(1);
        r0 = r0.wrapping_sub(4);
    }
    // rts ; nop
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
}
