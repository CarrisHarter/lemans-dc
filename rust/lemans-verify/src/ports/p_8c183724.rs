//! 0x8c183724 — unsigned-greater linked-list scan.
//!
//! r6 = *(r4+4); r3 = 0 (bt.s delay slot). While r6 != 0: r1 = *r5, r0 =
//! *(r6+12); if r0 > r1 (unsigned) follow *(r6) else record r3 = r6 and
//! follow *(r6+4). At the end, if r3 == 0 or *r5 > *(r3+12) store the
//! post-incremented r4 (r4 += 4 happens on that path only) at *r15, else
//! store r3 there. T ends 1 on the r4-store paths, 0 on the r3-store path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let mut r6 = mem.read_u32(r4 + 4).unwrap(); // mov.l @(4,r4),r6
    cpu.r[6] = r6;
    cpu.set_t(r6 == 0); // tst r6,r6
    let mut r3: u32 = 0; // bt.s delay slot: mov #0,r3 (always runs)
    cpu.r[3] = r3;
    if r6 != 0 {
        loop {
            let r1 = mem.read_u32(r5).unwrap(); // mov.l @r5,r1
            cpu.r[1] = r1;
            let r0 = mem.read_u32(r6 + 12).unwrap(); // mov.l @(12,r6),r0
            cpu.r[0] = r0;
            cpu.set_t(r0 > r1); // cmp/hi r1,r0
            if r0 > r1 {
                // bf not taken -> bra 0x8c18373c; delay slot: r6 = *r6
                r6 = mem.read_u32(r6).unwrap();
            } else {
                // bf taken -> 0x8c183738
                r3 = r6;
                r6 = mem.read_u32(r6 + 4).unwrap();
            }
            cpu.r[6] = r6;
            cpu.r[3] = r3;
            cpu.set_t(r6 == 0); // tst r6,r6
            if r6 == 0 {
                break; // bf 0x8c18372c not taken
            }
        }
    }
    // 0x8c183740
    cpu.set_t(r3 == 0); // tst r3,r3
    let mut store = r3; // 0x8c183754 path stores r3 (r4 not incremented)
    if r3 == 0 {
        store = cpu.r[4].wrapping_add(4); // 0x8c18374c: add #4,r4 then store r4
        cpu.r[4] = store;
    } else {
        let r1 = mem.read_u32(r3 + 12).unwrap(); // mov.l @(12,r3),r1
        cpu.r[1] = r1;
        let r0 = mem.read_u32(r5).unwrap(); // mov.l @r5,r0
        cpu.r[0] = r0;
        cpu.set_t(r0 > r1); // cmp/hi r1,r0
        if r0 > r1 {
            store = cpu.r[4].wrapping_add(4);
            cpu.r[4] = store;
        }
    }
    let r0 = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r0
    cpu.r[0] = r0;
    let _ = mem.write_u32(r0, store); // rts delay slot: mov.l r4/r3,@r0
}
