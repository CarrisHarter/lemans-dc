//! 0x8c15a1dc — initialise an arena/list root at r4 with element size r5.
//!
//! Straight-line with one counted loop. Stores r4 to *0x8c2b1ef8 and r5 to
//! *0x8c2b1f00, builds a header at r4+r5-8 (r4+104 there, 0x7fffffff at +4),
//! links it, then loops r3 = 3,5,...,23 storing r4+104 at r4+8+8k and
//! r4+12+8k (k = 0..10) while 22 >= r3 (bt.s delay adds 8 to r0 every pass,
//! including the final non-taken one). rts with pop r14 in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c15a230).unwrap_or(0); // 0x8c2b1ef8
    let g2 = mem.read_u32(0x8c15a234).unwrap_or(0); // 0x8c2b1f00
    let r4e = cpu.r[4];
    let r5e = cpu.r[5];
    // prologue: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);

    let mut r3 = r4e.wrapping_add(r5e);
    let _ = mem.write_u32(g, r4e); // mov.l r4,@r1
    r3 = r3.wrapping_sub(8);
    let r4 = r4e.wrapping_add(104);
    let _ = mem.write_u32(g2, r5e); // mov.l r5,@r7
    let _ = mem.write_u32(r3, r4); // mov.l r4,@r3
    let r5 = 0xFFFF_FFFFu32; // mov #-1,r5
    let _ = mem.write_u32(r3.wrapping_add(4), 0x7FFF_FFFF); // mov.l r0,@(4,r3)
    let _ = mem.write_u32(r4, r3); // mov.l r3,@r4
    let r7 = mem.read_u32(g2).unwrap_or(0).wrapping_add((-120i32) as u32); // add #-120,r7
    let _ = mem.write_u32(r4.wrapping_add(4), r7); // mov.l r7,@(4,r4)
    let r14 = mem.read_u32(g).unwrap_or(0); // mov.l @r1,r14 = r4e
    let r7b = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(r14, r7b); // mov.l r7,@r14
    let r7c = mem.read_u32(g).unwrap_or(0);
    let _ = mem.write_u32(r7c.wrapping_add(4), 0); // mov.l r6,@(4,r7), r6 = 0
    let r6 = mem.read_u32(g).unwrap_or(0);
    let _ = mem.write_u32(96u32.wrapping_add(r6), r5); // mov.l r5,@(r0,r6), r0 = 96

    let mut r0 = 4u32;
    let r5v = 22u32;
    let mut r3v = 1u32;
    loop {
        let r6a = mem.read_u32(g).unwrap_or(0);
        r3v = r3v.wrapping_add(2); // add #2,r3
        let t = (r5v as i32) >= (r3v as i32); // cmp/ge r3,r5 -> T = (r5 >= r3)
        let r6b = r6a.wrapping_add(4);
        let _ = mem.write_u32(r0.wrapping_add(r6b), r4); // mov.l r4,@(r0,r6)
        let r6c = mem.read_u32(g).unwrap_or(0).wrapping_add(4).wrapping_add(r0);
        let _ = mem.write_u32(r6c.wrapping_add(4), r4); // mov.l r4,@(4,r6)
        r0 = r0.wrapping_add(8); // bt.s delay slot (always runs)
        cpu.set_t(t);
        if !t {
            break;
        }
    }
    // rts ; delay mov.l @r15+,r14
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = v;

    cpu.r[0] = r0;
    cpu.r[1] = g;
    cpu.r[3] = r3v;
    cpu.r[4] = r4;
    cpu.r[5] = r5v;
    cpu.r[6] = mem.read_u32(g).unwrap_or(0).wrapping_add(4).wrapping_add(r0.wrapping_sub(8));
    cpu.r[7] = mem.read_u32(g).unwrap_or(0);
}
