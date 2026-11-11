//! 0x8c1550fc — copy 3-word elements from r4 to r6 until r4 == r5 or r6 == 0; r0 = r6 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.set_t(cpu.r[5] == cpu.r[4]); // cmp/eq r5,r4
        if cpu.r[5] == cpu.r[4] {
            cpu.r[0] = cpu.r[6];
            return;
        }
        cpu.set_t(cpu.r[6] == 0); // tst r6,r6
        if cpu.r[6] == 0 {
            cpu.r[0] = cpu.r[6];
            return;
        }
        cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6], cpu.r[1]);
        let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6] + 8, cpu.r[0]);
        cpu.r[4] = cpu.r[4].wrapping_add(12);
        cpu.r[6] = cpu.r[6].wrapping_add(12); // bf.s delay slot
    }
}
