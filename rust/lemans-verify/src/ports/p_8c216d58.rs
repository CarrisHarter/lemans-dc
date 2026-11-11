//! 0x8c216d58 — copy_bytes_count.
//!
//! Copies `r6` bytes from `r4` to `r5` when `r6 > 0`; `r6` counts down to 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let pl = (cpu.r[6] as i32) > 0;
    cpu.set_t(pl);
    cpu.r[7] = cpu.r[4];
    cpu.r[4] = cpu.r[5]; // delay slot of `bf.s`
    if pl {
        loop {
            cpu.r[3] = mem.read_u8(cpu.r[7]).unwrap_or(0) as u32;
            cpu.r[7] = cpu.r[7].wrapping_add(1);
            cpu.r[6] = cpu.r[6].wrapping_sub(1);
            let t = (cpu.r[6] as i32) > 0;
            cpu.set_t(t);
            let _ = mem.write_u8(cpu.r[4], cpu.r[3] as u8);
            cpu.r[4] = cpu.r[4].wrapping_add(1); // delay slot of `bt.s`
            if !t {
                break;
            }
        }
    }
}
