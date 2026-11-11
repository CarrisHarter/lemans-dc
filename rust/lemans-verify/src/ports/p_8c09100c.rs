//! 0x8c09100c — two fill loops (see ledger). r0/r5/r6/r7/r13/r14 are live registers.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_r13 = cpu.r[13];
    let saved_r14 = cpu.r[14];
    let base = cpu.r[4];
    cpu.r[3] = 0x178;
    cpu.r[7] = 0;
    cpu.r[0] = cpu.r[7];
    cpu.r[6] = base.wrapping_add(cpu.r[3]);
    cpu.r[1] = cpu.r[7];
    cpu.r[3] = 36;
    loop {
        cpu.r[5] = cpu.r[6];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[6]), cpu.r[1]);
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]);
        cpu.r[7] = cpu.r[7].wrapping_add(2);
        let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[1]);
        cpu.set_t(36 > (cpu.r[7] as i32)); // cmp/gt r7,r3
        cpu.r[0] = cpu.r[0].wrapping_add(8); // bt.s delay slot
        if 36 > (cpu.r[7] as i32) {
            continue;
        }
        break;
    }
    cpu.r[0] = cpu.r[1];
    cpu.r[13] = base.wrapping_add(80);
    cpu.r[4] = base.wrapping_add(72);
    cpu.r[14] = 36;
    cpu.r[7] = 0;
    cpu.r[3] = 10;
    loop {
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[14]);
        cpu.r[6] = cpu.r[13];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[7]);
        cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[0]);
        cpu.r[5] = cpu.r[4].wrapping_add(cpu.r[0]);
        let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[14]);
        cpu.r[1] = cpu.r[1].wrapping_add(2);
        let _ = mem.write_u32(cpu.r[5] + 12, cpu.r[7]);
        cpu.set_t(10 > (cpu.r[1] as i32)); // cmp/gt r1,r3
        cpu.r[0] = cpu.r[0].wrapping_add(24); // bt.s delay slot
        if 10 > (cpu.r[1] as i32) {
            continue;
        }
        break;
    }
    cpu.r[13] = saved_r13;
    cpu.r[14] = saved_r14;
}
