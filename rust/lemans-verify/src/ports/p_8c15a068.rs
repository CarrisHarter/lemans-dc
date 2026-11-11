//! 0x8c15a068 — allocate from the top chunk (wilderness).
//!
//! r4 = request size. `cmp/hi r0,r4` (r0 = 0x1000000) sets T = (r4 > r0)
//! unsigned: if r4 > 0x1000000, rts with r0 = 0. Otherwise align r4 up to 8
//! (r4 == 0 becomes 8), take the chunk at mem[mem[0x8c2b1ef8] + mem[0x8c2b1f00] - 8],
//! walk the list while (r4 > chunk_size) signed, then split if the remaining
//! size >= 8, else hand out the whole chunk. Return chunk+8 (or remainder+8).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c15a0f0).unwrap_or(0); // 0x8c2b1ef8
    let g2 = mem.read_u32(0x8c15a0f4).unwrap_or(0); // 0x8c2b1f00
    let mut r4 = cpu.r[4];
    if r4 > 0x0100_0000 {
        // cmp/hi r0,r4 -> T = (r4 > r0) = true ; rts ; delay mov #0,r0
        cpu.r[0] = 0;
        cpu.set_t(true);
        return;
    }
    // tst r4,r4 ; bf 0x8c15a07a — r4 == 0 is impossible here only if
    // r4 > 0x1000000 was false... handle it faithfully anyway:
    if r4 == 0 {
        r4 = 8; // bra 0x8c15a080 ; delay mov #8,r4
    } else {
        r4 = r4.wrapping_add(7) & 0xFFFF_FFF8;
    }
    let h = mem.read_u32(g).unwrap_or(0);
    let w = mem.read_u32(g2).unwrap_or(0);
    let mut r0 = mem.read_u32(h.wrapping_add(w).wrapping_sub(8)).unwrap_or(0);
    let mut r7 = mem.read_u32(r0.wrapping_add(4)).unwrap_or(0); // bra delay slot
    loop {
        let t = (r4 as i32) > (r7 as i32); // cmp/gt r7,r4 -> T = (r4 > r7)
        cpu.set_t(t);
        if !t {
            break;
        }
        r0 = mem.read_u32(r0).unwrap_or(0); // mov.l @r0,r0
        let _r1 = mem.read_u32(r0).unwrap_or(0); // mov.l @r0,r1 ; pref @r1 (no effect)
        r7 = mem.read_u32(r0.wrapping_add(4)).unwrap_or(0);
    }
    if r7 == 0x7FFF_FFFF {
        // cmp/eq r1,r7 -> rts ; delay mov #0,r0
        cpu.r[0] = 0;
        cpu.set_t(true);
        return;
    }
    let r6 = r4.wrapping_add(8);
    r7 = r7.wrapping_sub(r6); // remaining
    let t = (r7 as i32) >= 8; // cmp/ge r1,r7 -> T = (r7 >= r1)
    cpu.set_t(t);
    if t {
        // split path (0x8c15a0b4)
        let r5 = r0.wrapping_add(8).wrapping_add(r7);
        let r1 = r5.wrapping_add(r4);
        let _ = mem.write_u32(r1.wrapping_add(8), r5); // mov.l r5,@(8,r1)
        let r3 = r4.wrapping_neg();
        let _ = mem.write_u32(r5, r0); // mov.l r0,@r5
        let h = mem.read_u32(g).unwrap_or(0);
        let _ = mem.write_u32(r5.wrapping_add(4), r3); // mov.l r3,@(4,r5)
        let _ = mem.write_u32(r0.wrapping_add(4), r7); // mov.l r7,@(4,r0)
        let r1 = mem.read_u32(h).unwrap_or(0).wrapping_sub(r6);
        let _ = mem.write_u32(h, r1); // bra delay: mov.l r1,@r3
        // rts ; delay add #8,r0
        cpu.r[0] = r5.wrapping_add(8);
        cpu.r[1] = r1;
        cpu.r[3] = h;
        cpu.r[4] = r4;
        cpu.r[5] = r5;
        cpu.r[6] = r6;
        cpu.r[7] = r7;
    } else {
        // no-split path (0x8c15a0d6)
        let r4n = mem.read_u32(g).unwrap_or(0); // mov.l @r1,r4 with r1 = G: r4 = mem[G]
        let r1 = mem.read_u32(r0.wrapping_add(4)).unwrap_or(0);
        let r3 = mem.read_u32(r4n).unwrap_or(0).wrapping_sub(r1);
        let _ = mem.write_u32(r4n, r3); // mov.l r3,@r4
        let r1 = mem.read_u32(r0.wrapping_add(4)).unwrap_or(0);
        let r1n = r1.wrapping_neg();
        let _ = mem.write_u32(r0.wrapping_add(4), r1n); // mov.l r1,@(4,r0)
        // rts ; delay add #8,r0
        cpu.r[0] = r0.wrapping_add(8);
        cpu.r[1] = r1n;
        cpu.r[3] = r3;
        cpu.r[4] = r4n;
        cpu.r[6] = r6;
        cpu.r[7] = r7;
    }
}
