//! 0x8c0ffd38 — same node init as 0x8c0fd168, id cell at 0x8c2032f0 (lit 0x8c0ffd60).
//! Hidden side effects: r4 = 0, r6 = param, r3 = param+8, r1 = param+4,
//! r0 = param, double store to node+12 (0 then param+4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let param = cpu.r[4]; // mov.l r4,@r15
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[6] = param; // mov.l @r15,r6
    cpu.r[0] = mem.read_u32(0x8c0ffd60).unwrap(); // 0x8c2032f0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // mov.l @r0,r0
    cpu.r[3] = cpu.r[6];
    cpu.r[1] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]); // mov.l r0,@r6
    cpu.r[3] = cpu.r[3].wrapping_add(8); // add #8,r3
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]); // mov.l r4,@(4,r6)
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32; // mov.b @r5,r0
    let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8); // mov.b r0,@r3
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[4]); // mov.l r4,@(12,r6)
    cpu.r[0] = cpu.r[6]; // mov r6,r0
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[1]); // mov.l r1,@(12,r6)
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot: add #4,r15
}
