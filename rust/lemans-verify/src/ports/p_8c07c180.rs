//! 0x8c07c180 — construct an object at `r4`, optionally initialising the
//! embedded storage through 0x8c082054.
//!
//! Saves `r14`/`pr`. When the sign-extended low half of `r5` is non-zero,
//! stores `r4 + 36` at `[r4]` and calls 0x8c082054 on that address. Then
//! unconditionally stores the class word at `r4 + 4`, the vtable word at
//! `*[r4]`, the self-relative offset `(r4 + 36) - *[r4]` at `*[r4] + 24`,
//! zeroes the six words at `r4 + 8 … r4 + 28` and the byte at `r4 + 32`, and
//! returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
    let zero = cpu.r[0] == 0;
    cpu.set_t(zero);
    if !zero {
        cpu.r[4] = cpu.r[14].wrapping_add(36);
        let _ = mem.write_u32(cpu.r[14], cpu.r[4]);
        cpu.r[0] = mem.read_u32(0x8C07_C1C4).unwrap_or(0);
        cpu.pr = 0x8C07_C196; // return address of the `jsr`
        crate::registry::p_8c082054::run(cpu, mem);
    }
    cpu.r[1] = mem.read_u32(0x8C07_C1C8).unwrap_or(0);
    cpu.r[3] = cpu.r[14].wrapping_add(36);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(32);
    let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
    cpu.r[4] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.r[0] = 32;
    cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[4]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(28), cpu.r[1]);
    let _ = mem.write_u8(cpu.r[14].wrapping_add(32), 0);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
