//! 0x8c0c9d90 — invoke the byte-store hook on `[r4+4]` when non-null.
//!
//! `sts.l pr,@-r15` saves the return address, `[r4+4]` is loaded into `r4`,
//! and when it is non-null the function tail-calls the hook at 0x8c103f94
//! with that pointer as its argument. The callee stores the low byte of `r5`
//! at `r4+4` and leaves it in `r0`; the null path leaves `r0` untouched. `T`
//! carries the `tst` result (set on null) through to the caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    let null = cpu.r[4] == 0;
    cpu.set_t(null);
    if !null {
        cpu.r[0] = 0x8c103f94;
        // `jsr @r0` with a nop delay slot.
        crate::registry::p_8c103f94::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
