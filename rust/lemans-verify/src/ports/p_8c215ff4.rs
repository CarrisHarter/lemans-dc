//! 0x8c215ff4 — fill_ff.
//!
//! Fills `r5` bytes at `r4` with 0xff when `r5 > 0`; `r5` counts down to 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let pl = (cpu.r[5] as i32) > 0;
    cpu.set_t(pl);
    cpu.r[6] = u32::MAX; // delay slot of `bf.s`
    if pl {
        loop {
            cpu.r[5] = cpu.r[5].wrapping_sub(1);
            let _ = mem.write_u8(cpu.r[4], cpu.r[6] as u8);
            let t = (cpu.r[5] as i32) > 0;
            cpu.set_t(t);
            cpu.r[4] = cpu.r[4].wrapping_add(1); // delay slot of `bt.s`
            if !t {
                break;
            }
        }
    }
}
