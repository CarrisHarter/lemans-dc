//! 0x8c14add8 — copy 44-byte record r5 -> r4; returns r4

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14add8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[14]).unwrap();
    // 0x8c14adda: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c14addc: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14adde: mov r5,r7
    cpu.r[7] = cpu.r[5];
    // 0x8c14ade0: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c14ade2: fmov fr0,@r4
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap();
    // 0x8c14ade4: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14ade6: mov.l @(4,r5),r0
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap();
    // 0x8c14ade8: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14adea: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14adec: mov.l r0,@(4,r4)
    mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]).unwrap();
    // 0x8c14adee: add #32,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(32) as u32;
    // 0x8c14adf0: mov.l @(8,r5),r0
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap();
    // 0x8c14adf2: add #32,r14
    cpu.r[14] = (cpu.r[14] as i32).wrapping_add(32) as u32;
    // 0x8c14adf4: add #36,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(36) as u32;
    // 0x8c14adf6: mov.l r0,@(8,r4)
    mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]).unwrap();
    // 0x8c14adf8: add #36,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(36) as u32;
    // 0x8c14adfa: mov.l @(12,r5),r13
    cpu.r[13] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap();
    // 0x8c14adfc: add #40,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(40) as u32;
    // 0x8c14adfe: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14ae00: mov.l r13,@(12,r4)
    mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[13]).unwrap();
    // 0x8c14ae02: mov.l @(16,r5),r13
    cpu.r[13] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap();
    // 0x8c14ae04: mov.l r13,@(16,r4)
    mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[13]).unwrap();
    // 0x8c14ae06: mov.l @(20,r5),r13
    cpu.r[13] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap();
    // 0x8c14ae08: mov.l r13,@(20,r4)
    mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[13]).unwrap();
    // 0x8c14ae0a: mov.l @(24,r5),r13
    cpu.r[13] = mem.read_u32(cpu.r[5].wrapping_add(24)).unwrap();
    // 0x8c14ae0c: mov.l r13,@(24,r4)
    mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[13]).unwrap();
    // 0x8c14ae0e: mov.l @(28,r5),r13
    cpu.r[13] = mem.read_u32(cpu.r[5].wrapping_add(28)).unwrap();
    // 0x8c14ae10: mov.l r13,@(28,r4)
    mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[13]).unwrap();
    // 0x8c14ae12: add #40,r5
    cpu.r[5] = (cpu.r[5] as i32).wrapping_add(40) as u32;
    // 0x8c14ae14: fmov @r7,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap();
    // 0x8c14ae16: fmov fr0,@r14
    mem.write_f32(cpu.r[14], cpu.fr[0]).unwrap();
    // 0x8c14ae18: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14ae1a: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14ae1c: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14ae1e: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14ae20: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14ae24: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
}
