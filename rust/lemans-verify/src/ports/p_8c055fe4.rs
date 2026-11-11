//! 0x8c055fe4 — `r0 = *(u32*)(r4 + 0x38) + 8`, and the `rts` delay slot
//! `fmov @r0,fr0` loads fr0 from the *incremented* pointer. The C's FPSCR.SZ
//! double branch does not exist in the machine code; only fr0 (single) and
//! r0 change, T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r0 = mem.read_u32(cpu.r[4].wrapping_add(0x38)).unwrap();
    r0 = r0.wrapping_add(8);
    cpu.r[0] = r0;
    cpu.fr[0] = mem.read_f32(r0).unwrap(); // rts delay slot
}
