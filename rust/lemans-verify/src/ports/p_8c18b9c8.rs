//! 0x8c18b9c8 — bounds-checked element lookup.
//!
//! `if (index >= 0 && index < *base) return index * 0x14 + base[1];`
//! `return base[2];`

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let index = cpu.r[5] as i32;
    let count = mem.read_u32(base).unwrap_or(0) as i32;
    // `cmp/pz` then `cmp/gt` leave T observable on every path.
    if index < 0 {
        cpu.set_t(false);
        cpu.r[0] = mem.read_u32(base + 8).unwrap_or(0);
        return;
    }
    cpu.set_t(count > index);
    cpu.r[0] = if count > index {
        let stride = mem.read_u32(base + 4).unwrap_or(0);
        cpu.r[1] = stride; // the original leaves base[1] in r1 on this path
        // The hit path is `shll2 r0 ; add r5,r0 ; shll2 r0`, so the second
        // shll2's carry (bit 30 of index*5) is the final T.
        cpu.set_t((index as u32).wrapping_mul(5) >> 30 & 1 != 0);
        (index as u32).wrapping_mul(0x14).wrapping_add(stride)
    } else {
        mem.read_u32(base + 8).unwrap_or(0)
    };
}
