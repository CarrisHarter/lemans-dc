//! 0x8c14c680 — copy 172-byte struct r5 -> r4

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14c680: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[14]).unwrap();
    // 0x8c14c682: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c14c684: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[12]).unwrap();
    // 0x8c14c686: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[11]).unwrap();
    // 0x8c14c688: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[10]).unwrap();
    // 0x8c14c68a: mov.l r9,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[9]).unwrap();
    // 0x8c14c68c: mov.l r8,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[8]).unwrap();
    // 0x8c14c68e: add #-8,r15
    cpu.r[15] = (cpu.r[15] as i32).wrapping_add(-8) as u32;
    // 0x8c14c690: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14c692: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c694: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c696: fmov fr0,@r4
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap();
    // 0x8c14c698: add #4,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(4) as u32;
    // 0x8c14c69a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c69c: add #4,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(4) as u32;
    // 0x8c14c69e: mov r5,r6
    cpu.r[6] = cpu.r[5];
    // 0x8c14c6a0: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c6a2: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c6a4: add #8,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(8) as u32;
    // 0x8c14c6a6: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c6a8: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14c6aa: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c6ac: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14c6ae: add #8,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(8) as u32;
    // 0x8c14c6b0: fmov fr0,@r0
    mem.write_f32(cpu.r[0], cpu.fr[0]).unwrap();
    // 0x8c14c6b2: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c6b4: mov.b @(12,r5),r0
    cpu.r[0] = mem.read_u8(cpu.r[5].wrapping_add(12)).unwrap() as u32;
    // 0x8c14c6b6: mov r4,r8
    cpu.r[8] = cpu.r[4];
    // 0x8c14c6b8: mov r5,r10
    cpu.r[10] = cpu.r[5];
    // 0x8c14c6ba: mov.b r0,@(12,r4)
    mem.write_u8(cpu.r[4].wrapping_add(12), (cpu.r[0] & 0xff) as u8).unwrap();
    // 0x8c14c6bc: mov r4,r9
    cpu.r[9] = cpu.r[4];
    // 0x8c14c6be: mov.b @(13,r5),r0
    cpu.r[0] = mem.read_u8(cpu.r[5].wrapping_add(13)).unwrap() as u32;
    // 0x8c14c6c0: mov r5,r12
    cpu.r[12] = cpu.r[5];
    // 0x8c14c6c2: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c14c6c4: mov.b r0,@(13,r4)
    mem.write_u8(cpu.r[4].wrapping_add(13), (cpu.r[0] & 0xff) as u8).unwrap();
    // 0x8c14c6c6: add #24,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(24) as u32;
    // 0x8c14c6c8: mov.l @(16,r5),r0
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap();
    // 0x8c14c6ca: add #24,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(24) as u32;
    // 0x8c14c6cc: add #28,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(28) as u32;
    // 0x8c14c6ce: mov.l r0,@(16,r4)
    mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[0]).unwrap();
    // 0x8c14c6d0: add #28,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(28) as u32;
    // 0x8c14c6d2: mov.l @(20,r5),r13
    cpu.r[13] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap();
    // 0x8c14c6d4: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c6d6: add #32,r8
    cpu.r[8] = (cpu.r[8] as i32).wrapping_add(32) as u32;
    // 0x8c14c6d8: mov.l r13,@r15
    mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c14c6da: add #36,r10
    cpu.r[10] = (cpu.r[10] as i32).wrapping_add(36) as u32;
    // 0x8c14c6dc: mov.l @r15,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    // 0x8c14c6de: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c14c6e0: add #36,r9
    cpu.r[9] = (cpu.r[9] as i32).wrapping_add(36) as u32;
    // 0x8c14c6e2: mov.l r14,@(20,r4)
    mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[14]).unwrap();
    // 0x8c14c6e4: add #40,r12
    cpu.r[12] = (cpu.r[12] as i32).wrapping_add(40) as u32;
    // 0x8c14c6e6: fmov @r6,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    // 0x8c14c6e8: add #40,r11
    cpu.r[11] = (cpu.r[11] as i32).wrapping_add(40) as u32;
    // 0x8c14c6ea: add #44,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(44) as u32;
    // 0x8c14c6ec: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14c6ee: mov r5,r6
    cpu.r[6] = cpu.r[5];
    // 0x8c14c6f0: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c6f2: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c14c6f4: add #44,r13
    cpu.r[13] = (cpu.r[13] as i32).wrapping_add(44) as u32;
    // 0x8c14c6f6: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c6f8: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c6fa: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c6fc: add #32,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(32) as u32;
    // 0x8c14c6fe: mov.l r3,@(4,r15)
    mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[3]).unwrap();
    // 0x8c14c700: add #48,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(48) as u32;
    // 0x8c14c702: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14c704: add #48,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(48) as u32;
    // 0x8c14c706: add #52,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(52) as u32;
    // 0x8c14c708: fmov fr0,@r8
    mem.write_f32(cpu.r[8], cpu.fr[0]).unwrap();
    // 0x8c14c70a: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c70c: fmov @r10,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[10]).unwrap();
    // 0x8c14c70e: mov r5,r8
    cpu.r[8] = cpu.r[5];
    // 0x8c14c710: add #52,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(52) as u32;
    // 0x8c14c712: fmov fr0,@r9
    mem.write_f32(cpu.r[9], cpu.fr[0]).unwrap();
    // 0x8c14c714: add #56,r8
    cpu.r[8] = (cpu.r[8] as i32).wrapping_add(56) as u32;
    // 0x8c14c716: fmov @r12,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[12]).unwrap();
    // 0x8c14c718: mov r4,r9
    cpu.r[9] = cpu.r[4];
    // 0x8c14c71a: add #56,r9
    cpu.r[9] = (cpu.r[9] as i32).wrapping_add(56) as u32;
    // 0x8c14c71c: fmov fr0,@r11
    mem.write_f32(cpu.r[11], cpu.fr[0]).unwrap();
    // 0x8c14c71e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c720: fmov fr0,@r13
    mem.write_f32(cpu.r[13], cpu.fr[0]).unwrap();
    // 0x8c14c722: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c724: fmov @r6,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    // 0x8c14c726: add #64,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(64) as u32;
    // 0x8c14c728: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c72a: mov r5,r6
    cpu.r[6] = cpu.r[5];
    // 0x8c14c72c: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14c72e: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c730: add #60,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(60) as u32;
    // 0x8c14c732: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c14c734: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c736: fmov @r8,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[8]).unwrap();
    // 0x8c14c738: add #60,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(60) as u32;
    // 0x8c14c73a: add #64,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(64) as u32;
    // 0x8c14c73c: fmov fr0,@r9
    mem.write_f32(cpu.r[9], cpu.fr[0]).unwrap();
    // 0x8c14c73e: fmov @r6,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    // 0x8c14c740: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c742: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c744: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c746: add #84,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(84) as u32;
    // 0x8c14c748: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c74a: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c74c: add #68,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(68) as u32;
    // 0x8c14c74e: mov.l @r0,r3
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap();
    // 0x8c14c750: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c752: add #72,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(72) as u32;
    // 0x8c14c754: mov #68,r0
    cpu.r[0] = 68u32;
    // 0x8c14c756: mov.l r3,@(r0,r4)
    mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[3]).unwrap();
    // 0x8c14c758: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c75a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c75c: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c75e: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c760: add #72,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(72) as u32;
    // 0x8c14c762: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c764: add #76,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(76) as u32;
    // 0x8c14c766: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c768: add #76,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(76) as u32;
    // 0x8c14c76a: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c76c: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c76e: add #80,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(80) as u32;
    // 0x8c14c770: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c772: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c774: add #80,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(80) as u32;
    // 0x8c14c776: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c778: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c77a: add #84,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(84) as u32;
    // 0x8c14c77c: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c77e: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c780: add #88,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(88) as u32;
    // 0x8c14c782: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c784: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c786: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c788: add #88,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(88) as u32;
    // 0x8c14c78a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c78c: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c78e: add #92,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(92) as u32;
    // 0x8c14c790: mov.b @r0,r3
    cpu.r[3] = mem.read_u8(cpu.r[0]).unwrap() as u32;
    // 0x8c14c792: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c794: add #93,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(93) as u32;
    // 0x8c14c796: mov #92,r0
    cpu.r[0] = 92u32;
    // 0x8c14c798: extu.b r3,r3
    cpu.r[3] = cpu.r[3] & 0xff;
    // 0x8c14c79a: mov.b r3,@(r0,r4)
    mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), (cpu.r[3] & 0xff) as u8).unwrap();
    // 0x8c14c79c: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c79e: mov.b @r1,r7
    cpu.r[7] = mem.read_u8(cpu.r[1]).unwrap() as u32;
    // 0x8c14c7a0: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c7a2: mov #93,r0
    cpu.r[0] = 93u32;
    // 0x8c14c7a4: extu.b r7,r7
    cpu.r[7] = cpu.r[7] & 0xff;
    // 0x8c14c7a6: mov.b r7,@(r0,r4)
    mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), (cpu.r[7] & 0xff) as u8).unwrap();
    // 0x8c14c7a8: add #96,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(96) as u32;
    // 0x8c14c7aa: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c7ac: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14c7ae: add #96,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(96) as u32;
    // 0x8c14c7b0: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14c7b2: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c7b4: add #100,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(100) as u32;
    // 0x8c14c7b6: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c7b8: add #100,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(100) as u32;
    // 0x8c14c7ba: fmov fr0,@r0
    mem.write_f32(cpu.r[0], cpu.fr[0]).unwrap();
    // 0x8c14c7bc: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c7be: add #104,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(104) as u32;
    // 0x8c14c7c0: mov.b @r0,r6
    cpu.r[6] = mem.read_u8(cpu.r[0]).unwrap() as u32;
    // 0x8c14c7c2: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c7c4: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c7c6: extu.b r6,r6
    cpu.r[6] = cpu.r[6] & 0xff;
    // 0x8c14c7c8: mov #104,r0
    cpu.r[0] = 104u32;
    // 0x8c14c7ca: mov.b r6,@(r0,r4)
    mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), (cpu.r[6] & 0xff) as u8).unwrap();
    // 0x8c14c7cc: add #105,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(105) as u32;
    // 0x8c14c7ce: mov.b @r3,r6
    cpu.r[6] = mem.read_u8(cpu.r[3]).unwrap() as u32;
    // 0x8c14c7d0: mov #105,r0
    cpu.r[0] = 105u32;
    // 0x8c14c7d2: add #108,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(108) as u32;
    // 0x8c14c7d4: extu.b r6,r6
    cpu.r[6] = cpu.r[6] & 0xff;
    // 0x8c14c7d6: mov.b r6,@(r0,r4)
    mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), (cpu.r[6] & 0xff) as u8).unwrap();
    // 0x8c14c7d8: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c7da: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c7dc: add #108,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(108) as u32;
    // 0x8c14c7de: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c7e0: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c7e2: add #112,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(112) as u32;
    // 0x8c14c7e4: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c7e6: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c7e8: add #112,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(112) as u32;
    // 0x8c14c7ea: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c7ec: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c7ee: add #116,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(116) as u32;
    // 0x8c14c7f0: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c7f2: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c7f4: add #116,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(116) as u32;
    // 0x8c14c7f6: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c14c7f8: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c7fa: add #120,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(120) as u32;
    // 0x8c14c7fc: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c7fe: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c800: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c14c802: add #120,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(120) as u32;
    // 0x8c14c804: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c806: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c808: add #124,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(124) as u32;
    // 0x8c14c80a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c80c: add #124,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(124) as u32;
    // 0x8c14c80e: mov.l 0x8c14c8bc,r1
    cpu.r[1] = mem.read_u32(0x8c14c8bc).unwrap();
    // 0x8c14c810: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c812: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c814: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c816: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c818: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14c81a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c81c: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c81e: mov.l 0x8c14c8c0,r1
    cpu.r[1] = mem.read_u32(0x8c14c8c0).unwrap();
    // 0x8c14c820: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c822: mov r4,r8
    cpu.r[8] = cpu.r[4];
    // 0x8c14c824: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c826: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c828: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14c82a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c82c: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c82e: mov.l 0x8c14c8c4,r1
    cpu.r[1] = mem.read_u32(0x8c14c8c4).unwrap();
    // 0x8c14c830: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c832: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c834: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c836: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14c838: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c83a: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c83c: mov.l 0x8c14c8c8,r1
    cpu.r[1] = mem.read_u32(0x8c14c8c8).unwrap();
    // 0x8c14c83e: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c840: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c842: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c844: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14c846: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c848: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c84a: mov.l 0x8c14c8cc,r1
    cpu.r[1] = mem.read_u32(0x8c14c8cc).unwrap();
    // 0x8c14c84c: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c84e: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c850: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c852: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14c854: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c856: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c858: mov.l 0x8c14c8d0,r1
    cpu.r[1] = mem.read_u32(0x8c14c8d0).unwrap();
    // 0x8c14c85a: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c85c: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c85e: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c860: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14c862: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c864: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c866: mov.l 0x8c14c8d4,r1
    cpu.r[1] = mem.read_u32(0x8c14c8d4).unwrap();
    // 0x8c14c868: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c86a: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c86c: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c86e: add r1,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[1]);
    // 0x8c14c870: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c872: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 0x8c14c874: mov.l 0x8c14c8d8,r1
    cpu.r[1] = mem.read_u32(0x8c14c8d8).unwrap();
    // 0x8c14c876: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c878: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c87a: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c87c: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c14c87e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c880: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14c882: mov.l 0x8c14c8dc,r1
    cpu.r[1] = mem.read_u32(0x8c14c8dc).unwrap();
    // 0x8c14c884: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c14c886: fmov fr0,@r3
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap();
    // 0x8c14c888: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14c88a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14c88c: add r1,r8
    cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[1]);
    // 0x8c14c88e: mov.l 0x8c14c8e0,r7
    cpu.r[7] = mem.read_u32(0x8c14c8e0).unwrap();
    // 0x8c14c890: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 0x8c14c892: mov.l 0x8c14c8e4,r0
    cpu.r[0] = mem.read_u32(0x8c14c8e4).unwrap();
    // 0x8c14c894: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c14c896: add r7,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[7]);
    // 0x8c14c898: fmov fr0,@r8
    mem.write_f32(cpu.r[8], cpu.fr[0]).unwrap();
    // 0x8c14c89a: add r7,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[7]);
    // 0x8c14c89c: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c14c89e: add r0,r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]);
    // 0x8c14c8a0: fmov fr0,@r6
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap();
    // 0x8c14c8a2: add r0,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    // 0x8c14c8a4: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c14c8a6: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c14c8a8: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c14c8aa: add #8,r15
    cpu.r[15] = (cpu.r[15] as i32).wrapping_add(8) as u32;
    // 0x8c14c8ac: mov.l @r15+,r8
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14c8ae: mov.l @r15+,r9
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14c8b0: mov.l @r15+,r10
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14c8b2: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14c8b4: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14c8b6: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c14c8ba: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
}
