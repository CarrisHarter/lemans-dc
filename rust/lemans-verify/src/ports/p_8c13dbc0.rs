//! 0x8c13dbc0 — copy 8-byte elements from [r4..r5) to r6 (skipped entirely
//! when r6 is zero), then return r6.
//!
//! `cmp/eq r5,r4 ; bt done` skips the loop when r4 == r5 on entry. The loop
//! body tests r6 with `tst r6,r6` before copying, and the `bf.s` delay slot
//! `add #8,r6` runs on every iteration including the last, so r6 always ends
//! 8 past the final element and r0 (the rts delay slot `mov r6,r0`) is that
//! advanced pointer. Final T = 1 (last cmp/eq succeeded).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
    if !cpu.t() {
        loop {
            // 0x8c13dbc4: tst r6,r6 ; bt 0x8c13dbd0
            cpu.set_t(cpu.r[6] == 0);
            if !cpu.t() {
                let r1 = mem.read_u32(cpu.r[4]).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
                cpu.r[1] = r1;
                let _ = mem.write_u32(cpu.r[6], r1); // mov.l r1,@r6
                let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r6)
            }
            // 0x8c13dbd0: add #8,r4 ; cmp/eq r5,r4 ; bf.s 0x8c13dbc4 ; add #8,r6
            cpu.r[4] = cpu.r[4].wrapping_add(8);
            cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
            cpu.r[6] = cpu.r[6].wrapping_add(8); // bf.s delay slot: always runs
            if cpu.t() {
                break; // bf.s not taken -> fall to 0x8c13dbd8
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // rts delay slot: mov r6,r0
}
