//! 0x8c0b489c — copy [r4+4],[r4+8],[r4+12],[r4+16] to the 4-word block at
//! r3, where r3=`[r15]` (the word at the entry stack pointer). The last store
//! `fmov fr0,@r3` is the rts delay slot after `add #12,r3`.
//!
//! Leftover registers: r0=r3+8, r1=r3+4, r3=r3+12, r4=r4+16; fr0=[r4+16].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[4];
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let rd = |mem: &Memory, a: u32| mem.read_u32(a).unwrap_or(0);

    let _ = mem.write_u32(dst, rd(mem, src + 4));
    let _ = mem.write_u32(dst + 4, rd(mem, src + 8));
    let _ = mem.write_u32(dst + 8, rd(mem, src + 12));
    let s16 = rd(mem, src + 16);
    let _ = mem.write_u32(dst + 12, s16); // delay slot of `rts`

    cpu.fr[0] = f32::from_bits(s16);
    cpu.r[0] = dst + 8;
    cpu.r[1] = dst + 4;
    cpu.r[3] = dst + 12;
    cpu.r[4] = src + 16;
}
