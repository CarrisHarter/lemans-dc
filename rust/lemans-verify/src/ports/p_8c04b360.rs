//! 0x8c04b360 — `if (*(u32*)(r4+0x24) != 6) return *(u8*)(r4+0x98);`
//! else `return (*(i32*)(r4+0x9c) > r5);`
//!
//! Hidden side effects the C does not show: r0 is loaded with the table
//! offsets 0x98/0x9c from the literal pool before being reloaded with the
//! result (only the final value survives), the byte path leaves T clear, and
//! on the ==6 path `movt r0` makes r0 and T identical (0 or 1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap(); // mov.l @(36,r4),r0
    cpu.set_t(cpu.r[0] == 6); // cmp/eq #6,r0
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8C04B37C).unwrap(); // 0x98
        cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap() as i8 as i32 as u32; // mov.b @(r0,r4),r0
        cpu.r[0] &= 0xFF; // rts delay slot: extu.b r0,r0
        return;
    }
    cpu.r[0] = mem.read_u32(0x8C04B380).unwrap(); // 0x9c
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap(); // mov.l @(r0,r4),r0
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[5] as i32)); // cmp/gt r5,r0
    cpu.r[0] = cpu.t() as u32; // movt r0
    // rts ; nop
}
