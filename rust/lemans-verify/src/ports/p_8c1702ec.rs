//! 0x8c1702ec — call 0x8c2205d0, return whether it returned 0.
//!
//! Pushes pr, loads `r3 = 0x8c2205d0` from pool word `0x8c17030c`, and calls it
//! (`jsr @r3`, so pr = 0x8c1702f4 on entry; the callee preserves pr and the
//! wrapper restores the caller's pr from the stack). All argument registers
//! pass through untouched. After the call `tst r0,r0` sets T = (callee r0 == 0)
//! and the rts delay slot `movt r0` overwrites r0 with T, so the wrapper
//! returns 1 when the callee returned 0 and 0 otherwise. The callee always
//! returns 0 here, so the observable result is r0 = 1, T set.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov.l 0x8c17030c,r3 (0x8c2205d0)
    cpu.r[3] = mem.read_u32(0x8c17030c).unwrap_or(0);
    // jsr @r3 ; nop (delay slot)
    cpu.pr = 0x8c1702f4;
    crate::registry::p_8c2205d0::run(cpu, mem);
    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // tst r0,r0 ; rts ; movt r0 (delay slot)
    cpu.set_t(cpu.r[0] == 0);
    cpu.r[0] = cpu.t() as u32;
}
