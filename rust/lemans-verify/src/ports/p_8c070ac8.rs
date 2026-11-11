//! 0x8c070ac8 — bounded case-insensitive compare (r6 = max chars).
//!
//! Original side effects: r14 = -32 (restored via stack), r3 = 90, r7 = 65,
//! r0/r1 = last lowered bytes, r4/r5 advanced, r6 counts down, T = last
//! tst/cmp/dt; r0 = r0 - r1 in rts delay slot. r6 < 0 returns 0 immediately.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_r14 = cpu.r[14];
    cpu.set_t((cpu.r[6] as i32) >= 0); // cmp/pz r6
    if (cpu.r[6] as i32) < 0 {
        cpu.r[0] = 0; // bra delay slot
        return; // r14 restored by rts delay slot
    }
    cpu.r[14] = (-32i32) as u32;
    cpu.r[3] = 90;
    cpu.r[7] = 65;
    loop {
        let a = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
        cpu.r[0] = a;
        cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay slot
        cpu.set_t(cpu.r[0] >= cpu.r[7]); // cmp/ge r7,r0
        if cpu.r[0] >= 65 {
            cpu.set_t(cpu.r[3] >= cpu.r[0]); // cmp/ge r0,r3
            if cpu.r[3] >= cpu.r[0] {
                cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[14]); // +32
            }
        }
        let b = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
        cpu.r[1] = b;
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bf.s delay slot
        cpu.set_t(cpu.r[1] >= cpu.r[7]);
        if cpu.r[1] >= 65 {
            cpu.set_t(cpu.r[3] >= cpu.r[1]);
            if cpu.r[3] >= cpu.r[1] {
                cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[14]);
            }
        }
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            break;
        }
        cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r1,r0
        if cpu.r[1] != cpu.r[0] {
            break;
        }
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // dt r6
        cpu.set_t(cpu.r[6] == 0);
        if cpu.r[6] == 0 {
            break;
        }
    }
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay region: sub r1,r0
    cpu.r[14] = saved_r14; // rts delay slot: pop r14
}
