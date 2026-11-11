//! 0x8c2218a0 — copy (r6>>2) words from r5 to r4 (dt loop, delay r4+=4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = cpu.r[6] >> 2; // shlr2 r6
    loop {
        cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        cpu.r[6] = cpu.r[6].wrapping_sub(1);
        cpu.set_t(cpu.r[6] == 0); // dt r6
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
        cpu.r[4] = cpu.r[4].wrapping_add(4); // bf.s delay slot
        if cpu.r[6] != 0 {
            continue;
        }
        break;
    }
}
