//! 0x8c01125c — `*PTR_DAT_8c011264 = r4;` (global pointer store).
//!
//! `mov.l 0x8c011264,r3 ; rts ; mov.l r4,@r3`
//! The delay slot of `rts` performs the store, and leaves the loaded
//! pointer (0x8c1f2eb0, read from the image) in r3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let ptr = mem.read_u32(0x8c011264).unwrap_or(0);
    cpu.r[3] = ptr;
    let _ = mem.write_u32(ptr, cpu.r[4]); // delay slot of `rts`
}
