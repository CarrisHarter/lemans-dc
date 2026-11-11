//! 0x8c07e018 — advance a cursor and wrap it when it reaches the end.
//!
//! ```text
//! [r4+4] += 1 ; r3 = [r4+8] ; r1 = [r4] ; r3 += 4 ; [r4+8] = r3
//! r0 = [r1] ; r1 = [r1+12] ; r0 <<= 2 ; r1 += r0 ; cmp/eq r3,r1
//! bf done ; [r4+8] = [[r4]+12]
//! done: rts ; r0 = r4
//! ```
//!
//! Hidden side effects: `r0` ends as `r4` from the delay slot on both paths,
//! `r1` is `base + count*4`, `r3` is the advanced cursor, and T is the
//! `cmp/eq` that decides the wrap.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let counter = mem.read_u32(base + 4).unwrap_or(0).wrapping_add(1);
    let _ = mem.write_u32(base + 4, counter);

    let cursor = mem.read_u32(base + 8).unwrap_or(0).wrapping_add(4);
    let _ = mem.write_u32(base + 8, cursor);
    cpu.r[3] = cursor;

    let table = mem.read_u32(base).unwrap_or(0);
    let count = mem.read_u32(table).unwrap_or(0);
    let table_base = mem.read_u32(table + 12).unwrap_or(0);
    cpu.r[1] = table_base.wrapping_add(count << 2);

    let at_end = cpu.r[3] == cpu.r[1];
    cpu.set_t(at_end); // cmp/eq r3,r1
    if at_end {
        let _ = mem.write_u32(base + 8, table_base);
    }
    cpu.r[0] = base; // delay slot of `rts`
}
