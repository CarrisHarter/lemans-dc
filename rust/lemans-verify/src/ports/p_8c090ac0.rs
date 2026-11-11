//! 0x8c090ac0 — copy 36 bytes from r5 into the node at *(r4+8), then advance
//! the r4 list's next/cur pointers by 0x54 with wrap-around to *r4 at tail.
//!
//! Side effects the C does not show: byte 0 of the node is zeroed first;
//! r5 ends as src+32, r6 = src+20, r7 = dst+20, r3 = *(src+20), fr0 = the float
//! at src+32; the first `bf` delay slot loads r0 = *r4 unconditionally; the
//! second `bf` delay slot `add #84,r0` also runs unconditionally, so on the
//! no-wrap path r0 = *(r4+4)+84, and T holds the last `cmp/eq`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let list = cpu.r[4];
    let src = cpu.r[5];
    let dest = mem.read_u32(list + 8).unwrap_or(0); // mov.l @(8,r4),r0
    let _ = mem.write_u8(dest, 0); // mov #0,r1 ; mov.b r1,@r0
    let s4 = src.wrapping_add(4); // mov r5,r14 ; add #4,r14
    let r13 = dest.wrapping_add(4); // mov r0,r13 ; add #4,r13
    let r6 = src.wrapping_add(20); // mov r5,r6 ; add #20,r6
    let r7 = dest.wrapping_add(20); // mov r0,r7 ; add #20,r7
    let r3 = mem.read_u32(s4).unwrap_or(0); // mov.l @r14,r3
    let r1 = mem.read_u32(s4 + 4).unwrap_or(0); // mov.l @(4,r14),r1
    let _ = mem.write_u32(r13, r3); // mov.l r3,@r13
    let _ = mem.write_u32(r13 + 4, r1); // mov.l r1,@(4,r13)
    let r3 = mem.read_u32(s4 + 8).unwrap_or(0); // mov.l @(8,r14),r3
    let r1 = mem.read_u32(s4 + 12).unwrap_or(0); // mov.l @(12,r14),r1
    cpu.r[5] = src.wrapping_add(32); // add #32,r5
    let r0 = dest.wrapping_add(32); // add #32,r0
    let _ = mem.write_u32(r13 + 8, r3); // mov.l r3,@(8,r13)
    let _ = mem.write_u32(r13 + 12, r1); // mov.l r1,@(12,r13)
    let r3 = mem.read_u32(r6).unwrap_or(0); // mov.l @r6,r3
    let r1 = mem.read_u32(r6 + 4).unwrap_or(0); // mov.l @(4,r6),r1
    let _ = mem.write_u32(r7, r3); // mov.l r3,@r7
    let _ = mem.write_u32(r7 + 4, r1); // mov.l r1,@(4,r7)
    let r1 = mem.read_u32(r6 + 8).unwrap_or(0); // mov.l @(8,r6),r1
    let _ = mem.write_u32(r7 + 8, r1); // mov.l r1,@(8,r7)
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr0
    let _ = mem.write_f32(r0, cpu.fr[0]); // fmov fr0,@r0
    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;

    // list advance: *(r4+8) += 84, wrap to *r4 if it hit the tail
    let next = mem.read_u32(list + 8).unwrap_or(0).wrapping_add(84); // add #84,r0
    let _ = mem.write_u32(list + 8, next); // mov.l r0,@(8,r4)
    let r1 = mem.read_u32(list + 8).unwrap_or(0); // mov.l @(8,r4),r1
    let mut r0 = mem.read_u32(list + 12).unwrap_or(0); // mov.l @(12,r4),r0 (tail)
    cpu.set_t(r0 == r1); // cmp/eq r0,r1
    if cpu.t() {
        // bf not taken: mov.l @r4,r0 ; mov.l r0,@(8,r4)
        r0 = mem.read_u32(list).unwrap_or(0);
        let _ = mem.write_u32(list + 8, r0);
    }
    cpu.r[0] = r0;

    // second: *(r4+4) vs *(r4+8); both `bf`s here are undelayed (0x8B), so the
    // instructions after them run only when the branch is NOT taken.
    let r1 = mem.read_u32(list + 8).unwrap_or(0); // mov.l @(8,r4),r1
    let cur = mem.read_u32(list + 4).unwrap_or(0); // mov.l @(4,r4),r0
    cpu.r[0] = cur;
    cpu.set_t(cur == r1); // cmp/eq r0,r1
    if cpu.t() {
        let r0 = cur.wrapping_add(84); // add #84,r0
        let _ = mem.write_u32(list + 4, r0); // mov.l r0,@(4,r4)
        let r1 = mem.read_u32(list + 4).unwrap_or(0); // mov.l @(4,r4),r1
        let tail = mem.read_u32(list + 12).unwrap_or(0); // mov.l @(12,r4),r0
        cpu.r[0] = tail;
        cpu.set_t(tail == r1); // cmp/eq r0,r1
        if cpu.t() {
            cpu.r[0] = mem.read_u32(list).unwrap_or(0); // mov.l @r4,r0
            let _ = mem.write_u32(list + 4, cpu.r[0]); // mov.l r0,@(4,r4)
        }
        cpu.r[1] = r1;
    } else {
        cpu.r[1] = r1; // r0 stays cur
    }
    // mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 — r13/r14/r15 restored as-is
}
