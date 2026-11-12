//! 0x8c086aa4 — fill a 10-float (40-byte) record: four zeros, then three
//! −1e10, then three +1e10. Returns the record pointer in `r0`.
//!
//! Saves `r14`/`pr`, calls `0x8c08c05c` (r4 unchanged) to store 0.0 at
//! `r4+12`, then stores 0.0 at +0/+4/+8 from `fldi0 fr0`, loads the literal at
//! 0x8c086af8 (`0xd01502f9` = −1e10) into fr0 for +16/+20/+24, and the
//! literal at 0x8c086afc (`0x501502f9` = +1e10) for +28/+32/+36. The
//! Ghidra FPSCR_SZ branch is fiction: this is straight-line single-precision
//! code.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c086af4).unwrap_or(0);
    cpu.pr = 0x8c086ab0; // jsr @r0; return address, delay slot nop
    crate::registry::p_8c08c05c::run(cpu, mem);
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]); // fmov fr0,@r14 (+0)
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1 (+4)
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0 (+8)
    cpu.r[3] = cpu.r[14];
    cpu.r[0] = 0x8c086af8; // mova 0x8c086af8,r0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // fmov @r0,fr0
    cpu.r[3] = cpu.r[3].wrapping_add(16);
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3 (+16)
    cpu.r[1] = cpu.r[1].wrapping_add(20);
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1 (+20)
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0 (+24)
    cpu.r[4] = cpu.r[14];
    cpu.r[0] = 0x8c086afc; // mova 0x8c086afc,r0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // fmov @r0,fr0
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    cpu.r[3] = cpu.r[14];
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // fmov fr0,@r4 (+28)
    cpu.r[3] = cpu.r[3].wrapping_add(32);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3 (+32)
    cpu.r[1] = cpu.r[1].wrapping_add(36);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1 (+36)
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
