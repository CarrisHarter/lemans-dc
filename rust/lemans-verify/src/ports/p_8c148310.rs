//! 0x8c148310 — vtable-ish switch: r0 = mem[mem[0x8c27a898]+0xc4]; if r5 < 3
//! and the switch value is 0..=3, return mem[r4 + 24/36/48/60 + r5*4] with r4
//! advanced by the same offset and r5 replaced by r5*4; otherwise (r5 >= 3 or
//! unknown value) return mem[r4+16] via the rts delay slot.
//!
//! In this oracle non-`.s` bt/bf are immediate (delay slot skipped), so each
//! `bt` jumps straight to its case and the default path falls to 0x8c14835a.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 3; // mov #3,r0
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[5] as i32)); // cmp/gt r5,r0: T = r0 > r5
    if !cpu.t() {
        // bf 0x8c14835a: rts delay slot mov.l @(16,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap();
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c148360).unwrap(); // 0x8c27a898
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap(); // mov.l @r0,r1
    cpu.r[0] = mem.read_u32(0x8c148364).unwrap(); // 0xc4
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap(); // mov.l @(r0,r1),r0
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    let off = if cpu.t() {
        24
    } else {
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.t() {
            36
        } else {
            cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
            if cpu.t() {
                48
            } else {
                cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
                if cpu.t() {
                    60
                } else {
                    // bra 0x8c14835a: rts delay slot mov.l @(16,r4),r0
                    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap();
                    return;
                }
            }
        }
    };
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    cpu.r[4] = (cpu.r[4] as i32).wrapping_add(off) as u32; // add #off,r4
    // rts delay slot: mov.l @(r0,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap();
}
