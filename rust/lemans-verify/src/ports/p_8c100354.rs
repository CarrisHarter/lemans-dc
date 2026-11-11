//! 0x8c100354 — store fr0 (zeroed by `fldi0`) as eight consecutive f32 at the
//! global pointer 0x8c2ab318 (literal at 0x8c100384), +0..+28; the last store
//! is the `rts` delay slot.
//!
//! Register residue the C does not show: r0/r1/r3/r4/r5 end at +24/+20/+16/+12/
//! +8 and r6 at +28 (the delay-slot `add #28,r6` runs before the store).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(0x8c100384).unwrap_or(0); // 0x8c2ab318
    cpu.r[6] = base;
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[0] = base;
    let _ = mem.write_f32(base, cpu.fr[0]); // fmov fr0,@r6
    cpu.r[0] = base.wrapping_add(4);
    cpu.r[5] = base;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    cpu.r[4] = base;
    cpu.r[3] = base;
    cpu.r[1] = base;
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]); // fmov fr0,@r5
    cpu.r[0] = base;
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(16);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = cpu.r[1].wrapping_add(20);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[6] = cpu.r[6].wrapping_add(28);
    // rts ; fmov fr0,@r6 (delay slot)
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]);
}
