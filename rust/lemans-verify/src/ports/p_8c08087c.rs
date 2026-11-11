//! 0x8c08087c — init_iterator.
//!
//! Builds an iterator over the `[r4+8]` count from the `[r4+12]` base into the
//! out-struct passed on the stack. `r1` is the count shifted by 2 and `r3` the
//! end pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let out = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[5] = out; // mov.l @r15,r5
    let count = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    let base = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.r[1] = count << 2; // shll2 r1
    cpu.r[0] = 0;
    cpu.r[3] = base.wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(out, cpu.r[4]);
    let _ = mem.write_u32(out + 4, 0);
    let _ = mem.write_u32(out + 8, cpu.r[3]); // delay slot of `rts`
}
