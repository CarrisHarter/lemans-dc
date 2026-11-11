//! 0x8c148368 — same switch shape as 0x8c148310 but with case offsets
//! 72/84/96/108 and default return mem[r4+20] from the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 3; // mov #3,r0
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[5] as i32)); // cmp/gt r5,r0: T = r0 > r5
    if !cpu.t() {
        // bf 0x8c1483b2: rts delay slot mov.l @(20,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap();
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c1483b8).unwrap(); // 0x8c27a898
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap();
    cpu.r[0] = mem.read_u32(0x8c1483bc).unwrap(); // 0xc4
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap();
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    let off = if cpu.t() {
        72
    } else {
        cpu.set_t(cpu.r[0] == 1);
        if cpu.t() {
            84
        } else {
            cpu.set_t(cpu.r[0] == 2);
            if cpu.t() {
                96
            } else {
                cpu.set_t(cpu.r[0] == 3);
                if cpu.t() {
                    108
                } else {
                    // bra 0x8c1483b2: rts delay slot mov.l @(20,r4),r0
                    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap();
                    return;
                }
            }
        }
    };
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[0] = cpu.r[5];
    cpu.r[4] = (cpu.r[4] as i32).wrapping_add(off) as u32;
    // rts delay slot: mov.l @(r0,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap();
}
