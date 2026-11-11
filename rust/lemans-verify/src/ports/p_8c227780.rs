//! 0x8c227780 — validate r4 fields, bit-pack into struct; rts r0=10 or 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = 0x179f8;
    cpu.r[7] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    if cpu.r[7] & cpu.r[3] == 0 || cpu.r[7] & 0x101f8 == 0 || cpu.r[5] == 1 {
        // bt taken when T=1 (mask==0); all three must be zero-T... emulate: if any T==1 -> r0=10
    }
    let t1 = cpu.r[7] & 0x179f8 == 0;
    let t2 = cpu.r[7] & 0x101f8 == 0;
    let t3 = cpu.r[5] == 1;
    if t1 || t2 || t3 { cpu.r[0] = 10; cpu.sr |= 1; cpu.r[15] = cpu.r[15].wrapping_add(4); return; }
    cpu.r[7] = cpu.r[5] << 1;
    cpu.r[7] <<= 2;
    cpu.r[3] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[15], cpu.r[7]);
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]).wrapping_add(8);
    cpu.r[0] = mem.read_u32(cpu.r[6] + 24).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[3] = 0xffffffc0;
    cpu.r[0] &= 63;
    cpu.r[1] = cpu.r[4];
    cpu.r[2] &= cpu.r[3];
    cpu.r[2] |= cpu.r[0];
    let _ = mem.write_u32(cpu.r[7], cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[6] + 24).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[6] + 28).unwrap_or(0);
    cpu.r[0] |= cpu.r[2];
    cpu.r[2] = 0xffe00000;
    cpu.r[3] >>= 2;
    cpu.r[0] &= cpu.r[2];
    cpu.r[2] = 0x1fffff;
    cpu.r[3] >>= 1;
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] |= cpu.r[3];
    let _ = mem.write_u32(cpu.r[1] + 12, cpu.r[0]);
    cpu.r[0] = 0; cpu.sr |= 1;
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay
}
