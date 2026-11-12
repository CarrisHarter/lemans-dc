//! 0x8c151524 - stamp a vtable word and clear three sub-objects at +4/+16/+32.
//!
//! `[r14] = 0x8c1ef40c` lands in the first `jsr` delay slot. Each 0x8c0b4ff0
//! call gets `r14+4`, `r14+16`, `r14+32` from its own delay slot. `fldi0` then
//! `fmov` stores +0.0 at `[r14+28]` before the third call.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[1] = mem.read_u32(0x8c151554).unwrap_or(0x8c1ef40c); // mov.l 0x8c151554,r1
    cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
    cpu.r[0] = mem.read_u32(0x8c151558).unwrap_or(0x8c0b4ff0); // mov.l 0x8c151558,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[1]); // jsr delay: mov.l r1,@r14
    cpu.pr = 0x8c151534;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[0] = mem.read_u32(0x8c151558).unwrap_or(0x8c0b4ff0); // mov.l 0x8c151558,r0
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    cpu.r[4] = cpu.r[4].wrapping_add(16); // jsr delay: add #16,r4
    cpu.pr = 0x8c15153c;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[0] = mem.read_u32(0x8c151558).unwrap_or(0x8c0b4ff0); // mov.l 0x8c151558,r0
    cpu.r[1] = cpu.r[14].wrapping_add(28); // mov r14,r1 ; add #28,r1
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits()); // fmov fr0,@r1
    cpu.r[4] = cpu.r[4].wrapping_add(32); // jsr delay: add #32,r4
    cpu.pr = 0x8c15154c;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
