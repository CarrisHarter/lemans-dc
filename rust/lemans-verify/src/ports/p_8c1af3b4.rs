//! 0x8c1af3b4 — release the block stored at `*(r4 + 8)`, if it is non-null.
//!
//! ```text
//! sts.l pr,@-r15 ; r5=[r4+8] ; tst r5,r5 ; bt out
//! r0=[r4+4] ; r1=r5 ; shll2 r0 ; shll2 r0 ; shll r0 (T=old bit31)
//! add r0,r1 ; cmp/hi r5,r1 ; bf.s call ; mov r5,r3 (delay)
//! loop: add #32,r3 ; cmp/hi r3,r1 ; bt loop
//! call: r0=0x8c01032c ; jsr @r0 ; mov r5,r4 (delay)
//! out: lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! `*(r4 + 4)` is a 32-byte element count; the loop walks `r5` up to
//! `r5 + count*0x20` in `r3`, but that residue is never read (the `jsr` delay
//! slot only passes `r5` in `r4`, and the free call rewrites `r0,r1,r3`). It is
//! transcribed anyway so the register and T state at the call match. In this
//! oracle `shll2` leaves T alone; only `shll` records bit 31. The call is the
//! `free` thunk at 0x8c01032c, i.e. `free(*(r4 + 8))`.
//!
//! The entry `pr` is pushed on the stack and popped by the exit; the callee
//! does not touch `pr` or `r15`, so the saved slot survives.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1af3b4: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1af3b6: mov.l @(8,r4),r5
    let r5 = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.r[5] = r5;
    // 0x8c1af3b8: tst r5,r5
    cpu.set_t(r5 == 0);
    // 0x8c1af3ba: bt 0x8c1af3da
    if r5 == 0 {
        exit(cpu, mem);
        return;
    }
    // 0x8c1af3bc: mov.l @(4,r4),r0
    let mut r0 = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    // 0x8c1af3be: mov r5,r1
    let mut r1 = r5;
    // 0x8c1af3c0: shll2 r0 ; 0x8c1af3c2: shll2 r0 — T untouched in this oracle
    r0 <<= 2;
    r0 <<= 2;
    // 0x8c1af3c4: shll r0 — T is bit 31 of the pre-shift value
    cpu.set_t(r0 >> 31 != 0);
    r0 <<= 1;
    // 0x8c1af3c6: add r0,r1
    r1 = r1.wrapping_add(r0);
    // 0x8c1af3c8: cmp/hi r5,r1
    cpu.set_t(r1 > r5);
    // 0x8c1af3ca: bf.s 0x8c1af3d4 ; 0x8c1af3cc: mov r5,r3 (delay slot)
    let mut r3 = r5;
    if r1 > r5 {
        loop {
            // 0x8c1af3ce: add #32,r3
            r3 = r3.wrapping_add(32);
            // 0x8c1af3d0: cmp/hi r3,r1
            cpu.set_t(r1 > r3);
            // 0x8c1af3d2: bt 0x8c1af3ce
            if r1 <= r3 {
                break;
            }
        }
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    // 0x8c1af3d4: mov.l 0x8c1af3e0,r0
    cpu.r[0] = mem.read_u32(0x8c1af3e0).unwrap_or(0x8c01032c);
    // 0x8c1af3d6: jsr @r0 ; 0x8c1af3d8: mov r5,r4 (delay slot)
    cpu.r[4] = r5;
    cpu.pr = 0x8c1af3da;
    crate::registry::p_8c01032c::run(cpu, mem);
    // 0x8c1af3da: lds.l @r15+,pr ; 0x8c1af3dc: rts ; 0x8c1af3de: nop (delay)
    exit(cpu, mem);
}

/// 0x8c1af3da: `lds.l @r15+,pr` then `rts` (the `nop` delay slot is inert).
fn exit(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
