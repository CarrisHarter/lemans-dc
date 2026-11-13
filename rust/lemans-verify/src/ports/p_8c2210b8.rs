//! 0x8c2210b8 — copy twelve bytes when the indexed slot word is clear.
//!
//! Saves `r4`/`r5`, calls `0x8c221da8` (word at `0x8c2bde98 + 128*r4 + 4`),
//! and returns `-1` when that word is nonzero. A clear word restores the
//! arguments and tail-calls `0x8c223f38`. The `jmp` delay slot reloads `pr`.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_pr = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], saved_pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let frame = cpu.r[15];
    let _ = mem.write_u32(frame, cpu.r[4]);
    let _ = mem.write_u32(frame.wrapping_add(4), cpu.r[5]);
    cpu.r[3] = word(mem, 0x8c221310); // 0x8c221da8
    cpu.pr = 0x8c2210c6;
    cpu.r[4] = word(mem, frame); // jsr delay
    crate::registry::p_8c221da8::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        cpu.pr = word(mem, cpu.r[15]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 0xffff_ffff; // rts delay: mov #-1, r0
        return;
    }
    cpu.r[4] = word(mem, frame);
    cpu.r[5] = word(mem, frame.wrapping_add(4));
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.r[3] = word(mem, 0x8c22131c); // 0x8c223f38
    cpu.pr = word(mem, cpu.r[15]); // jmp delay: lds.l @r15+, pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    crate::registry::p_8c223f38::run(cpu, mem);
}
