//! 0x8c21bba6 — byte-swap-ish copy of a stack-passed word into r4+8..11,
//! then classify *(r4+8) and store a result at r4+20.
//!
//! r2 = 0x8c2ba634, r1 = 0x10c4; r3 = *r5 (r5 += 4, @r5+); the word is spilled
//! to the stack (excluded window). If *( *(0x8c2ba634) + 0x10c4 ) == 1 the
//! bytes land reversed (stack[0]->r4+11 ... stack[3]->r4+8, last via the bra
//! delay slot), else in order (stack[0]->r4+8 ... stack[3]->r4+11).
//! r0 = extu.b(*(r4+8)); r3 = 0xff: if r0==0xff -> r1=-1, *(r4+20)=-1 (bra
//! delay), T=1; else r2=0xe0, T=(r0>=0xe0): if so r1=*r5 (r5 now r5e+4),
//! *(r4+20)=r1; else *(r4+20)=0 (bra delay). rts delay: r15 += 4, r0 = 0.
//!
//! Final: r0=0, r1=-1 / *r5e+4 / 0x10c4, r2=0x8c2ba634 (0xff path) or 0xe0,
//! r3=0xff, r5=r5e+4, r6=r15e+3, r15=r15e, T = 1,1,0 per path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5e = cpu.r[5];
    let r15e = cpu.r[15];
    cpu.r[2] = 0x8c2ba634; // mov.l literal,r2
    let mut r1: u32 = 0x10c4; // mov.w 0x10c4,r1
    let r3 = mem.read_u32(r5e).unwrap_or(0); // mov.l @r5+,r3 (spilled to stack window)
    let r3v = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r2,r0
    let r0 = mem.read_u32(r3v.wrapping_add(r1)).unwrap_or(0); // mov.l @(r0,r1),r0
    let eq = r0 == 1; // cmp/eq #1,r0
    cpu.set_t(eq);
    let b = r3.to_le_bytes(); // bytes at r15e-4 .. r15e-1
    let r6 = r15e.wrapping_sub(1); // add #-4,r15; mov r15,r6; three mov.b @r6+,r0
    if eq {
        mem.write_u8(r4 + 11, b[0]).unwrap();
        mem.write_u8(r4 + 10, b[1]).unwrap();
        mem.write_u8(r4 + 9, b[2]).unwrap();
        mem.write_u8(r4 + 8, b[3]).unwrap(); // bra delay slot: mov.b r0,@(8,r4)
    } else {
        mem.write_u8(r4 + 8, b[0]).unwrap();
        mem.write_u8(r4 + 9, b[1]).unwrap();
        mem.write_u8(r4 + 10, b[2]).unwrap();
        mem.write_u8(r4 + 11, b[3]).unwrap();
    }
    let mut r0 = mem.read_u8(r4 + 8).unwrap_or(0) as u32; // mov.b @(8,r4),r0
    let r3f: u32 = 0x00ff; // mov.w 0x00ff,r3
    r0 &= 0xff; // extu.b r0,r0
    cpu.set_t(r0 == r3f); // cmp/eq r3,r0
    let mut r2 = 0x8c2ba634u32;
    if r0 == 0xff {
        r1 = 0xffffffff; // mov #-1,r1
        mem.write_u32(r4 + 20, r1).unwrap(); // bra delay slot: mov.l r1,@(20,r4)
    } else {
        r2 = 0x00e0; // mov.w 0x00e0,r2
        let t = (r0 as i32) >= (r2 as i32); // cmp/ge r2,r0
        cpu.set_t(t);
        if t {
            r1 = mem.read_u32(r5e.wrapping_add(4)).unwrap_or(0); // mov.l @r5,r1 (r5=r5e+4)
            mem.write_u32(r4 + 20, r1).unwrap();
        } else {
            mem.write_u32(r4 + 20, 0).unwrap(); // bra delay slot: mov.l r0,@(20,r4), r0=0
        }
    }
    cpu.r[0] = 0; // mov #0,r0 before rts
    cpu.r[1] = r1;
    cpu.r[2] = r2;
    cpu.r[3] = r3f;
    cpu.r[5] = r5e.wrapping_add(4); // mov.l @r5+,r3
    cpu.r[6] = r6;
    cpu.r[15] = r15e; // add #-4,r15 ... rts delay add #4,r15
}
