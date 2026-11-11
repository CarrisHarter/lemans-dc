//! 0x8c02097c — copy 16-byte records from [r4..r5) to r6; return r6.
//!
//! `cmp/eq r5,r4 ; bt done ; loop { tst r6,r6 ; bt skip ; four
//! mov.l @(n,r4),r0 / mov.l r0,@(n,r6) pairs ; skip: add #16,r4 ; cmp/eq
//! r5,r4 ; bf.s loop ; add #16,r6 } done: rts ; mov r6,r0`
//!
//! Side effects the C hides: r4 AND r6 advance by 0x10 per iteration (the
//! r6 increment is the bf.s delay slot and runs even on the final pass),
//! r0 is clobbered by every load but the rts delay slot overwrites it with
//! the final r6, and T ends set because the loop only exits through a
//! cmp/eq that compared equal.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let end = cpu.r[5];
    cpu.set_t(cpu.r[4] == end); // 0x8c02097c cmp/eq r5,r4
    if cpu.r[4] != end {
        loop {
            if cpu.r[6] != 0 {
                // 0x8c020984..0x8c020992: four load/store pairs, r0 holds each load
                for off in [0u32, 4, 8, 12] {
                    let v = mem.read_u32(cpu.r[4].wrapping_add(off)).unwrap_or(0);
                    cpu.r[0] = v;
                    let _ = mem.write_u32(cpu.r[6].wrapping_add(off), v);
                }
            }
            cpu.r[4] = cpu.r[4].wrapping_add(16); // 0x8c020994
            cpu.set_t(cpu.r[4] == end); // 0x8c020996
            cpu.r[6] = cpu.r[6].wrapping_add(16); // bf.s delay slot 0x8c02099a
            if cpu.r[4] == end {
                break; // bf.s not taken
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // rts delay slot 0x8c02099e
}
