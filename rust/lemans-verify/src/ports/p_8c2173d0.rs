//! 0x8c2173d0 — get_field_if_gated.
//!
//! Returns 1 and writes `[r4+12]` through `r5` when `r4` is non-null and the
//! word at `[r4+76]` is non-zero; otherwise returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = 0; // delay slot of `rts`
        return;
    }
    cpu.r[0] = 76;
    cpu.r[3] = mem.read_u16(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as i16 as i32 as u32;
    let t2 = cpu.r[3] == 0;
    cpu.set_t(t2);
    if t2 {
        cpu.r[0] = 0; // delay slot of `rts`
        return;
    }
    cpu.r[3] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.r[0] = 1;
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
}
