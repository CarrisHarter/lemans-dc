//! 0x8c16ee30 — set_float_if_flag_96.
//!
//! If bit 0 of `[r4+8]` is set, store `fr4` at `*([r4+92]+96)` when it
//! differs, then OR `0x400000` into `[r4+8]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r0 = mem.read_u32(cpu.r[4] + 8).unwrap_or(0) & 1;
    cpu.r[0] = r0;
    cpu.set_t(r0 == 0);
    if r0 != 0 {
        let mut p = cpu.r[4].wrapping_add(92);
        p = mem.read_u32(p).unwrap_or(0);
        p = p.wrapping_add(96);
        cpu.fr[0] = mem.read_f32(p).unwrap_or(0.0);
        let eq = cpu.fr[0] == cpu.fr[4];
        cpu.set_t(eq);
        cpu.r[0] = p;
        if !eq {
            let _ = mem.write_f32(p, cpu.fr[4]);
            cpu.r[0] = mem.read_u32(0x8c16ee54).unwrap_or(0);
            let mut r1 = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
            r1 |= cpu.r[0];
            let _ = mem.write_u32(cpu.r[4] + 8, r1);
            cpu.r[1] = r1;
        }
    }
}
