//! 0x8c21b418 — pointer bump on a global struct plus a fixed-fill loop.
//!
//! r5 = 0x8c2ba634 (literal); r3 = *0x8c2ba634. r0 = mov.w 0x0fcc = 0x0fcc
//! (bit 15 clear, so it extends to +0xfcc). *(r3+0xfcc) += 4 * *(r3+0xfd8)
//! (the +0xfd8 word is r0+12, shll2'd). If r4 > 0 (cmp/pl; T survives to the
//! bf.s), store 0x700 (literal 0x0700) at the new *(r3+0xfcc) + 4*i for
//! i = 0..r4 (loop: add #1,r6; store; cmp/ge r4,r6; delay-slot add #4,r1).
//! Then *(r3+0xfd8) = r4 (entry value), *(r3+0xfdc) += r4<<2.
//! rts delay slot zeroes r0.
//!
//! Final: r0=0, r4=r4e<<2, r5=0x8c2ba634, r6=max(r4e,0), r7=0x700,
//! r1 = new *(r3+0xfcc) + 4*r4e when r4e>0 else the re-read word,
//! r2 = updated *(r3+0xfdc), r3 = *0x8c2ba634, T = (r4e > 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4e = cpu.r[4];
    cpu.r[5] = 0x8c2ba634; // mov.l literal,r5
    let t = (r4e as i32) > 0; // cmp/pl r4
    cpu.set_t(t);
    let r0: u32 = 0x0fcc; // mov.w 0x0fcc,r0 (bit15 clear -> +0xfcc)
    let r3 = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r3
    let r1 = r3.wrapping_add(r0.wrapping_add(12)); // r3 + 0xfd8
    let w = mem.read_u32(r1).unwrap_or(0); // mov.l @r1,r1
    let mut r2 = mem.read_u32(r3.wrapping_add(r0)).unwrap_or(0); // @(r0,r3)
    let w = w << 2; // shll2 r1 (T untouched)
    let r7: u32 = 0x0700; // mov.w 0x0700,r7
    r2 = r2.wrapping_add(w);
    mem.write_u32(r3.wrapping_add(r0), r2).unwrap(); // mov.l r2,@(r0,r3)
    let base = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r1
    let mut r1 = mem.read_u32(base.wrapping_add(r0)).unwrap_or(0); // @(r0,r1)
    let mut r6: u32 = 0; // bf.s delay slot: mov #0,r6
    if t {
        // bf.s not taken (r4 > 0): fall into the loop
        loop {
            r6 = r6.wrapping_add(1); // add #1,r6
            mem.write_u32(r1, r7).unwrap(); // mov.l r7,@r1
            let t2 = (r6 as i32) >= (r4e as i32); // cmp/ge r4,r6
            cpu.set_t(t2);
            r1 = r1.wrapping_add(4); // bf.s delay slot: add #4,r1
            if t2 {
                break;
            }
        }
    }
    let r0 = 0x0fd8u32; // mov.w 0x0fd8,r0
    let r2v = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r2
    mem.write_u32(r2v.wrapping_add(r0), r4e).unwrap(); // mov.l r4,@(r0,r2)
    let r0 = r0.wrapping_add(4); // add #4,r0 -> 0xfdc
    let r3n = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r3
    let r4s = r4e << 2; // shll2 r4
    let mut r2 = mem.read_u32(r3n.wrapping_add(r0)).unwrap_or(0); // @(r0,r3)
    r2 = r2.wrapping_add(r4s); // add r4,r2
    mem.write_u32(r3n.wrapping_add(r0), r2).unwrap(); // mov.l r2,@(r0,r3)
    cpu.r[0] = 0; // rts delay slot: mov #0,r0
    cpu.r[1] = r1;
    cpu.r[2] = r2;
    cpu.r[3] = r3n;
    cpu.r[4] = r4s;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
