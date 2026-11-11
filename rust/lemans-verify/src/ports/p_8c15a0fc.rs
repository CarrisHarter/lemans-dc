//! 0x8c15a0fc — first-fit malloc from size bins.
//!
//! r4 = request. `cmp/hi r0,r4` (r0 = 0x1000000) sets T = (r4 > r0) unsigned:
//! r4 > 0x1000000 -> rts r0 = 0. Else align r4 to 8 (0 -> 8), compute the bin
//! index r3 (smallest k with r4 < 16<<k), look up bin mem[H+4+4*r3]
//! (H = mem[0x8c2b1ef8]); if its size < r4 (signed) walk the chunk list at
//! 8c15a140, then either split the chunk (remaining >= 8) or take it whole,
//! insert the remainder into the bins (walking up while r6 > bin entry
//! unsigned), and return chunk+8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c15a1d0).unwrap_or(0); // 0x8c2b1ef8
    let mut r4 = cpu.r[4];
    if r4 > 0x0100_0000 {
        cpu.r[0] = 0; // rts ; delay mov #0,r0
        cpu.set_t(true);
        return;
    }
    if r4 == 0 {
        r4 = 8; // bra 0x8c15a114 ; delay mov #8,r4
    } else {
        r4 = r4.wrapping_add(7) & 0xFFFF_FFF8;
    }
    // bin index: r0 = 16, r3 = 1; while (r4 >= r0) { r3 += 1; r0 <<= 1 }
    let mut r0: u32 = 16;
    let mut r3: u32 = 1; // bra delay: mov #1,r3
    while (r4 as i32) >= (r0 as i32) {
        // 8c15a11a: add #1,r3 ; shll r0
        r3 = r3.wrapping_add(1);
        r0 = r0 << 1;
    }
    // 8c15a122: r1 = mem[mem[G] + 4 + 4*r3]  (bin head chunk)
    let h = mem.read_u32(g).unwrap_or(0);
    let mut r1 = mem.read_u32(h.wrapping_add(4).wrapping_add(r3 << 2)).unwrap_or(0);
    let mut r7 = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0);
    if (r4 as i32) > (r7 as i32) {
        // cmp/gt r7,r4 -> T = (r4 > r7): search the bin's chunk list
        let mut r7a = r7;
        if (r7a as i32) < 0 {
            r7a = r7a.wrapping_neg(); // neg r7,r7
        }
        r1 = r1.wrapping_add(r7a).wrapping_add(8);
        let mut r0 = r1; // mov r1,r0
        loop {
            let mut r5 = mem.read_u32(r0.wrapping_add(4)).unwrap_or(0);
            // cmp/pz r5 ; bt.s 8c15a14a ; delay mov r0,r1 (always runs)
            r1 = r0;
            if (r5 as i32) < 0 {
                r5 = r5.wrapping_neg();
            }
            r0 = r1.wrapping_add(r5).wrapping_add(8);
            // pref @r0: no architectural effect
            r7 = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0);
            if !((r4 as i32) > (r7 as i32)) {
                break;
            }
        }
    }
    cpu.set_t(r7 == 0x7FFF_FFFF); // cmp/eq 0x7fffffff,r7
    if r7 == 0x7FFF_FFFF {
        cpu.r[0] = 0; // rts ; delay mov #0,r0
        cpu.set_t(true);
        return;
    }
    let r6 = r4.wrapping_add(8);
    r7 = r7.wrapping_sub(r6); // remaining
    let t = (r7 as i32) >= 8; // cmp/ge r0,r7
    cpu.set_t(t);
    if t {
        // split (0x8c15a16e)
        let r5 = r1.wrapping_add(8).wrapping_add(r4);
        let r0 = r5.wrapping_add(r7);
        let _ = mem.write_u32(r0.wrapping_add(8), r5); // mov.l r5,@(8,r0)
        let r4n = r4.wrapping_neg(); // neg r4,r4
        let _ = mem.write_u32(r5, r1); // mov.l r1,@r5
        let _ = mem.write_u32(r5.wrapping_add(4), r7); // mov.l r7,@(4,r5)
        let _ = mem.write_u32(r1.wrapping_add(4), r4n); // mov.l r4,@(4,r1)
        let r4b = mem.read_u32(g).unwrap_or(0); // mov.l @r0,r4 with r0 = G: r4 = mem[G] = H
        let r0b = mem.read_u32(r4b).unwrap_or(0).wrapping_sub(r6);
        let _ = mem.write_u32(r4b, r0b); // bra delay: mov.l r0,@r4
        r4 = r4n; // r4 register now holds -size
    } else {
        // no-split (0x8c15a18e)
        let r5 = mem.read_u32(g).unwrap_or(0); // mov.l @r0,r5
        let r0 = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0);
        let r4b = mem.read_u32(r5).unwrap_or(0);
        let r4n = r4b.wrapping_sub(r0);
        let _ = mem.write_u32(r5, r4n); // mov.l r4,@r5
        let r0 = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0);
        let r0n = r0.wrapping_neg();
        let _ = mem.write_u32(r1.wrapping_add(4), r0n); // mov.l r0,@(4,r1)
        r4 = r4n; // r4 = mem[H] - mem[r1+4]
    }
    // common tail (0x8c15a1a0): insert remainder into bins
    let r0t = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0);
    let mut r6t = r1.wrapping_add(8).wrapping_sub(r0t); // sub r0,r6 (bra delay)
    let r5 = g;
    loop {
        let r4b = mem.read_u32(r5).unwrap_or(0); // mov.l @r5,r4
        let r0 = r3 << 2;
        let slot = r0.wrapping_add(r4b.wrapping_add(4));
        let entry = mem.read_u32(slot).unwrap_or(0);
        let t = r6t > entry; // cmp/hi r0,r6 -> T = (r6 > r0) unsigned
        cpu.set_t(t);
        if !t {
            break;
        }
        let _ = mem.write_u32(slot, r6t); // mov.l r6,@(r0,r4)
        r3 = r3.wrapping_add(1);
    }
    // 8c15a1c6: add #8,r1 ; rts ; delay mov r1,r0
    r1 = r1.wrapping_add(8);
    cpu.r[0] = r1;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = mem.read_u32(g).unwrap_or(0).wrapping_add(4); // tail ends with r4 = mem[G]+4
    cpu.r[5] = r5;
    cpu.r[6] = r6t;
    cpu.r[7] = r7;
}
