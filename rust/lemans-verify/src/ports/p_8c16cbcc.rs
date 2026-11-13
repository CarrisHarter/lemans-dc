//! 0x8c16cbcc — load the object at `[base+0xf8]` and tail-call `0x8c16d904`.
//!
//! `r4` becomes that object. `pr` is not saved: this is a `jmp`, not a `jsr`.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = rd(mem, 0x8c16cbd8); // 0xf8
    cpu.r[4] = rd(mem, cpu.r[4].wrapping_add(cpu.r[0]));
    cpu.r[0] = rd(mem, 0x8c16cbdc); // 0x8c16d904
    crate::registry::p_8c16d904::run(cpu, mem);
}
