//! 0x8c0b48c8 — copy three floats from `r4+20/24/28` to a stack-passed
//! destination.
//!
//! ```text
//! r3 = [r15] ; fr0 = [r4+20] ; r1 = r3+4 ; [r3] = fr0
//! fr0 = [r4+24] ; r4 += 28 ; r3 += 8 ; [r1] = fr0
//! fr0 = [r4] ; rts ; [r3] = fr0
//! ```
//!
//! Hidden side effects: `r4` is advanced by 28, `r3` by 8, `r1` is `dest+4`,
//! `r0` ends as `r4+24`, and the third store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[4];
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = src.wrapping_add(20);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[1] = cpu.r[3].wrapping_add(4);
    cpu.r[0] = src;
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[4] = src.wrapping_add(28);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // delay slot of `rts`
}
