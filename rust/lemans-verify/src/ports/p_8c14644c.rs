//! 0x8c14644c — matrix init: writes 0, 1, -1 and 7 fmul products into the r4 struct

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14644c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[14]).unwrap();
    // 0x8c14644e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c146450: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[12]).unwrap();
    // 0x8c146452: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); mem.write_u32(cpu.r[15], cpu.r[11]).unwrap();
    // 0x8c146454: mov #0,r0
    cpu.r[0] = 0u32;
    // 0x8c146456: mov.l r0,@r4
    mem.write_u32(cpu.r[4], cpu.r[0]).unwrap();
    // 0x8c146458: mov r4,r5
    cpu.r[5] = cpu.r[4];
    // 0x8c14645a: mov.l 0x8c1464f4,r3
    cpu.r[3] = mem.read_u32(0x8c1464f4).unwrap();
    // 0x8c14645c: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14645e: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c146460: mov.l @r3,r0
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap();
    // 0x8c146462: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 0x8c146464: mov.l 0x8c1464f8,r1
    cpu.r[1] = mem.read_u32(0x8c1464f8).unwrap();
    // 0x8c146466: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c146468: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c14646a: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14646c: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14646e: add #4,r5
    cpu.r[5] = (cpu.r[5] as i32).wrapping_add(4) as u32;
    // 0x8c146470: add #8,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(8) as u32;
    // 0x8c146472: fmov fr0,@r5
    mem.write_f32(cpu.r[5], cpu.fr[0]).unwrap();
    // 0x8c146474: mova 0x8c1464fc,r0
    cpu.r[0] = 0x8c1464fc;
    // 0x8c146476: mov.l @r3,r5
    cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap();
    // 0x8c146478: add #12,r11
    cpu.r[11] = (cpu.r[11] as i32).wrapping_add(12) as u32;
    // 0x8c14647a: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c14647c: add r1,r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
    // 0x8c14647e: fmov @r5,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c146480: mova 0x8c146500,r0
    cpu.r[0] = 0x8c146500;
    // 0x8c146482: add #16,r7
    cpu.r[7] = (cpu.r[7] as i32).wrapping_add(16) as u32;
    // 0x8c146484: fmul fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    // 0x8c146486: fmov fr1,@r6
    mem.write_f32(cpu.r[6], cpu.fr[1]).unwrap();
    // 0x8c146488: mov #-1,r12
    cpu.r[12] = (-1i32) as u32;
    // 0x8c14648a: mov.l @r3,r5
    cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap();
    // 0x8c14648c: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 0x8c14648e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c146490: add r1,r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
    // 0x8c146492: fmov @r5,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c146494: mova 0x8c146504,r0
    cpu.r[0] = 0x8c146504;
    // 0x8c146496: add #32,r13
    cpu.r[13] = (cpu.r[13] as i32).wrapping_add(32) as u32;
    // 0x8c146498: fmul fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    // 0x8c14649a: fmov fr1,@r11
    mem.write_f32(cpu.r[11], cpu.fr[1]).unwrap();
    // 0x8c14649c: mov r4,r5
    cpu.r[5] = cpu.r[4];
    // 0x8c14649e: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c1464a0: mov #1,r11
    cpu.r[11] = 1u32;
    // 0x8c1464a2: add #36,r14
    cpu.r[14] = (cpu.r[14] as i32).wrapping_add(36) as u32;
    // 0x8c1464a4: fmov fr0,@r7
    mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap();
    // 0x8c1464a6: mova 0x8c146508,r0
    cpu.r[0] = 0x8c146508;
    // 0x8c1464a8: mov.l r11,@(20,r4)
    mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[11]).unwrap();
    // 0x8c1464aa: add #40,r6
    cpu.r[6] = (cpu.r[6] as i32).wrapping_add(40) as u32;
    // 0x8c1464ac: mov.l r11,@(24,r4)
    mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[11]).unwrap();
    // 0x8c1464ae: add #44,r5
    cpu.r[5] = (cpu.r[5] as i32).wrapping_add(44) as u32;
    // 0x8c1464b0: mov.l r12,@(28,r4)
    mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[12]).unwrap();
    // 0x8c1464b2: mov.l @r3,r7
    cpu.r[7] = mem.read_u32(cpu.r[3]).unwrap();
    // 0x8c1464b4: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c1464b6: add r1,r7
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]);
    // 0x8c1464b8: fmov @r7,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap();
    // 0x8c1464ba: mova 0x8c14650c,r0
    cpu.r[0] = 0x8c14650c;
    // 0x8c1464bc: fmul fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    // 0x8c1464be: fmov fr1,@r13
    mem.write_f32(cpu.r[13], cpu.fr[1]).unwrap();
    // 0x8c1464c0: mov.l @r3,r7
    cpu.r[7] = mem.read_u32(cpu.r[3]).unwrap();
    // 0x8c1464c2: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c1464c4: add r1,r7
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]);
    // 0x8c1464c6: fmov @r7,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap();
    // 0x8c1464c8: mova 0x8c146510,r0
    cpu.r[0] = 0x8c146510;
    // 0x8c1464ca: fmul fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    // 0x8c1464cc: fmov fr1,@r14
    mem.write_f32(cpu.r[14], cpu.fr[1]).unwrap();
    // 0x8c1464ce: mov.l @r3,r3
    cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap();
    // 0x8c1464d0: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c1464d2: add r1,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    // 0x8c1464d4: fmov @r3,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c1464d6: mova 0x8c146514,r0
    cpu.r[0] = 0x8c146514;
    // 0x8c1464d8: fmul fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    // 0x8c1464da: fmov fr1,@r6
    mem.write_f32(cpu.r[6], cpu.fr[1]).unwrap();
    // 0x8c1464dc: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c1464de: fmov fr0,@r5
    mem.write_f32(cpu.r[5], cpu.fr[0]).unwrap();
    // 0x8c1464e0: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 0x8c1464e2: mov.l r11,@(48,r4)
    mem.write_u32(cpu.r[4].wrapping_add(48), cpu.r[11]).unwrap();
    // 0x8c1464e4: mov.l r11,@(52,r4)
    mem.write_u32(cpu.r[4].wrapping_add(52), cpu.r[11]).unwrap();
    // 0x8c1464e6: mov.l r12,@(56,r4)
    mem.write_u32(cpu.r[4].wrapping_add(56), cpu.r[12]).unwrap();
    // 0x8c1464e8: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1464ea: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1464ec: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1464f0: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4);
}
