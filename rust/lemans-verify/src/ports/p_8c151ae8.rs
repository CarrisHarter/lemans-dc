//! 0x8c151ae8 - scale the vector at `r4+88` by fr4 into a 12-byte local, then merge it.
//!
//! The frame is 12 bytes; `mov.l r0,@-r15` pushes the frame pointer so the
//! callee reads it as its stack argument. After the call the frame pointer is
//! r15 itself, which becomes the `r5` argument of 0x8c151340 (the `r4+32`
//! thunk) via its delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // add #-12,r15

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = cpu.r[15]; // mov r15,r0
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@-r15
    cpu.r[0] = cpu.r[14].wrapping_add(88); // mov r14,r0 ; add #88,r0
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr4
    cpu.r[0] = mem.read_u32(0x8c151b14).unwrap_or(0x8c0b4e0c); // mov.l 0x8c151b14,r0
    cpu.r[4] = cpu.r[5]; // jsr delay: mov r5,r4
    cpu.pr = 0x8c151b00;
    crate::registry::p_8c0b4e0c::run(cpu, mem);

    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.r[0] = mem.read_u32(0x8c151b18).unwrap_or(0x8c151340); // mov.l 0x8c151b18,r0
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    cpu.r[5] = cpu.r[15]; // jsr delay: mov r15,r5
    cpu.pr = 0x8c151b0a;
    crate::registry::p_8c151340::run(cpu, mem);

    cpu.r[15] = cpu.r[15].wrapping_add(12); // add #12,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
