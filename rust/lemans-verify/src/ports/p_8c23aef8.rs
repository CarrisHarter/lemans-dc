//! 0x8c23aef8 — link_object.
//!
//! Copies two words from `0x8c2c86cc` into `[[r4+0x88]+40` and `+36`, stores
//! `r5`/`r6`/`r7` at `r4+24/+28` and `[[r4+0x88]+28`, then sets bit 0x10000 in
//! `[[r4+0x88]+36]` when `r7` is non-zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u16(0x8c23af44).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[1] = mem.read_u32(0x8c23af6c).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3] + 40, cpu.r[2]);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3] + 36, cpu.r[2]);
    let t = cpu.r[7] == 0;
    cpu.set_t(t);
    let _ = mem.write_u32(cpu.r[4] + 28, cpu.r[6]);
    let _ = mem.write_u32(cpu.r[4] + 24, cpu.r[5]);
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3] + 28, cpu.r[7]);
    if !t {
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.r[2] = mem.read_u32(0x8c23af70).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[3] + 36).unwrap_or(0);
        cpu.r[1] = cpu.r[1] | cpu.r[2];
        let _ = mem.write_u32(cpu.r[3] + 36, cpu.r[1]);
    }
    cpu.r[0] = 0; // delay slot of `rts`
}
