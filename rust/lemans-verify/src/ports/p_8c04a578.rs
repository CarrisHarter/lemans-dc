//! 0x8c04a578 — one-shot list init: if the flag byte at 0x8c23dc30 is zero,
//! zero the node at 0x8c23dc38, push it via 0x8c017844 (r4 = node,
//! r5 = 0x8c04b018, r6 = 0x8c23dc20), set the flag to 1. Returns the node
//! pointer 0x8c23dc38 on both paths (flag set = skip straight there).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c04a5a8).unwrap_or(0); // 0x8c23dc30
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32; // mov.b + exts.b
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[4] = mem.read_u32(0x8c04a5ac).unwrap_or(0); // 0x8c23dc38
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
        cpu.r[6] = mem.read_u32(0x8c04a5b0).unwrap_or(0); // 0x8c23dc20
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
        cpu.r[5] = mem.read_u32(0x8c04a5b4).unwrap_or(0); // 0x8c04b018
        let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
        cpu.r[0] = mem.read_u32(0x8c04a5b8).unwrap_or(0); // 0x8c017844
        cpu.pr = 0x8c04a598;
        crate::registry::p_8c017844::run(cpu, mem);
        cpu.r[1] = mem.read_u32(0x8c04a5a8).unwrap_or(0); // 0x8c23dc30
        cpu.r[0] = 1;
        let _ = mem.write_u8(cpu.r[1], cpu.r[0] as u8);
    }
    cpu.r[0] = mem.read_u32(0x8c04a5ac).unwrap_or(0); // 0x8c23dc38
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
