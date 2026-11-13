//! 0x8c2210e6 — copy a marker tail only when its 128-byte slot is free.
//!
//! `0x8c221da8` loads the word at `base + 128*r4 + 4`. A nonzero word returns
//! -1 and leaves that scaled address in `r4` (T clear from `tst`). A zero word
//! reloads the caller's `r4` and `r5` and tail-calls `0x8c224030`. Both exits
//! pop the saved `pr`.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // sts.l pr, @-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = rd(mem, 0x8c221310); // 0x8c221da8
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    cpu.pr = 0x8c2210f4;
    cpu.r[4] = rd(mem, cpu.r[15]); // jsr delay slot
    crate::registry::p_8c221da8::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0, r0
    if !cpu.t() {
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        cpu.pr = rd(mem, cpu.r[15]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = (-1i32) as u32; // rts delay slot
        return;
    }
    cpu.r[4] = rd(mem, cpu.r[15]);
    cpu.r[5] = rd(mem, cpu.r[15].wrapping_add(4));
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.r[3] = rd(mem, 0x8c221320); // 0x8c224030
    // jmp delay slot pops pr before the tail call runs
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    crate::registry::p_8c224030::run(cpu, mem);
}
