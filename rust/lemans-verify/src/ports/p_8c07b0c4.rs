//! 0x8c07b0c4 — swap_four_words.
//!
//! Swaps the four words at `r4` and `r5` unless they are the same pointer.
//! `r0` ends as the word copied into `r4+12`, `r1` as the one copied out.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let a = cpu.r[4];
    let b = cpu.r[5];
    let same = a == b;
    cpu.set_t(same); // cmp/eq r5,r4
    if same {
        return;
    }
    cpu.r[1] = mem.read_u32(a).unwrap_or(0);
    let _ = mem.write_u32(a, mem.read_u32(b).unwrap_or(0));
    let _ = mem.write_u32(b, cpu.r[1]);

    cpu.r[1] = mem.read_u32(a + 4).unwrap_or(0);
    cpu.r[0] = mem.read_u32(b + 4).unwrap_or(0);
    let _ = mem.write_u32(a + 4, cpu.r[0]);
    let _ = mem.write_u32(b + 4, cpu.r[1]);

    cpu.r[1] = mem.read_u32(a + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(b + 8).unwrap_or(0);
    let _ = mem.write_u32(a + 8, cpu.r[0]);
    let _ = mem.write_u32(b + 8, cpu.r[1]);

    cpu.r[1] = mem.read_u32(a + 12).unwrap_or(0);
    cpu.r[0] = mem.read_u32(b + 12).unwrap_or(0);
    let _ = mem.write_u32(a + 12, cpu.r[0]);
    let _ = mem.write_u32(b + 12, cpu.r[1]);
}
