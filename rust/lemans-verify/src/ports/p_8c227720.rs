//! 0x8c227720 — gated bit insert.
//!
//! Two mask gates on `[r4+4]` and `r5 == 1`, then a masked insert at a fixed
//! shift into the word at `r4 + 8*r5 + 8`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x179f8;
    cpu.r[7] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let t1 = (cpu.r[7] & cpu.r[2]) == 0;
    cpu.set_t(t1);
    if t1 {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[1] = 0x101f8;
    let t2 = (cpu.r[7] & cpu.r[1]) == 0;
    cpu.set_t(t2);
    if !t2 {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[0] = cpu.r[5];
    let eq = cpu.r[0] == 1;
    cpu.set_t(eq);
    if !eq {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[7] = cpu.r[5];
    cpu.set_t((cpu.r[7] >> 31) & 1 != 0); // `shll r7` sets T
    cpu.r[7] = cpu.r[7] << 1;
    cpu.r[3] = cpu.r[4];
    cpu.r[7] = cpu.r[7] << 2;
    cpu.r[2] = 1;
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    cpu.r[3] = 0xefff;
    cpu.r[6] = cpu.r[6] & cpu.r[2];
    cpu.r[7] = cpu.r[7].wrapping_add(8);
    cpu.r[1] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[6] = cpu.r[6] << 8;
    cpu.r[6] = cpu.r[6] << 2;
    cpu.r[6] = cpu.r[6] << 2;
    cpu.r[1] = cpu.r[1] & cpu.r[3];
    cpu.r[6] = cpu.r[6] | cpu.r[1];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[7], cpu.r[6]);
}
