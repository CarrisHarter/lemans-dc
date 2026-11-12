//! 0x8c0b633c — construct/label an object at `r4`.
//!
//! Saves `r14`/`pr`, calls 0x8c0b5e78 with `r4` unchanged: that callee stores
//! `0x8c1eaaac` to `[r4]` and returns `r4` in `r0`; both effects are dead here
//! because the tail overwrites `[r4]` and `r0`. The tail stamps
//! `[r4] = 0x8c1eaaf0`, zeroes `+4`, `+8`, `+12`, the byte at `+16`
//! (`mov #16,r0 ; mov.b r1,@(r0,r14)`) and `+20`, then returns `r4` in `r0`
//! with `r1 = 0`. `r14`/`pr` are restored by the epilogue; the `rts` delay slot
//! pops `r14`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c0b6364).unwrap_or(0); // 0x8c0b5e78
    // `jsr @r0`; the delay slot is `nop`, so only `pr` changes before the call.
    cpu.pr = 0x8c0b6348;
    crate::registry::p_8c0b5e78::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c0b6368).unwrap_or(0); // 0x8c1eaaf0
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    cpu.r[0] = 16;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[1]);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1] as u8);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[1]);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
