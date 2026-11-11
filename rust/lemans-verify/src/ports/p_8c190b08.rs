//! 0x8c190b08 — state-machine tick over global state at 0x8c2b75f8.
//!
//! `state = *0x8c2b75f8; if (state == 2) { *0x8c26c360 = 0; r4 = *(*(r4+4)+24); }`
//! `else if (state == 0) { if (++*0x8c26c360 >= *0x8c2b75f0) { *0x8c2b75f8 = 2;
//!   *0x8c26c360 = 0; r4 = *(r4+16); } else r4 = *(*(r4+4)+24); }`
//! `else { if (--*0x8c26c360 >= 0) { *0x8c2b75f8 = 2; *0x8c26c360 = 0;
//!   r4 = *(r4+12); } else { r4 = *(*(r4+4)+28); *0x8c2b75f8 = 0; } }`
//! then the common tail: `*(*0x8c26c370 + 0xf4) = *(0x8c2b7600 + 4*(*0x8c26c360));`
//! and `rts` returns r4 in r0.
//!
//! Delay-slot details: the `bf.s`/`bt.s` delay slots store the inc/dec'd
//! counter to *0x8c26c360 on every path, and the cap-exceeded paths then
//! store 0 over it (write order matters). With counter 0 the decrement path
//! reads table[-1] (0x8c2b7600 + 0xfffffffc = 0x8c2b75fc). Final T is the
//! deciding compare of each path (1 for state==2, cap reached, or counter
//! still >= 0; 0 otherwise). r5 ends 0x8c26c360 on the non-2 paths and keeps
//! its entry value on the state==2 path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let r1 = mem.read_u32(0x8c190b80).unwrap_or(0); // 0x8c2b75f8 (state ptr)
    let mut r0 = mem.read_u32(r1).unwrap_or(0);
    cpu.set_t(r0 == 2); // cmp/eq #2,r0
    let mut r5 = cpu.r[5];
    if r0 == 2 {
        // 0x8c190b10: r1 = 0x8c26c360; r0 = 0; *r1 = 0
        let cntp = mem.read_u32(0x8c190b84).unwrap_or(0);
        let _ = mem.write_u32(cntp, 0);
        let p = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        let q = mem.read_u32(p).unwrap_or(0);
        r4 = mem.read_u32(q.wrapping_add(24)).unwrap_or(0); // bra delay slot
    } else {
        cpu.set_t(r0 == 0); // tst r0,r0 (0x8c190b1e)
        if r0 == 0 {
            // increment path (0x8c190b22)
            r5 = mem.read_u32(0x8c190b84).unwrap_or(0); // 0x8c26c360
            let capp = mem.read_u32(0x8c190b88).unwrap_or(0); // 0x8c2b75f0
            let mut r3 = mem.read_u32(r5).unwrap_or(0);
            let cap = mem.read_u32(capp).unwrap_or(0);
            r3 = r3.wrapping_add(1); // add #1,r3
            let t = (r3 as i32) >= (cap as i32); // cmp/ge r0,r3
            cpu.set_t(t);
            let _ = mem.write_u32(r5, r3); // bf.s delay slot: mov.l r3,@r5 (always)
            if t {
                let _ = mem.write_u32(r1, 2); // mov #2,r3; mov.l r3,@r1
                let _ = mem.write_u32(r5, 0); // mov #0,r0; mov.l r0,@r5
                r4 = mem.read_u32(r4.wrapping_add(16)).unwrap_or(0); // bra delay: mov.l @(16,r4),r4
            } else {
                // 0x8c190b3e
                let p = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
                let q = mem.read_u32(p).unwrap_or(0);
                r4 = mem.read_u32(q.wrapping_add(24)).unwrap_or(0); // bra delay
            }
        } else {
            // decrement path (0x8c190b46)
            r5 = mem.read_u32(0x8c190b84).unwrap_or(0); // 0x8c26c360
            r0 = mem.read_u32(r5).unwrap_or(0);
            r0 = r0.wrapping_add(0xFFFF_FFFF); // add #-1,r0
            let t = (r0 as i32) >= 0; // cmp/pz r0
            cpu.set_t(t);
            let _ = mem.write_u32(r5, r0); // bt.s delay slot: mov.l r0,@r5 (always)
            if t {
                // bt.s taken (T=1, counter-1 >= 0) -> 0x8c190b5e
                let p = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
                let q = mem.read_u32(p).unwrap_or(0);
                r4 = mem.read_u32(q.wrapping_add(28)).unwrap_or(0);
                let _ = mem.write_u32(r1, 0); // mov #0,r0; mov.l r0,@r1
            } else {
                // fall-through (T=0) -> 0x8c190b52
                let _ = mem.write_u32(r1, 2); // mov #2,r3; mov.l r3,@r1
                let _ = mem.write_u32(r5, 0); // mov #0,r0; mov.l r0,@r5
                r4 = mem.read_u32(r4.wrapping_add(12)).unwrap_or(0); // bra delay: mov.l @(12,r4),r4
            }
        }
    }
    // common tail (0x8c190b68)
    let cntp = mem.read_u32(0x8c190b84).unwrap_or(0); // 0x8c26c360
    let outp = mem.read_u32(0x8c190b8c).unwrap_or(0); // 0x8c26c370
    let mut r1 = mem.read_u32(cntp).unwrap_or(0);
    let r3 = mem.read_u32(outp).unwrap_or(0);
    r1 <<= 2; // shll2 r1
    let table = mem.read_u32(0x8c190b90).unwrap_or(0); // 0x8c2b7600
    r1 = mem.read_u32(table.wrapping_add(r1)).unwrap_or(0);
    let off = mem.read_u32(0x8c190b94).unwrap_or(0); // 0xf4
    let _ = mem.write_u32(r3.wrapping_add(off), r1); // mov.l r1,@(r0,r3)
    // rts delay slot: mov r4,r0
    cpu.r[0] = r4;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
}
