//! 0x8c0ab680 — merge: copies from r5 to r4+108, from r6 to r4+120, and from r6 to r4+0x90.
//!
//! Original side effects: r7 = r4+108, r3 = r4+120, r4 += 0x90, r0/r1 = last
//! loaded words.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[7] = cpu.r[4].wrapping_add(108);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[7], cpu.r[0]);
    cpu.r[3] = cpu.r[4].wrapping_add(120);
    cpu.r[0] = 0x90;
    let _ = mem.write_u32(cpu.r[7] + 4, cpu.r[1]);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[7] + 8, cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[3] + 4, cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3] + 8, cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // rts delay slot
}
