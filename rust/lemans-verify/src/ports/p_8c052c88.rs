//! 0x8c052c88 — copy the three bytes at `r5+0x1f0..0x1f2` to
//! `r4+0x1f0..0x1f2`. The offsets are PC-relative literals at 0x8c052ca4/a/c.
//!
//! Side effects the decompiled C does not show: the third store happens in
//! the `rts` delay slot (`mov.b r1,@(r0,r4)` at 0x8c052ca0), so it always
//! runs; r0 ends as the last literal 0x1f2 and r1 as the last byte copied
//! (extu.b, so zero-extended even though `mov.b` loads sign-extend). T is
//! untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    for lit_addr in [0x8c052ca4u32, 0x8c052ca8, 0x8c052cac] {
        let off = mem.read_u32(lit_addr).unwrap_or(0);
        let v = mem.read_u8(off.wrapping_add(r5)).unwrap();
        mem.write_u8(off.wrapping_add(r4), v).unwrap();
        cpu.r[0] = off;
        cpu.r[1] = v as u32; // extu.b r1,r1
    }
}
