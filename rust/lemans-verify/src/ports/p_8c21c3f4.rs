//! 0x8c21c3f4 — fill_words.
//!
//! Stores `r5` at `r4 + 4*i` for i in `[0, r6)` when `r6 > 0`; returns `r6`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let pl = (cpu.r[6] as i32) > 0;
    cpu.set_t(pl);
    cpu.r[7] = 0;
    cpu.r[0] = cpu.r[4]; // delay slot of `bf.s`
    if pl {
        loop {
            cpu.r[7] = cpu.r[7].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[0], cpu.r[5]);
            let ge = (cpu.r[7] as i32) >= (cpu.r[6] as i32);
            cpu.set_t(ge);
            cpu.r[0] = cpu.r[0].wrapping_add(4); // delay slot of `bf.s`
            if ge {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // delay slot of `rts`
}
