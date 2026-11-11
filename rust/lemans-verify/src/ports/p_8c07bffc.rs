//! 0x8c07bffc — flag_to_float.
//!
//! Reads the byte at `r4+32`, turns it into `fr1` (1.0 or 0.0), compares with
//! zero, and returns the flag as an integer in `r1`/`fpul` and as a float in
//! `fr0`. The byte is then cleared. T is the `fcmp/eq` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let flag = mem.read_u8(base + 32).unwrap_or(0);
    cpu.r[0] = flag as u32; // mov.b + extu.b + extu.b

    let nonzero = flag != 0;
    cpu.set_t(nonzero);
    cpu.fr[1] = if nonzero { 1.0 } else { 0.0 }; // fldi1 on the bra delay slot
    cpu.fr[0] = 0.0; // fldi0 fr0

    let equal = cpu.fr[0] == cpu.fr[1];
    cpu.set_t(equal); // fcmp/eq fr0,fr1
    cpu.r[1] = 0u32.wrapping_sub(equal as u32); // subc r1,r1
    cpu.r[1] = cpu.r[1].wrapping_add(1); // add #1,r1

    cpu.r[0] = 32;
    cpu.r[3] = 0;
    let _ = mem.write_u8(base + 32, 0); // mov.b r3,@(r0,r4)
    cpu.fpul = cpu.r[1]; // lds r1,fpul
    cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0 (delay slot)
}
