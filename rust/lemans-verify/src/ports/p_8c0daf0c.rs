//! 0x8c0daf0c — *(r4+0xd0) = *r15; r1 = 1; rts delay byte[r4+0xcc] = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = 0xd0;
    cpu.r[1] = cpu.r[4].wrapping_add(cpu.r[3]);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.r[0] = 0xcc;
    cpu.r[1] = 1;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u8); // rts delay slot
}
