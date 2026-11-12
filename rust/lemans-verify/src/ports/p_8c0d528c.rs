//! 0x8c0d528c — scale two 3-float vectors in place by the literal at
//! 0x8c0d52b0 (-1.0): 0x8c0b4ef0(record+12), then 0x8c0b4ef0(record+24).
//!
//! The literal is reloaded into fr4 before each call via `mova`/`fmov`, and
//! the callee scales record[3..5] and record[6..8] (floats) by fr4. Residue:
//! r0 = record+28, r4 = record+32 (both from the second callee); r14 is
//! restored by the rts delay slot and pr by the `lds.l`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0d528c..8c0d5290 — save r14/pr; r14 = record.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];

    // 8c0d5292..8c0d5296 — fr4 = [0x8c0d52b0] (-1.0); r0 = [0x8c0d52b4].
    cpu.r[0] = 0x8c0d52b0; // mova
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = mem.read_u32(0x8c0d52b4).unwrap_or(0);
    cpu.pr = 0x8c0d529c;
    cpu.r[4] = cpu.r[4].wrapping_add(12); // delay slot of `jsr`
    crate::registry::p_8c0b4ef0::run(cpu, mem);

    // 8c0d529c..8c0d52a0 — reload fr4; r14 += 24.
    cpu.r[0] = 0x8c0d52b0; // mova
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[14] = cpu.r[14].wrapping_add(24);

    // 8c0d52a2..8c0d52a6 — 0x8c0b4ef0(record+24); delay: r4 = r14.
    cpu.r[0] = mem.read_u32(0x8c0d52b4).unwrap_or(0);
    cpu.pr = 0x8c0d52a8;
    cpu.r[4] = cpu.r[14]; // delay slot of `jsr`
    crate::registry::p_8c0b4ef0::run(cpu, mem);

    // 8c0d52a8..8c0d52ac — restore pr via `lds.l`, r14 in the rts delay slot.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
