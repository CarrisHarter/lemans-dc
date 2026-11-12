//! 0x8c0ed80c — clear and initialise an 0x50-byte structure.
//!
//! `r4` is the structure base. The routine pushes `r14`/`pr`, zeroes words at
//! offsets 0, 4, 8, 12 and 16, then calls `0x8c0b4ff0` on `base+20` and on
//! `base+32` (that callee zeroes three words at its argument). Finally it
//! zeroes the remaining words at offsets 44, 48, 52, 56, 60, 64, 68, 72 and 76,
//! which completes the 0x50-byte region (the callee writes 20..40).
//! Both `jsr` sites reload the callee pointer from the literal at 0x8c0ed850.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]);
    cpu.r[0] = mem.read_u32(0x8c0ed850).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[1]);
    cpu.pr = 0x8c0ed824;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]); // delay slot of `jsr`
    crate::registry::p_8c0b4ff0::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c0ed850).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.pr = 0x8c0ed82c;
    cpu.r[4] = cpu.r[4].wrapping_add(32); // delay slot of `jsr`
    crate::registry::p_8c0b4ff0::run(cpu, mem);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(44), cpu.r[1]);
    cpu.r[0] = 64;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(48), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(52), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(56), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(60), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = 68;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = 72;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = 76;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
