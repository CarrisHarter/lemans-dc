//! 0x8c217aa4 — word_field_82.
//!
//! Returns `word[r4+76]`-gated `word[r4+82]`, or 0 when `r4` is null
//! or the gate word is zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = 0 as u32; // delay slot of `rts`
        return;
    }
    cpu.r[0] = 76;
    cpu.r[3] = mem.read_u16(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as i16 as i32 as u32;
    let t2 = cpu.r[3] == 0;
    cpu.set_t(t2);
    if t2 {
        cpu.r[0] = 0 as u32; // delay slot of `rts`
        return;
    }
    cpu.r[0] = 82;
    cpu.r[0] = mem.read_u16(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as i16 as i32 as u32; // delay slot of `rts`
}
