//! 0x8c0cc51c — append four 24-byte rows to the output buffer whose pointer
//! lives in the cell at 0x8c27a730, each row = (r4[0..2] + const vector, *C,
//! w, 0/1) for the four constant vectors A/B/D/E.
//!
//! CFG (sketched before porting):
//! ```text
//! entry: push r14,r13,r12            (stack only; all restored at exit)
//!   r1 = [r4+16]
//!   r0 = r1 & 0x40 ; tst ; bf BODY    ; bit 0x40 set  -> body
//!   r0 = 0x100     ; r1 &= r0 ; tst ; bf BODY   ; bit 0x100 set -> body
//!   bra EXIT                          ; both clear -> skip (T=1)
//! BODY: straight-line 0x8c0cc538..0x8c0cc648, no branches
//! EXIT: pop r12,r13 ; rts (delay: pop r14)
//! ```
//!
//! Hidden side effects the decompiler does not show: on the body path every
//! low register ends as a buffer pointer — r0=O+0x60, r1=O+0x48, r3=O+0x50,
//! r4=O+0x4c, r5=0x8c27a730 (cell address), r6=E+4, r7=E+8 — the pointer
//! cell 0x8c27a730 itself is stored four times (O+0x18/+0x30/+0x48/+0x60),
//! fr2=1.0 and fr3=0.0, and T=0 (last tst before the body). On the skip path
//! only r0=0x100, r1=flags&0x100 and T=1 survive; r12..r15 are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let flags = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap(); // mov.l @(16,r4),r1
    cpu.r[1] = flags;
    cpu.r[0] = flags & 0x40; // mov r1,r0 ; and #64,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        // bf not taken: second mask
        cpu.r[0] = mem.read_u32(0x8c0cc654).unwrap(); // mov.l DAT,r0 = 0x100
        cpu.r[1] &= cpu.r[0]; // and r0,r1
        cpu.set_t(cpu.r[1] == 0); // tst r1,r1
        if cpu.r[1] == 0 {
            return; // bf not taken again -> bra 0x8c0cc64a; T=1, r0=0x100, r1=0
        }
    }
    // BODY. r12/r13/r14 are pushed on entry and popped at exit, so keep the
    // scratch in locals and never touch cpu.r[12..15].
    let p = mem.read_u32(0x8c0cc65c).unwrap(); // r5 = 0x8c27a730
    cpu.r[5] = p;
    cpu.r[3] = cpu.r[4]; // mov r4,r3
    let a = mem.read_u32(0x8c0cc658).unwrap(); // scratch r12 = A
    let mut sc12 = a;
    cpu.r[0] = sc12; // mov r12,r0
    cpu.fr[0] = mem.read_f32(sc12).unwrap(); // fmov @r12,fr0
    cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap(); // fmov @r4,fr1
    cpu.r[0] = cpu.r[0].wrapping_add(4); // add #4,r0 -> A+4
    cpu.r[1] = mem.read_u32(p).unwrap(); // mov.l @r5,r1 -> O
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    sc12 = sc12.wrapping_add(8); // add #8,r12 -> A+8
    mem.write_f32(cpu.r[1], cpu.fr[1]).unwrap(); // fmov fr1,@r1 -> O+0
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.r[6] = mem.read_u32(p).unwrap(); // mov.l @r5,r6 -> O
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr1 (r4+4)
    cpu.r[6] = cpu.r[6].wrapping_add(4); // O+4
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0 (A+4)
    cpu.r[1] = cpu.r[1].wrapping_add(8); // r4+8
    cpu.fr[2] = 1.0; // fldi1 fr2
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    mem.write_f32(cpu.r[6], cpu.fr[1]).unwrap(); // -> O+4
    cpu.r[7] = mem.read_u32(p).unwrap(); // O
    let b = mem.read_u32(0x8c0cc660).unwrap(); // scratch r13 = B
    cpu.r[7] = cpu.r[7].wrapping_add(8); // O+8
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // r4+8
    cpu.r[6] = b; // mov r13,r6
    cpu.fr[0] = mem.read_f32(sc12).unwrap(); // A+8
    cpu.r[6] = cpu.r[6].wrapping_add(4); // B+4
    let c = mem.read_u32(0x8c0cc664).unwrap(); // r0 = C
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[7], cpu.fr[1]).unwrap(); // -> O+8
    sc12 = mem.read_u32(p).unwrap(); // mov.l @r5,r12 -> O
    cpu.r[7] = mem.read_u32(c).unwrap(); // mov.l @r0,r7 -> *C
    let d = mem.read_u32(0x8c0cc668).unwrap(); // scratch r14 = D
    mem.write_u32(sc12.wrapping_add(12), cpu.r[7]).unwrap(); // mov.l r7,@(12,r12) -> O+12
    cpu.r[7] = mem.read_u32(p).unwrap();
    cpu.r[7] = cpu.r[7].wrapping_add(16); // O+16
    mem.write_f32(cpu.r[7], cpu.fr[3]).unwrap(); // O+16 = 0.0
    cpu.r[7] = mem.read_u32(p).unwrap();
    cpu.r[7] = cpu.r[7].wrapping_add(20); // O+20
    mem.write_f32(cpu.r[7], cpu.fr[3]).unwrap(); // O+20 = 0.0
    cpu.r[7] = mem.read_u32(p).unwrap(); // O
    cpu.fr[0] = mem.read_f32(b).unwrap(); // fmov @r13,fr0 (B)
    cpu.r[7] = cpu.r[7].wrapping_add(24); // O+24
    mem.write_u32(p, cpu.r[7]).unwrap(); // mov.l r7,@r5 -> *P = O+24
    let mut sc13 = b.wrapping_add(8); // add #8,r13 -> B+8
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[7], cpu.fr[1]).unwrap(); // -> O+24
    cpu.r[7] = mem.read_u32(p).unwrap(); // O+24
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap(); // r4+4
    cpu.r[7] = cpu.r[7].wrapping_add(4); // O+28
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0 (B+4)
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[7], cpu.fr[1]).unwrap(); // -> O+28
    cpu.r[6] = mem.read_u32(p).unwrap(); // O+24
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // r4+8
    cpu.r[6] = cpu.r[6].wrapping_add(8); // O+32
    cpu.fr[0] = mem.read_f32(sc13).unwrap(); // fmov @r13,fr0 (B+8)
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[6], cpu.fr[1]).unwrap(); // -> O+32
    cpu.r[7] = mem.read_u32(p).unwrap(); // O+24
    cpu.r[6] = mem.read_u32(c).unwrap(); // mov.l @r0,r6 -> *C
    mem.write_u32(cpu.r[7].wrapping_add(12), cpu.r[6]).unwrap(); // -> O+36
    cpu.r[6] = mem.read_u32(p).unwrap();
    cpu.r[6] = cpu.r[6].wrapping_add(16); // O+40
    mem.write_f32(cpu.r[6], cpu.fr[2]).unwrap(); // O+40 = 1.0
    cpu.r[6] = mem.read_u32(p).unwrap();
    cpu.r[6] = cpu.r[6].wrapping_add(20); // O+44
    mem.write_f32(cpu.r[6], cpu.fr[3]).unwrap(); // O+44 = 0.0
    cpu.r[6] = mem.read_u32(p).unwrap(); // O+24
    cpu.fr[0] = mem.read_f32(d).unwrap(); // fmov @r14,fr0 (D)
    cpu.r[6] = cpu.r[6].wrapping_add(24); // O+48
    mem.write_u32(p, cpu.r[6]).unwrap(); // *P = O+48
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[6], cpu.fr[1]).unwrap(); // -> O+48
    cpu.r[6] = d.wrapping_add(4); // mov r14,r6 ; add #4,r6 -> D+4
    let mut sc14 = d.wrapping_add(8); // add #8,r14 -> D+8
    sc13 = mem.read_u32(p).unwrap(); // mov.l @r5,r13 -> O+48
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap(); // r4+4
    sc13 = sc13.wrapping_add(4); // O+52
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // D+4
    let e = mem.read_u32(0x8c0cc66c).unwrap(); // mov.l DAT,r7 -> E
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(sc13, cpu.fr[1]).unwrap(); // -> O+52
    cpu.r[6] = e; // mov r7,r6
    cpu.r[7] = e; // r7 = E
    cpu.r[6] = cpu.r[6].wrapping_add(4); // E+4
    sc13 = mem.read_u32(p).unwrap(); // O+48
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // r4+8
    sc13 = sc13.wrapping_add(8); // O+56
    cpu.fr[0] = mem.read_f32(sc14).unwrap(); // fmov @r14,fr0 (D+8)
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(sc13, cpu.fr[1]).unwrap(); // -> O+56
    sc13 = mem.read_u32(p).unwrap(); // O+48
    let cval = mem.read_u32(c).unwrap(); // mov.l @r0,r14 -> *C
    mem.write_u32(sc13.wrapping_add(12), cval).unwrap(); // -> O+60
    sc14 = mem.read_u32(p).unwrap(); // mov.l @r5,r14 -> O+48
    sc14 = sc14.wrapping_add(16); // O+64
    mem.write_f32(sc14, cpu.fr[3]).unwrap(); // O+64 = 0.0
    sc14 = mem.read_u32(p).unwrap();
    sc14 = sc14.wrapping_add(20); // O+68
    mem.write_f32(sc14, cpu.fr[2]).unwrap(); // O+68 = 1.0
    sc14 = mem.read_u32(p).unwrap(); // O+48
    cpu.fr[0] = mem.read_f32(e).unwrap(); // fmov @r7,fr0 (E)
    sc14 = sc14.wrapping_add(24); // O+72
    mem.write_u32(p, sc14).unwrap(); // *P = O+72
    cpu.r[7] = e.wrapping_add(8); // add #8,r7 -> E+8
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(sc14, cpu.fr[1]).unwrap(); // -> O+72
    cpu.r[4] = mem.read_u32(p).unwrap(); // mov.l @r5,r4 -> O+72
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap(); // orig r4+4
    cpu.r[4] = cpu.r[4].wrapping_add(4); // O+76
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0 (E+4)
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[4], cpu.fr[1]).unwrap(); // -> O+76
    cpu.r[3] = mem.read_u32(p).unwrap(); // mov.l @r5,r3 -> O+72
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // orig r4+8
    cpu.r[3] = cpu.r[3].wrapping_add(8); // O+80
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap(); // fmov @r7,fr0 (E+8)
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    mem.write_f32(cpu.r[3], cpu.fr[1]).unwrap(); // -> O+80
    cpu.r[1] = mem.read_u32(p).unwrap(); // mov.l @r5,r1 -> O+72
    cpu.r[0] = mem.read_u32(c).unwrap(); // mov.l @r0,r0 -> *C
    mem.write_u32(cpu.r[1].wrapping_add(12), cpu.r[0]).unwrap(); // -> O+84
    cpu.r[0] = mem.read_u32(p).unwrap(); // O+72
    cpu.r[0] = cpu.r[0].wrapping_add(16); // O+88
    mem.write_f32(cpu.r[0], cpu.fr[2]).unwrap(); // O+88 = 1.0
    cpu.r[0] = mem.read_u32(p).unwrap();
    cpu.r[0] = cpu.r[0].wrapping_add(20); // O+92
    mem.write_f32(cpu.r[0], cpu.fr[2]).unwrap(); // O+92 = 1.0
    cpu.r[0] = mem.read_u32(p).unwrap();
    cpu.r[0] = cpu.r[0].wrapping_add(24); // O+96
    mem.write_u32(p, cpu.r[0]).unwrap(); // *P = O+96
    // T unchanged inside the body: 0 on both entries (last tst before it).
}
