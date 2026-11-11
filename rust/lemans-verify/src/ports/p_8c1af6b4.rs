//! 0x8c1af6b4 — walk 32-byte blocks from r4 up to r5, copying 8 floats per
//! block into the parallel buffer at r6 (only while r6 != 0).
//!
//! ```text
//! cmp/eq r5,r4 ; bt 0x8c1af71c          ; T = (r4 == r5)
//! 0x8c1af6b8: tst r6,r6 ; bt 0x8c1af714
//!   copy 8 floats [r4+i*4] -> [r6+i*4], i = 0..8 (fr0 ends as [r4+28])
//! 0x8c1af714: add #32,r4 ; cmp/eq r5,r4 ; bf.s 0x8c1af6b8 ; add #32,r6
//! 0x8c1af71c: rts ; mov r6,r0
//! ```
//! The `bf.s` delay slot `add #32,r6` runs on every iteration, including
//! the final non-taken one, so r6 ends at r6_entry + 32*n and r0 = that.
//! Exit T = 1 always. Copy-path finals: r1 = r6+28, r3 = r4+24, r7 = r6+24.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    if cpu.r[4] == cpu.r[5] {
        cpu.set_t(true); // cmp/eq r5,r4, bt taken
    } else {
        loop {
            if cpu.r[6] != 0 {
                let (r4, r6) = (cpu.r[4], cpu.r[6]);
                for i in 0..8u32 {
                    let v = mem.read_f32(r4 + i * 4).unwrap();
                    mem.write_f32(r6 + i * 4, v).unwrap();
                    cpu.fr[0] = v;
                }
                cpu.r[0] = r4 + 28;
                cpu.r[1] = r6 + 28;
                cpu.r[3] = r4 + 24;
                cpu.r[7] = r6 + 24;
            }
            cpu.r[4] = cpu.r[4].wrapping_add(32);
            cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
            cpu.r[6] = cpu.r[6].wrapping_add(32); // bf.s delay slot, always
            if cpu.r[4] == cpu.r[5] {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // rts delay slot: mov r6,r0
}
