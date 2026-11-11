//! 0x8c0848d8 — same shape as 0x8c083b9c, but the vtable comes from *(0x8c1f94c0).
//!
//! Side effects the C does not show: r4 zeroed, r6 := param, r3 := param+8,
//! r1 := param+4, r0 := param at exit; r15 net unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15 (stack window)
    let param = cpu.r[4];
    cpu.r[6] = param; // mov.l @r15,r6
    let ptr = mem.read_u32(0x8c084900).unwrap_or(0); // mov.l literal -> 0x8c1f94c0
    cpu.r[0] = mem.read_u32(ptr).unwrap_or(0); // mov.l @r0,r0
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[3] = param.wrapping_add(8); // add #8,r3
    cpu.r[1] = param.wrapping_add(4); // add #4,r1
    let _ = mem.write_u32(param, cpu.r[0]); // mov.l r0,@r6
    let _ = mem.write_u32(param.wrapping_add(4), 0); // mov.l r4,@(4,r6)
    let b = mem.read_u8(cpu.r[5]).unwrap_or(0); // mov.b @r5,r0
    cpu.r[0] = b as u32;
    let _ = mem.write_u8(param.wrapping_add(8), b); // mov.b r0,@r3
    let _ = mem.write_u32(param.wrapping_add(12), 0); // mov.l r4,@(12,r6)
    cpu.r[0] = param; // mov r6,r0
    let _ = mem.write_u32(param.wrapping_add(12), cpu.r[1]); // mov.l r1,@(12,r6)
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot
}
