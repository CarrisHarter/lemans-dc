//! 0x8c155dfc — byte[r4+16] = (fr4 == 0) ? 0 : 1; rts delay *(r4) = fr4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = 0.0; // fldi0 fr0
    let eq = cpu.fr[4] == 0.0;
    cpu.set_t(eq); // fcmp/eq fr0,fr4
    cpu.r[0] = 16;
    cpu.r[1] = if eq { 0 } else { 1 };
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u8); // bra delay
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // rts delay slot
}
