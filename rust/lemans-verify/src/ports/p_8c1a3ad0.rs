//! 0x8c1a3ad0 — if *(r4+16) > r5: r1 = *(r4+8) + r5*8; *r6 = *r1; rts delay *r7 = *(r1+4). Else r0 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 16).unwrap_or(0);
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[5] as i32)); // cmp/gt r5,r0: T=(r0>r5)
    if (cpu.r[0] as i32) > (cpu.r[5] as i32) {
        cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
        cpu.r[5] = (cpu.r[5] << 2) << 1;
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]);
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[1] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[7], cpu.r[0]); // rts delay slot
    } else {
        cpu.r[0] = 0;
    }
}
