//! 0x8c197a5c — ring-buffer push of 3 dwords from r5 into [r4+8].
//!
//! Copies [r5..r5+12) to [[r4+8] .. +12), then advances head [r4+8] by 12,
//! wrapping it to [r4] when it reaches [r4+12]. If head now equals tail
//! [r4+4], tail advances by 12 too, wrapping to [r4] at [r4+12].
//! r0/r1 hold the last compared values; T is the last cmp/eq result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    cpu.r[3] = mem.read_u32(r4 + 8).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[5] + 4).unwrap();
    let r3 = cpu.r[3];
    mem.write_u32(r3, cpu.r[1]).unwrap();
    mem.write_u32(r3 + 4, cpu.r[0]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[5] + 8).unwrap();
    mem.write_u32(r3 + 8, cpu.r[0]).unwrap();

    cpu.r[0] = mem.read_u32(r4 + 8).unwrap().wrapping_add(12);
    mem.write_u32(r4 + 8, cpu.r[0]).unwrap();
    cpu.r[1] = mem.read_u32(r4 + 8).unwrap();
    cpu.r[0] = mem.read_u32(r4 + 12).unwrap();
    cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r0,r1 -> T = (r1 == r0)
    if cpu.r[1] == cpu.r[0] {
        cpu.r[0] = mem.read_u32(r4).unwrap();
        mem.write_u32(r4 + 8, cpu.r[0]).unwrap();
    }

    cpu.r[1] = mem.read_u32(r4 + 8).unwrap();
    cpu.r[0] = mem.read_u32(r4 + 4).unwrap();
    cpu.set_t(cpu.r[1] == cpu.r[0]);
    if cpu.r[1] == cpu.r[0] {
        cpu.r[0] = cpu.r[0].wrapping_add(12);
        mem.write_u32(r4 + 4, cpu.r[0]).unwrap();
        cpu.r[1] = mem.read_u32(r4 + 4).unwrap();
        cpu.r[0] = mem.read_u32(r4 + 12).unwrap();
        cpu.set_t(cpu.r[1] == cpu.r[0]);
        if cpu.r[1] == cpu.r[0] {
            cpu.r[0] = mem.read_u32(r4).unwrap();
            mem.write_u32(r4 + 4, cpu.r[0]).unwrap();
        }
    }
    // rts ; nop
}
