//! 0x8c165b50 - seed an object, then restamp its vtable word twice.
//!
//! 0x8c056bf8 writes the first vtable word; afterwards `[r14] = 0x8c1e9198`,
//! `[r14+4] = 0`, then `[r14] = 0x8c1efe74`, so the first two stores are
//! dead. Returns r14. r14 and pr are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c165b70).unwrap_or(0x8c056bf8); // mov.l 0x8c165b70,r0
    cpu.pr = 0x8c165b5c;
    crate::registry::p_8c056bf8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c165b74).unwrap_or(0x8c1e9198); // mov.l 0x8c165b74,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // mov.l r0,@r14
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r14)
    cpu.r[0] = mem.read_u32(0x8c165b78).unwrap_or(0x8c1efe74); // mov.l 0x8c165b78,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // mov.l r0,@r14
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
