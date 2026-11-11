//! 0x8c1a3a70 — lookup_pair_by_index.
//!
//! Reads an index from `[[r4+4]+72]` (0 when `[r4+4]` is null), compares it with
//! `[r4+16]`, and if `[r4+16] > index` writes the 8-byte entry at
//! `[r4+8] + 8*index` through `r5`/`r6`; otherwise writes 0 and -1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let t = cpu.r[0] == 0;
    cpu.set_t(t);
    let mut r3: u32 = 0;
    if !t {
        cpu.r[0] = cpu.r[0].wrapping_add(72);
        r3 = mem.read_u32(cpu.r[0]).unwrap_or(0);
    }
    cpu.r[3] = r3;
    cpu.r[0] = mem.read_u32(cpu.r[4] + 16).unwrap_or(0);
    let gt = (cpu.r[0] as i32) > (r3 as i32);
    cpu.set_t(gt);
    if gt {
        cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
        r3 = r3 << 2;
        cpu.set_t((r3 >> 31) & 1 != 0); // `shll` sets T from the bit shifted out
        r3 = r3 << 1;
        cpu.r[3] = r3;
        cpu.r[1] = cpu.r[1].wrapping_add(r3);
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[5], cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[1] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6], cpu.r[0]); // delay slot of `rts`
    } else {
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[5], cpu.r[0]);
        cpu.r[0] = u32::MAX;
        let _ = mem.write_u32(cpu.r[6], cpu.r[0]); // delay slot of `rts`
    }
}
