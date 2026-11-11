//! 0x8c079c78 — dec *(r4+4); if *(r4+8)==*(*r4+12) then *(r4+8)=*(*r4+12)+(*r4)*4; finally *(r4+8)-=4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let v = mem.read_u32(cpu.r[4] + 4).unwrap_or(0).wrapping_sub(1);
    cpu.r[0] = v;
    let _ = mem.write_u32(cpu.r[4] + 4, v);
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[3] + 12).unwrap_or(0);
    cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r1,r0
    if cpu.r[1] == cpu.r[0] {
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[0] = cpu.r[0] << 2; // shll2 r0
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    }
    cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    cpu.r[1] = cpu.r[1].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]); // rts delay slot
}
