//! 0x8c14c8e8 — copy 216-byte struct r5 -> r4

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14c8e8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[14]).unwrap();
    // 0x8c14c8ea: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c14c8ec: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[12]).unwrap();
    // 0x8c14c8ee: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[11]).unwrap();
    // 0x8c14c8f0: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[10]).unwrap();
    // 0x8c14c8f2: mov.l r9,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[9]).unwrap();
    // 0x8c14c8f4: mov.l r8,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[8]).unwrap();
    // 0x8c14c8f6: add #-4,r15
    cpu.r[15] = (cpu.r[15] as i32).wrapping_add(-4) as u32;
    // 0x8c14c8f8: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14c8fa: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c8fc: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c8fe: fmov fr0,@r4
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap();
    // 0x8c14c900: add #4,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(4) as u32;
    // 0x8c14c902: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c904: add #4,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(4) as u32;
    // 0x8c14c906: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c908: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c90a: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c90c: add #8,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(8) as u32;
    // 0x8c14c90e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c910: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c912: add #8,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(8) as u32;
    // 0x8c14c914: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c916: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c918: add #12,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(12) as u32;
    // 0x8c14c91a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c91c: add #12,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(12) as u32;
    // 0x8c14c91e: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c920: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c922: add #16,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(16) as u32;
    // 0x8c14c924: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c926: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c928: add #16,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(16) as u32;
    // 0x8c14c92a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c92c: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c92e: add #20,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(20) as u32;
    // 0x8c14c930: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c932: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c934: add #20,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(20) as u32;
    // 0x8c14c936: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c938: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c93a: add #24,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(24) as u32;
    // 0x8c14c93c: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c93e: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c940: add #24,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(24) as u32;
    // 0x8c14c942: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c944: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c946: add #28,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(28) as u32;
    // 0x8c14c948: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c94a: mov r4,r8
    cpu.r[8] = cpu.r[4];
    // 0x8c14c94c: add #28,r8
    cpu.r[8] = (cpu.r[8] as i32).wrapping_add(28) as u32;
    // 0x8c14c94e: mov r5,r6
    cpu.r[6] = cpu.r[5];
    // 0x8c14c950: fmov fr0,@r8
    mem.write_f32(cpu.r[8], cpu.fr[0]).unwrap();
    // 0x8c14c952: add #32,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(32) as u32;
    // 0x8c14c954: fmov @r6,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    // 0x8c14c956: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14c958: add #32,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(32) as u32;
    // 0x8c14c95a: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c95c: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14c95e: add #36,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(36) as u32;
    // 0x8c14c960: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c962: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c964: add #36,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(36) as u32;
    // 0x8c14c966: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c968: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c96a: add #40,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(40) as u32;
    // 0x8c14c96c: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c96e: mov r4,r9
    cpu.r[9] = cpu.r[4];
    // 0x8c14c970: add #40,r9
    cpu.r[9] = (cpu.r[9] as i32).wrapping_add(40) as u32;
    // 0x8c14c972: mov r5,r8
    cpu.r[8] = cpu.r[5];
    // 0x8c14c974: fmov fr0,@r9
    mem.write_f32(cpu.r[9], cpu.fr[0]).unwrap();
    // 0x8c14c976: add #44,r8
    cpu.r[8] = (cpu.r[8] as i32).wrapping_add(44) as u32;
    // 0x8c14c978: fmov @r8,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[8]).unwrap();
    // 0x8c14c97a: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c14c97c: add #44,r11
    cpu.r[11] = (cpu.r[11] as i32).wrapping_add(44) as u32;
    // 0x8c14c97e: mov r5,r6
    cpu.r[6] = cpu.r[5];
    // 0x8c14c980: fmov fr0,@r11
    mem.write_f32(cpu.r[11], cpu.fr[0]).unwrap();
    // 0x8c14c982: add #48,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(48) as u32;
    // 0x8c14c984: fmov @r6,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    // 0x8c14c986: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14c988: add #48,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(48) as u32;
    // 0x8c14c98a: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c98c: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14c98e: add #52,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(52) as u32;
    // 0x8c14c990: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c992: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c994: add #52,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(52) as u32;
    // 0x8c14c996: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c998: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c99a: add #56,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(56) as u32;
    // 0x8c14c99c: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c99e: mov r4,r10
    cpu.r[10] = cpu.r[4];
    // 0x8c14c9a0: add #56,r10
    cpu.r[10] = (cpu.r[10] as i32).wrapping_add(56) as u32;
    // 0x8c14c9a2: mov r5,r9
    cpu.r[9] = cpu.r[5];
    // 0x8c14c9a4: fmov fr0,@r10
    mem.write_f32(cpu.r[10], cpu.fr[0]).unwrap();
    // 0x8c14c9a6: add #60,r9
    cpu.r[9] = (cpu.r[9] as i32).wrapping_add(60) as u32;
    // 0x8c14c9a8: fmov @r9,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[9]).unwrap();
    // 0x8c14c9aa: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 0x8c14c9ac: add #60,r12
    cpu.r[12] = (cpu.r[12] as i32).wrapping_add(60) as u32;
    // 0x8c14c9ae: mov r5,r11
    cpu.r[11] = cpu.r[5];
    // 0x8c14c9b0: fmov fr0,@r12
    mem.write_f32(cpu.r[12], cpu.fr[0]).unwrap();
    // 0x8c14c9b2: add #64,r11
    cpu.r[11] = (cpu.r[11] as i32).wrapping_add(64) as u32;
    // 0x8c14c9b4: fmov @r11,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[11]).unwrap();
    // 0x8c14c9b6: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c14c9b8: add #64,r13
    cpu.r[13] = (cpu.r[13] as i32).wrapping_add(64) as u32;
    // 0x8c14c9ba: mov r5,r8
    cpu.r[8] = cpu.r[5];
    // 0x8c14c9bc: fmov fr0,@r13
    mem.write_f32(cpu.r[13], cpu.fr[0]).unwrap();
    // 0x8c14c9be: add #68,r8
    cpu.r[8] = (cpu.r[8] as i32).wrapping_add(68) as u32;
    // 0x8c14c9c0: fmov @r8,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[8]).unwrap();
    // 0x8c14c9c2: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14c9c4: add #68,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(68) as u32;
    // 0x8c14c9c6: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c9c8: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14c9ca: add #72,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(72) as u32;
    // 0x8c14c9cc: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14c9ce: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c9d0: add #72,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(72) as u32;
    // 0x8c14c9d2: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c9d4: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c9d6: add #76,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(76) as u32;
    // 0x8c14c9d8: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c9da: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14c9dc: add #76,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(76) as u32;
    // 0x8c14c9de: mov r5,r10
    cpu.r[10] = cpu.r[5];
    // 0x8c14c9e0: mov r4,r9
    cpu.r[9] = cpu.r[4];
    // 0x8c14c9e2: mov r5,r12
    cpu.r[12] = cpu.r[5];
    // 0x8c14c9e4: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c14c9e6: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c14c9e8: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c14c9ea: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14c9ec: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c9ee: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c9f0: fmov fr0,@r0
    mem.write_f32(cpu.r[0], cpu.fr[0]).unwrap();
    // 0x8c14c9f2: add #80,r10
    cpu.r[10] = (cpu.r[10] as i32).wrapping_add(80) as u32;
    // 0x8c14c9f4: mov r5,r8
    cpu.r[8] = cpu.r[5];
    // 0x8c14c9f6: fmov @r10,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[10]).unwrap();
    // 0x8c14c9f8: add #80,r9
    cpu.r[9] = (cpu.r[9] as i32).wrapping_add(80) as u32;
    // 0x8c14c9fa: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c9fc: fmov fr0,@r9
    mem.write_f32(cpu.r[9], cpu.fr[0]).unwrap();
    // 0x8c14c9fe: add #84,r12
    cpu.r[12] = (cpu.r[12] as i32).wrapping_add(84) as u32;
    // 0x8c14ca00: mov r5,r9
    cpu.r[9] = cpu.r[5];
    // 0x8c14ca02: fmov @r12,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[12]).unwrap();
    // 0x8c14ca04: add #84,r11
    cpu.r[11] = (cpu.r[11] as i32).wrapping_add(84) as u32;
    // 0x8c14ca06: add #92,r9
    cpu.r[9] = (cpu.r[9] as i32).wrapping_add(92) as u32;
    // 0x8c14ca08: fmov fr0,@r11
    mem.write_f32(cpu.r[11], cpu.fr[0]).unwrap();
    // 0x8c14ca0a: add #88,r14
    cpu.r[14] = (cpu.r[14] as i32).wrapping_add(88) as u32;
    // 0x8c14ca0c: mov.l r9,@r15
    mem.write_u32(cpu.r[15], cpu.r[9]).unwrap();
    // 0x8c14ca0e: add #88,r13
    cpu.r[13] = (cpu.r[13] as i32).wrapping_add(88) as u32;
    // 0x8c14ca10: fmov @r14,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap();
    // 0x8c14ca12: add #92,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(92) as u32;
    // 0x8c14ca14: add #96,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(96) as u32;
    // 0x8c14ca16: fmov fr0,@r13
    mem.write_f32(cpu.r[13], cpu.fr[0]).unwrap();
    // 0x8c14ca18: add #96,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(96) as u32;
    // 0x8c14ca1a: fmov @r9,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[9]).unwrap();
    // 0x8c14ca1c: add #100,r8
    cpu.r[8] = (cpu.r[8] as i32).wrapping_add(100) as u32;
    // 0x8c14ca1e: add #100,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(100) as u32;
    // 0x8c14ca20: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14ca22: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14ca24: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14ca26: fmov @r8,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[8]).unwrap();
    // 0x8c14ca28: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14ca2a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca2c: add #104,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(104) as u32;
    // 0x8c14ca2e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca30: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14ca32: add #104,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(104) as u32;
    // 0x8c14ca34: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca36: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14ca38: add #108,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(108) as u32;
    // 0x8c14ca3a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca3c: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14ca3e: add #108,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(108) as u32;
    // 0x8c14ca40: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14ca42: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14ca44: add #112,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(112) as u32;
    // 0x8c14ca46: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14ca48: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14ca4a: add #112,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(112) as u32;
    // 0x8c14ca4c: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca4e: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14ca50: add #116,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(116) as u32;
    // 0x8c14ca52: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca54: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14ca56: add #116,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(116) as u32;
    // 0x8c14ca58: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14ca5a: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14ca5c: add #120,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(120) as u32;
    // 0x8c14ca5e: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14ca60: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca62: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14ca64: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14ca66: add #120,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(120) as u32;
    // 0x8c14ca68: add #124,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(124) as u32;
    // 0x8c14ca6a: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14ca6c: add #124,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(124) as u32;
    // 0x8c14ca6e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca70: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14ca72: mov.l 0x8c14cbc8,r1
    cpu.r[1] = mem.read_u32(0x8c14cbc8).unwrap();
    // 0x8c14ca74: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca76: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14ca78: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14ca7a: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14ca7c: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca7e: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14ca80: mov.l 0x8c14cbcc,r1
    cpu.r[1] = mem.read_u32(0x8c14cbcc).unwrap();
    // 0x8c14ca82: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca84: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14ca86: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14ca88: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14ca8a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca8c: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14ca8e: mov.l 0x8c14cbd0,r1
    cpu.r[1] = mem.read_u32(0x8c14cbd0).unwrap();
    // 0x8c14ca90: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14ca92: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14ca94: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14ca96: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14ca98: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14ca9a: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14ca9c: mov.l 0x8c14cbd4,r1
    cpu.r[1] = mem.read_u32(0x8c14cbd4).unwrap();
    // 0x8c14ca9e: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14caa0: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14caa2: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14caa4: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14caa6: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14caa8: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14caaa: mov.l 0x8c14cbd8,r1
    cpu.r[1] = mem.read_u32(0x8c14cbd8).unwrap();
    // 0x8c14caac: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14caae: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cab0: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cab2: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cab4: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cab6: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cab8: mov.l 0x8c14cbdc,r1
    cpu.r[1] = mem.read_u32(0x8c14cbdc).unwrap();
    // 0x8c14caba: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cabc: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cabe: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cac0: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cac2: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cac4: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cac6: mov.l 0x8c14cbe0,r1
    cpu.r[1] = mem.read_u32(0x8c14cbe0).unwrap();
    // 0x8c14cac8: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14caca: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cacc: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cace: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cad0: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cad2: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cad4: mov.l 0x8c14cbe4,r1
    cpu.r[1] = mem.read_u32(0x8c14cbe4).unwrap();
    // 0x8c14cad6: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cad8: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cada: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cadc: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cade: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cae0: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cae2: mov.l 0x8c14cbe8,r1
    cpu.r[1] = mem.read_u32(0x8c14cbe8).unwrap();
    // 0x8c14cae4: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cae6: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cae8: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14caea: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14caec: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14caee: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14caf0: mov.l 0x8c14cbec,r1
    cpu.r[1] = mem.read_u32(0x8c14cbec).unwrap();
    // 0x8c14caf2: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14caf4: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14caf6: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14caf8: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cafa: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cafc: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cafe: mov.l 0x8c14cbf0,r1
    cpu.r[1] = mem.read_u32(0x8c14cbf0).unwrap();
    // 0x8c14cb00: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb02: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cb04: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb06: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cb08: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb0a: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cb0c: mov.l 0x8c14cbf4,r1
    cpu.r[1] = mem.read_u32(0x8c14cbf4).unwrap();
    // 0x8c14cb0e: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb10: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cb12: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb14: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb16: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cb18: mov.l 0x8c14cbf8,r3
    cpu.r[3] = mem.read_u32(0x8c14cbf8).unwrap();
    // 0x8c14cb1a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb1c: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14cb1e: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cb20: add r3,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    // 0x8c14cb22: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb24: add r3,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    // 0x8c14cb26: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14cb28: mov.l 0x8c14cbfc,r1
    cpu.r[1] = mem.read_u32(0x8c14cbfc).unwrap();
    // 0x8c14cb2a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb2c: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cb2e: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb30: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cb32: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb34: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cb36: mov.l 0x8c14cc00,r1
    cpu.r[1] = mem.read_u32(0x8c14cc00).unwrap();
    // 0x8c14cb38: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb3a: mov r4,r8
    cpu.r[8] = cpu.r[4];
    // 0x8c14cb3c: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cb3e: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb40: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cb42: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb44: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cb46: mov.l 0x8c14cc04,r1
    cpu.r[1] = mem.read_u32(0x8c14cc04).unwrap();
    // 0x8c14cb48: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb4a: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cb4c: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb4e: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cb50: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb52: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cb54: mov.l 0x8c14cc08,r1
    cpu.r[1] = mem.read_u32(0x8c14cc08).unwrap();
    // 0x8c14cb56: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb58: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cb5a: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb5c: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cb5e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb60: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cb62: mov.l 0x8c14cc0c,r1
    cpu.r[1] = mem.read_u32(0x8c14cc0c).unwrap();
    // 0x8c14cb64: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb66: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cb68: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb6a: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cb6c: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb6e: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cb70: mov.l 0x8c14cc10,r1
    cpu.r[1] = mem.read_u32(0x8c14cc10).unwrap();
    // 0x8c14cb72: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb74: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cb76: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb78: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14cb7a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb7c: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14cb7e: mov.l 0x8c14cc14,r1
    cpu.r[1] = mem.read_u32(0x8c14cc14).unwrap();
    // 0x8c14cb80: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb82: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cb84: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb86: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14cb88: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb8a: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14cb8c: mov.l 0x8c14cc18,r1
    cpu.r[1] = mem.read_u32(0x8c14cc18).unwrap();
    // 0x8c14cb8e: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14cb90: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14cb92: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14cb94: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14cb96: add r1,r8
    cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[1]);
    // 0x8c14cb98: mov.l 0x8c14cc1c,r7
    cpu.r[7] = mem.read_u32(0x8c14cc1c).unwrap();
    // 0x8c14cb9a: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14cb9c: mov.l 0x8c14cc20,r0
    cpu.r[0] = mem.read_u32(0x8c14cc20).unwrap();
    // 0x8c14cb9e: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14cba0: add r7,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[7]);
    // 0x8c14cba2: fmov fr0,@r8
    mem.write_f32(cpu.r[8], cpu.fr[0]).unwrap();
    // 0x8c14cba4: add r7,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[7]);
    // 0x8c14cba6: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14cba8: add r0,r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]);
    // 0x8c14cbaa: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14cbac: add r0,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    // 0x8c14cbae: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14cbb0: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14cbb2: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14cbb4: add #4,r15
    cpu.r[15] = (cpu.r[15] as i32).wrapping_add(4) as u32;
    // 0x8c14cbb6: mov.l @r15+,r8
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cbb8: mov.l @r15+,r9
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cbba: mov.l @r15+,r10
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cbbc: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cbbe: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cbc0: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14cbc4: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
}
