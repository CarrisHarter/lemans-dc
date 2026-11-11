//! 0x8c218156 — copy_field_if_version.
//!
//! When `r4` is non-null and `[r4] >= 2`, copies `[r4+16]` to `[r4+60]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[2] = 2;
        let ge = (cpu.r[3] as i32) >= (cpu.r[2] as i32);
        cpu.set_t(ge);
        if ge {
            cpu.r[4] = cpu.r[4].wrapping_add(16);
            cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[4] + 44, cpu.r[3]);
        }
    }
}
