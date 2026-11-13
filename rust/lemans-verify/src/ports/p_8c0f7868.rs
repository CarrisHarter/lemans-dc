//! 0x8c0f7868 — stamp a vtable, drain the shared list, then maybe free the object.
//!
//! A null object returns 0 with T set. Otherwise the delay slot of the call
//! stores `0x8c1ecf74` at `[object]` and `0x8c0f8960(object, 0)` overwrites
//! that with `0x8c1ed000`, decrements the shared counter, and releases the
//! slots at `object+36` and `object+4`. The incoming count is not forwarded, so
//! the drain never frees. A positive low half of the saved count then frees
//! the object through `0x8c01032c`. `r0` is the object. `r13`/`r14` restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.r[1] = rd(mem, 0x8c0f7898); // 0x8c1ecf74
        cpu.r[5] = 0;
        cpu.r[0] = rd(mem, 0x8c0f789c); // 0x8c0f8960
        cpu.pr = 0x8c0f7880;
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]); // jsr delay
        crate::registry::p_8c0f8960::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c0f78a0); // 0x8c01032c
            cpu.pr = 0x8c0f788c;
            cpu.r[4] = cpu.r[13]; // jsr delay
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
