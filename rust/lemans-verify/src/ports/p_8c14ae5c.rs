//! 0x8c14ae5c - reset the sub-array at `r4+40`, then stamp -1 at `[r4+52]`.
//!
//! The `jsr` delay slot adds 40 to r4. `mov #-1,r0` and `mov #0,r1` run after
//! the call, so r0 = 0xffffffff at exit and the byte at +56 is cleared.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c14ae78).unwrap_or(0x8c14ce1c); // mov.l 0x8c14ae78,r0
    cpu.r[4] = cpu.r[4].wrapping_add(40); // jsr delay: add #40,r4
    cpu.pr = 0x8c14ae68;
    crate::registry::p_8c14ce1c::run(cpu, mem);
    cpu.r[0] = 0xffff_ffff; // mov #-1,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(52), cpu.r[0]); // mov.l r0,@(52,r14)
    cpu.r[1] = 0; // mov #0,r1
    cpu.r[0] = 56; // mov #56,r0
    let _ = mem.write_u8(cpu.r[14].wrapping_add(56), cpu.r[1] as u8); // mov.b r1,@(r0,r14)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
