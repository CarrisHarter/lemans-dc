//! 0x8c1552b4 — copy 11 words from `r5` to the pointer stored at `[r4+8]`.
//!
//! ```text
//! r3 = [r4+8]
//! r1 = [r5+0] ; r0 = [r5+4] ; [r3+0] = r1 ; [r3+4] = r0
//! ... through +32/+36 ...
//! r0 = [r5+40] ; rts ; [r3+40] = r0
//! ```
//!
//! Hidden side effects: `r1` keeps the word at `r5+32` and `r0` the word at
//! `r5+40`, the last store is the delay slot, and `r3` keeps the destination.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    for i in 0..11u32 {
        let v = mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0);
        // The tail loads the 11th word into r0, not r1.
        if i == 10 || i % 2 == 1 {
            cpu.r[0] = v;
        } else {
            cpu.r[1] = v;
        }
        let _ = mem.write_u32(cpu.r[3] + i * 4, v);
    }
}
