//! 0x8c0fe9fc — construct an object at `r4` with vtable 0x8c1ed104.
//!
//! Saves `r14`/`pr`, then calls 0x8c0b5e78 with `r4` unchanged: that callee
//! stores `0x8c1eaaac` to `[r4]` and returns `r4` in `r0`; both effects are
//! dead because the tail overwrites `[r4]` and `r0`. The tail stamps
//! `[r4] = 0x8c1ed104` after the call, zeroes `+4`, `+28` (0x1c), `+36`
//! (0x24) and `+48` (0x30), and leaves `r4` in `r0`. `r14`/`pr` are restored;
//! the `rts` delay slot pops `r14`, so `r14` ends at its entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c0fea20).unwrap_or(0); // 0x8c0b5e78
    // `jsr @r0`; the delay slot is `nop`, so only `pr` changes before the call.
    cpu.pr = 0x8c0fea08;
    crate::registry::p_8c0b5e78::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c0fea24).unwrap_or(0); // 0x8c1ed104
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[14].wrapping_add(28), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(36), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(48), cpu.r[1]);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
