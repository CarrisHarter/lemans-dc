//! 0x8c05de64 — init struct at r4: [0]=*0x8c1f7540, [4]=0, [8]=byte(r5), [12]=r4+4.
//!
//! Original side effects: r15 net unchanged, r6 = original r4, r0 = r6,
//! r1 = r6+4, r3 = r6+8, r4 = 0; the [12] slot is written twice, finally r1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    cpu.r[6] = s; // via r15 round-trip
    cpu.r[0] = mem.read_u32(0x8c1f7540).unwrap_or(0); // pool value stored as-is
    cpu.r[4] = 0;
    cpu.r[3] = cpu.r[6];
    cpu.r[1] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    let b = mem.read_u8(cpu.r[5]).unwrap_or(0);
    cpu.r[0] = b as u32;
    let _ = mem.write_u8(cpu.r[3], b);
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[4]);
    cpu.r[0] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[1]);
}
