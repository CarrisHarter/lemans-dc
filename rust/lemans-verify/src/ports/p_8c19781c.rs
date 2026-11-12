//! 0x8c19781c - swap the sub-object at `[r4+60]`, releasing the old one.
//!
//! When `[r13+60]` is non-null it goes to 0x8c03c950 first, then the slot is
//! overwritten with r14. r14 and pr are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(60)).unwrap_or(0); // mov.l @(60,r13),r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] != 0 {
        cpu.r[0] = mem.read_u32(0x8c19783c).unwrap_or(0x8c03c950); // mov.l 0x8c19783c,r0
        cpu.pr = 0x8c197832;
        crate::registry::p_8c03c950::run(cpu, mem);
    }
    let _ = mem.write_u32(cpu.r[13].wrapping_add(60), cpu.r[14]); // mov.l r14,@(60,r13)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
