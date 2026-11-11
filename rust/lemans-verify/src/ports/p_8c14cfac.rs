//! 0x8c14cfac — copy 48-byte record r5 -> r4; returns r4

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14cfac: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[14]).unwrap();
    // 0x8c14cfae: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c14cfb0: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[12]).unwrap();
    // 0x8c14cfb2: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14cfb4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c14cfb6: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c14cfb8: fmov fr0,@r4
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap();
    // 0x8c14cfba: mov r5,r6
    cpu.r[6] = cpu.r[5];
    // 0x8c14cfbc: mov.l @(4,r5),r0
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap();
    // 0x8c14cfbe: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14cfc0: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14cfc2: mov.l r0,@(4,r4)
    mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]).unwrap();
    // 0x8c14cfc4: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cfc6: mov.l @(8,r5),r0
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap();
    // 0x8c14cfc8: add #32,r14
    cpu.r[14] = (cpu.r[14] as i32).wrapping_add(32) as u32;
    // 0x8c14cfca: add #32,r13
    cpu.r[13] = (cpu.r[13] as i32).wrapping_add(32) as u32;
    // 0x8c14cfcc: mov.l r0,@(8,r4)
    mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]).unwrap();
    // 0x8c14cfce: add #36,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(36) as u32;
    // 0x8c14cfd0: mov.l @(12,r5),r0
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap();
    // 0x8c14cfd2: add #36,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(36) as u32;
    // 0x8c14cfd4: add #40,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(40) as u32;
    // 0x8c14cfd6: mov.l r0,@(12,r4)
    mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[0]).unwrap();
    // 0x8c14cfd8: add #40,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(40) as u32;
    // 0x8c14cfda: mov.l @(16,r5),r12
    cpu.r[12] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap();
    // 0x8c14cfdc: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14cfde: mov.l r12,@(16,r4)
    mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[12]).unwrap();
    // 0x8c14cfe0: mov.l @(20,r5),r12
    cpu.r[12] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap();
    // 0x8c14cfe2: mov.l r12,@(20,r4)
    mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[12]).unwrap();
    // 0x8c14cfe4: mov.l @(24,r5),r12
    cpu.r[12] = mem.read_u32(cpu.r[5].wrapping_add(24)).unwrap();
    // 0x8c14cfe6: mov.l r12,@(24,r4)
    mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[12]).unwrap();
    // 0x8c14cfe8: mov.l @(28,r5),r12
    cpu.r[12] = mem.read_u32(cpu.r[5].wrapping_add(28)).unwrap();
    // 0x8c14cfea: mov.l r12,@(28,r4)
    mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[12]).unwrap();
    // 0x8c14cfec: fmov @r14,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap();
    // 0x8c14cfee: fmov fr0,@r13
    mem.write_f32(cpu.r[13], cpu.fr[0]).unwrap();
    // 0x8c14cff0: fmov @r6,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    // 0x8c14cff2: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14cff4: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14cff6: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cff8: mov.l @(44,r5),r1
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(44)).unwrap();
    // 0x8c14cffa: mov.l r1,@(44,r4)
    mem.write_u32(cpu.r[4].wrapping_add(44), cpu.r[1]).unwrap();
    // 0x8c14cffc: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cffe: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14d002: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
}
