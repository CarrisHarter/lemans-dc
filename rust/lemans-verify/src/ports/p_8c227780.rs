//! 0x8c227780 — validate r4 fields, bit-pack into struct; rts r0=10 or 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = 0x179f8; // mov.l 0x8c2277e0,r3
    cpu.r[7] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let t1 = cpu.r[7] & cpu.r[3] == 0; // tst r7,r3
    cpu.set_t(t1);
    if t1 {
        cpu.r[0] = 10;
        cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay
        return;
    }
    cpu.r[1] = 0x101f8; // mov.l 0x8c2277e4,r1
    let t2 = cpu.r[7] & cpu.r[1] == 0; // tst r7,r1
    cpu.set_t(t2);
    if !t2 {
        cpu.r[0] = cpu.r[5];
        let t3 = cpu.r[0] == 1; // cmp/eq #1,r0
        cpu.set_t(t3);
        if t3 {
            cpu.r[0] = 10;
            cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay
            return;
        }
    }
    // main path from 0x8c22779c
    cpu.r[7] = cpu.r[5];
    cpu.set_t(cpu.r[7] >> 31 != 0); // shll r7
    cpu.r[7] = cpu.r[7] << 1;
    cpu.r[7] = cpu.r[7] << 2; // shll2 r7
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
    cpu.r[3] >>= 2; // shlr2 r3
    cpu.r[0] &= cpu.r[2];
    cpu.r[2] = 0x1fffff;
    cpu.set_t(cpu.r[3] & 1 != 0); // shlr r3
    cpu.r[3] >>= 1;
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] |= cpu.r[3];
    let _ = mem.write_u32(cpu.r[1] + 12, cpu.r[0]);
    cpu.r[0] = 0; // mov #0,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay
}
