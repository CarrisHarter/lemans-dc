//! 0x8c05fcd0 — build a 16-byte header at r4 from r5 and the global table.
//!
//! Original: pushes r4, loads r6 = [r15] (the saved r4 = param), r0 =
//! mem32[mem32[0x8c05fcf8]], then stores [r6]=r0, [r6+4]=0, byte [r5] to
//! [r6+8], 0 then r6+4 to [r6+12], returns r6.
//!
//! Leftovers the C does not show: r4 = 0, r6 = param, r3 = param+8,
//! r1 = param+4, r0 = param at exit; the [r6+12] slot is written twice
//! (0, then param+4). r15 nets to zero (-4 push, +4 in the rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let param = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], param); // mov.l r4,@r15
    let table = mem.read_u32(0x8c05fcf8).unwrap_or(0); // mov.l 0x8c05fcf8,r0
    let head = mem.read_u32(table).unwrap_or(0); // mov.l @r0,r0
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r6
    cpu.r[0] = head;
    cpu.r[3] = cpu.r[6]; // mov r6,r3
    cpu.r[1] = cpu.r[6]; // mov r6,r1
    let _ = mem.write_u32(cpu.r[6], head); // mov.l r0,@r6
    cpu.r[3] = cpu.r[3].wrapping_add(8); // add #8,r3
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[4]); // mov.l r4,@(4,r6)
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    let b = mem.read_u8(cpu.r[5]).unwrap_or(0); // mov.b @r5,r0
    cpu.r[0] = b as u32;
    let _ = mem.write_u8(cpu.r[3], b); // mov.b r0,@r3
    let _ = mem.write_u32(cpu.r[6].wrapping_add(12), cpu.r[4]); // mov.l r4,@(12,r6)
    cpu.r[0] = cpu.r[6]; // mov r6,r0
    let _ = mem.write_u32(cpu.r[6].wrapping_add(12), cpu.r[1]); // mov.l r1,@(12,r6)
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot: add #4,r15
}
