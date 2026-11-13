//! 0x8c156b34 — free the child tree at `[object+4]`, then clear the three header words.
//!
//! A zero child word returns with `r0 = 0` and T set. Otherwise `0x8c156e20`
//! runs with `r5 = [object+4]`, then the object is rewritten: `[object] = 0`,
//! `[object+4] = 0`, `[object+12] = object+4`. `r1` is 0 and `r0` is `object+4`.
//! `r14` is restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = rd(mem, cpu.r[14].wrapping_add(4));
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[14].wrapping_add(4);
        cpu.r[5] = rd(mem, cpu.r[0]);
        cpu.r[0] = rd(mem, 0x8c156b60);
        cpu.pr = 0x8c156b4c;
        crate::registry::p_8c156e20::run(cpu, mem);
        cpu.r[1] = 0;
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
        cpu.r[0] = cpu.r[14].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[0]);
    }
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
