//! 0x8c06f830 — zero a tail struct, clear a word, store -300.0 and +0.0.
//!
//! 0x8c0ed854 runs on `r4+8` (the `jsr` delay slot adds 8). 0x8c01011c then
//! zeroes `[r14+88]`. The literal at 0x8c06f87c (-300.0) is stored at `[r14]`,
//! `fldi0` at `[r14+4]`, and 0 at +92/+96/+100/+104/+108/+112. `r0 = r14`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c06f874).unwrap_or(0x8c0ed854);
    cpu.r[4] = cpu.r[4].wrapping_add(8); // jsr delay slot
    cpu.pr = 0x8c06f83c;
    crate::registry::p_8c0ed854::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c06f878).unwrap_or(0x8c01011c);
    cpu.r[4] = cpu.r[14].wrapping_add(88); // jsr delay slot
    cpu.pr = 0x8c06f844;
    crate::registry::p_8c01011c::run(cpu, mem);
    cpu.fr[1] = f32::from_bits(mem.read_u32(0x8c06f87c).unwrap_or(0xc3960000));
    cpu.fr[0] = 0.0; // fldi0
    cpu.r[1] = 0;
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[14], cpu.fr[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    for off in [100u32, 104, 108, 112, 92, 96] {
        cpu.r[0] = off;
        let _ = mem.write_u32(cpu.r[14].wrapping_add(off), cpu.r[1]);
    }
    cpu.r[0] = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.r[14] = saved14;
}
