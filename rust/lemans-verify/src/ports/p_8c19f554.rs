//! 0x8c19f554 — set_flag_byte.
//!
//! If `byte[r4+8]` already equals `r5 & 0xff` nothing changes; otherwise, when
//! `r5 & 0xff` is zero and `[r4+4] == 8`, clears `byte[r4+48]`, then stores
//! `r5` at `byte[r4+8]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4] + 8).unwrap_or(0) as u32;
    cpu.r[1] = cpu.r[5] & 0xff;
    cpu.r[0] = cpu.r[0] & 0xff;
    let eq = cpu.r[1] == cpu.r[0];
    cpu.set_t(eq);
    if !eq {
        let t = cpu.r[1] == 0;
        cpu.set_t(t);
        let mut clear = false;
        if t {
            cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
            let e8 = cpu.r[0] == 8;
            cpu.set_t(e8);
            clear = e8;
        }
        if clear {
            cpu.r[0] = 48;
            cpu.r[1] = 0;
            let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
        }
        cpu.r[0] = cpu.r[5];
        let _ = mem.write_u8(cpu.r[4] + 8, cpu.r[0] as u8);
    }
}
