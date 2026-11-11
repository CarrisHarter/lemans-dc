//! 0x8c19f538 — set_mode_and_clear.
//!
//! Stores `r5` at `[r4+4]` unconditionally (the `bf.s` delay slot), then when
//! `r5 == 8` and `byte[r4+8]` is zero clears `byte[r4+48]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5];
    let eq = cpu.r[0] == 8;
    cpu.set_t(eq);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[5]); // delay slot of `bf.s`
    if eq {
        cpu.r[0] = mem.read_u8(cpu.r[4] + 8).unwrap_or(0) as u32;
        cpu.r[0] = cpu.r[0] & 0xff;
        let t = cpu.r[0] == 0;
        cpu.set_t(t);
        if t {
            cpu.r[0] = 48;
            cpu.r[1] = 0;
            let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
        }
    }
}
