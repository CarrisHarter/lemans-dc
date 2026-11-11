//! 0x8c1a6de8 — struct init, pool 0x8c211470 single deref, zeros at +8..+24.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    cpu.r[0] = s;
    cpu.r[4] = mem.read_u32(0x8c211470).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.r[3] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
    for off in [4u32, 8, 12, 16, 20, 24] {
        let _ = mem.write_u32(cpu.r[0] + off, cpu.r[1]);
    }
}
