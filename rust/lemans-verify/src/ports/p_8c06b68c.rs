//! 0x8c06b68c — 291-instruction leaf, single straight-line path (no branches,
//! no loops): copies three 16-byte float blocks (r5, r6, r7) plus a stack-arg
//! block into the output struct at r4, scales by literal-pool constants
//! (-3.0 @0x8c06b8e4, 4.0 @0x8c06b8e8, 2.0 @0x8c06b8ec) and stores a second
//! 4x3 matrix at r4+0x50..r4+0x8c. Generated instruction-by-instruction from
//! analysis/export/disassembly.asm.
//!
//! Side effects the decompiled C does not show: final r0 = r4, r1/r3 = loaded
//! words, r2 = entry_r15-32, r5 = entry_r15-44, r6 = r4+64, r7 = r4+80;
//! r8..r14 and r15 restored by epilogue pops; fr0..fr8 clobbered (fr9..fr15
//! untouched); T and all system registers unchanged.

use sh4_oracle::{Cpu, Memory};

fn r32(mem: &Memory, a: u32) -> u32 { mem.read_u32(a).unwrap() }
fn w32(mem: &mut Memory, a: u32, v: u32) { mem.write_u32(a, v).unwrap() }
fn rf(mem: &Memory, a: u32) -> f32 { mem.read_f32(a).unwrap() }
fn wf(mem: &mut Memory, a: u32, v: f32) { mem.write_f32(a, v).unwrap() }

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c06b68c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[14]);
    // 8c06b68e: mov #100,r2
    cpu.r[2] = 0x00000064;
    // 8c06b690: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[13]);
    // 8c06b692: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[12]);
    // 8c06b694: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[11]);
    // 8c06b696: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[10]);
    // 8c06b698: mov.l r9,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[9]);
    // 8c06b69a: mov.l r8,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); w32(mem, cpu.r[15], cpu.r[8]);
    // 8c06b69c: add #-72,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(72);
    // 8c06b69e: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b6a0: mov.l @r2,r0
    cpu.r[0] = r32(mem, cpu.r[2]);
    // 8c06b6a2: fmov @r5,fr0
    cpu.fr[0] = rf(mem, cpu.r[5]);
    // 8c06b6a4: mov r5,r3
    cpu.r[3] = cpu.r[5];
    // 8c06b6a6: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 8c06b6a8: fmov fr0,@r4
    wf(mem, cpu.r[4], cpu.fr[0]);
    // 8c06b6aa: add #4,r3
    cpu.r[3] = cpu.r[3].wrapping_add(0x00000004);
    // 8c06b6ac: fmov @r3,fr0
    cpu.fr[0] = rf(mem, cpu.r[3]);
    // 8c06b6ae: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 8c06b6b0: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b6b2: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 8c06b6b4: add #4,r3
    cpu.r[3] = cpu.r[3].wrapping_add(0x00000004);
    // 8c06b6b6: fmov fr0,@r3
    wf(mem, cpu.r[3], cpu.fr[0]);
    // 8c06b6b8: add #8,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000008);
    // 8c06b6ba: fmov @r1,fr0
    cpu.fr[0] = rf(mem, cpu.r[1]);
    // 8c06b6bc: mov r6,r3
    cpu.r[3] = cpu.r[6];
    // 8c06b6be: mov r4,r10
    cpu.r[10] = cpu.r[4];
    // 8c06b6c0: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 8c06b6c2: add #8,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000008);
    // 8c06b6c4: fmov fr0,@r1
    wf(mem, cpu.r[1], cpu.fr[0]);
    // 8c06b6c6: add #12,r5
    cpu.r[5] = cpu.r[5].wrapping_add(0x0000000c);
    // 8c06b6c8: fmov @r5,fr0
    cpu.fr[0] = rf(mem, cpu.r[5]);
    // 8c06b6ca: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 8c06b6cc: add #12,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x0000000c);
    // 8c06b6ce: fmov fr0,@r1
    wf(mem, cpu.r[1], cpu.fr[0]);
    // 8c06b6d0: mov r4,r5
    cpu.r[5] = cpu.r[4];
    // 8c06b6d2: fmov @r6,fr0
    cpu.fr[0] = rf(mem, cpu.r[6]);
    // 8c06b6d4: mov r6,r1
    cpu.r[1] = cpu.r[6];
    // 8c06b6d6: add #16,r14
    cpu.r[14] = cpu.r[14].wrapping_add(0x00000010);
    // 8c06b6d8: fmov fr0,@r14
    wf(mem, cpu.r[14], cpu.fr[0]);
    // 8c06b6da: add #4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000004);
    // 8c06b6dc: fmov @r1,fr0
    cpu.fr[0] = rf(mem, cpu.r[1]);
    // 8c06b6de: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 8c06b6e0: mov r0,r8
    cpu.r[8] = cpu.r[0];
    // 8c06b6e2: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 8c06b6e4: add #20,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000014);
    // 8c06b6e6: add #36,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000024);
    // 8c06b6e8: mov.l r1,@r2
    w32(mem, cpu.r[2], cpu.r[1]);
    // 8c06b6ea: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 8c06b6ec: fmov fr0,@r1
    wf(mem, cpu.r[1], cpu.fr[0]);
    // 8c06b6ee: add #8,r3
    cpu.r[3] = cpu.r[3].wrapping_add(0x00000008);
    // 8c06b6f0: fmov @r3,fr0
    cpu.fr[0] = rf(mem, cpu.r[3]);
    // 8c06b6f2: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 8c06b6f4: add #24,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000018);
    // 8c06b6f6: mov.l r1,@r15
    w32(mem, cpu.r[15], cpu.r[1]);
    // 8c06b6f8: mov r4,r3
    cpu.r[3] = cpu.r[4];
    // 8c06b6fa: fmov fr0,@r1
    wf(mem, cpu.r[1], cpu.fr[0]);
    // 8c06b6fc: add #12,r6
    cpu.r[6] = cpu.r[6].wrapping_add(0x0000000c);
    // 8c06b6fe: fmov @r6,fr0
    cpu.fr[0] = rf(mem, cpu.r[6]);
    // 8c06b700: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 8c06b702: add #28,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x0000001c);
    // 8c06b704: mov.l r1,@(4,r15)
    w32(mem, cpu.r[15].wrapping_add(4), cpu.r[1]);
    // 8c06b706: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 8c06b708: fmov fr0,@r1
    wf(mem, cpu.r[1], cpu.fr[0]);
    // 8c06b70a: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 8c06b70c: fmov @r7,fr0
    cpu.fr[0] = rf(mem, cpu.r[7]);
    // 8c06b70e: add #32,r6
    cpu.r[6] = cpu.r[6].wrapping_add(0x00000020);
    // 8c06b710: mov r7,r1
    cpu.r[1] = cpu.r[7];
    // 8c06b712: fmov fr0,@r6
    wf(mem, cpu.r[6], cpu.fr[0]);
    // 8c06b714: add #4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000004);
    // 8c06b716: fmov @r1,fr0
    cpu.fr[0] = rf(mem, cpu.r[1]);
    // 8c06b718: add #36,r5
    cpu.r[5] = cpu.r[5].wrapping_add(0x00000024);
    // 8c06b71a: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b71c: mov r7,r1
    cpu.r[1] = cpu.r[7];
    // 8c06b71e: fmov fr0,@r5
    wf(mem, cpu.r[5], cpu.fr[0]);
    // 8c06b720: add #8,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x00000008);
    // 8c06b722: fmov @r1,fr0
    cpu.fr[0] = rf(mem, cpu.r[1]);
    // 8c06b724: add #40,r3
    cpu.r[3] = cpu.r[3].wrapping_add(0x00000028);
    // 8c06b726: add #12,r7
    cpu.r[7] = cpu.r[7].wrapping_add(0x0000000c);
    // 8c06b728: fmov fr0,@r3
    wf(mem, cpu.r[3], cpu.fr[0]);
    // 8c06b72a: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 8c06b72c: fmov @r7,fr0
    cpu.fr[0] = rf(mem, cpu.r[7]);
    // 8c06b72e: add #44,r1
    cpu.r[1] = cpu.r[1].wrapping_add(0x0000002c);
    // 8c06b730: add #48,r10
    cpu.r[10] = cpu.r[10].wrapping_add(0x00000030);
    // 8c06b732: fmov fr0,@r1
    wf(mem, cpu.r[1], cpu.fr[0]);
    // 8c06b734: mov r0,r7
    cpu.r[7] = cpu.r[0];
    // 8c06b736: fmov @r0,fr0
    cpu.fr[0] = rf(mem, cpu.r[0]);
    // 8c06b738: add #4,r8
    cpu.r[8] = cpu.r[8].wrapping_add(0x00000004);
    // 8c06b73a: add #52,r11
    cpu.r[11] = cpu.r[11].wrapping_add(0x00000034);
    // 8c06b73c: fmov fr0,@r10
    wf(mem, cpu.r[10], cpu.fr[0]);
    // 8c06b73e: add #8,r7
    cpu.r[7] = cpu.r[7].wrapping_add(0x00000008);
    // 8c06b740: fmov @r8,fr0
    cpu.fr[0] = rf(mem, cpu.r[8]);
    // 8c06b742: add #56,r12
    cpu.r[12] = cpu.r[12].wrapping_add(0x00000038);
    // 8c06b744: add #12,r0
    cpu.r[0] = cpu.r[0].wrapping_add(0x0000000c);
    // 8c06b746: fmov fr0,@r11
    wf(mem, cpu.r[11], cpu.fr[0]);
    // 8c06b748: mov r4,r8
    cpu.r[8] = cpu.r[4];
    // 8c06b74a: fmov @r7,fr0
    cpu.fr[0] = rf(mem, cpu.r[7]);
    // 8c06b74c: add #60,r13
    cpu.r[13] = cpu.r[13].wrapping_add(0x0000003c);
    // 8c06b74e: add #112,r8
    cpu.r[8] = cpu.r[8].wrapping_add(0x00000070);
    // 8c06b750: fmov fr0,@r12
    wf(mem, cpu.r[12], cpu.fr[0]);
    // 8c06b752: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 8c06b754: fmov @r0,fr0
    cpu.fr[0] = rf(mem, cpu.r[0]);
    // 8c06b756: add #116,r7
    cpu.r[7] = cpu.r[7].wrapping_add(0x00000074);
    // 8c06b758: add #16,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000010);
    // 8c06b75a: fmov fr0,@r13
    wf(mem, cpu.r[13], cpu.fr[0]);
    // 8c06b75c: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 8c06b75e: fmov fr4,@r8
    wf(mem, cpu.r[8], cpu.fr[4]);
    // 8c06b760: add #120,r0
    cpu.r[0] = cpu.r[0].wrapping_add(0x00000078);
    // 8c06b762: fmov fr5,@r7
    wf(mem, cpu.r[7], cpu.fr[5]);
    // 8c06b764: fmov fr6,@r0
    wf(mem, cpu.r[0], cpu.fr[6]);
    // 8c06b766: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 8c06b768: add #124,r7
    cpu.r[7] = cpu.r[7].wrapping_add(0x0000007c);
    // 8c06b76a: mov.l r7,@(8,r15)
    w32(mem, cpu.r[15].wrapping_add(8), cpu.r[7]);
    // 8c06b76c: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 8c06b76e: mov.l 0x8c06b8d4,r8
    cpu.r[8] = r32(mem, 0x8c06b8d4);
    // 8c06b770: fmov fr7,@r7
    wf(mem, cpu.r[7], cpu.fr[7]);
    // 8c06b772: add r8,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[8]);
    // 8c06b774: fmov fr8,@r0
    wf(mem, cpu.r[0], cpu.fr[8]);
    // 8c06b776: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 8c06b778: mov.l 0x8c06b8d8,r0
    cpu.r[0] = r32(mem, 0x8c06b8d8);
    // 8c06b77a: mov r4,r8
    cpu.r[8] = cpu.r[4];
    // 8c06b77c: add #96,r8
    cpu.r[8] = cpu.r[8].wrapping_add(0x00000060);
    // 8c06b77e: add r0,r7
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[0]);
    // 8c06b780: mov.l 0x8c06b8dc,r0
    cpu.r[0] = r32(mem, 0x8c06b8dc);
    // 8c06b782: fmov fr9,@r7
    wf(mem, cpu.r[7], cpu.fr[9]);
    // 8c06b784: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 8c06b786: add r0,r7
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[0]);
    // 8c06b788: mov.l r7,@(12,r15)
    w32(mem, cpu.r[15].wrapping_add(12), cpu.r[7]);
    // 8c06b78a: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 8c06b78c: mov.l 0x8c06b8e0,r9
    cpu.r[9] = r32(mem, 0x8c06b8e0);
    // 8c06b78e: fmov fr10,@r7
    wf(mem, cpu.r[7], cpu.fr[10]);
    // 8c06b790: add r9,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[9]);
    // 8c06b792: fmov fr11,@r0
    wf(mem, cpu.r[0], cpu.fr[11]);
    // 8c06b794: mov.l @r14,r9
    cpu.r[9] = r32(mem, cpu.r[14]);
    // 8c06b796: mova 0x8c06b8e4,r0
    cpu.r[0] = 0x8c06b8e4;
    // 8c06b798: mov.l @(4,r14),r7
    cpu.r[7] = r32(mem, cpu.r[14].wrapping_add(4));
    // 8c06b79a: mov.l r7,@r2
    w32(mem, cpu.r[2], cpu.r[7]);
    // 8c06b79c: mov.l r9,@r8
    w32(mem, cpu.r[8], cpu.r[9]);
    // 8c06b79e: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b7a0: mov.l r7,@(4,r8)
    w32(mem, cpu.r[8].wrapping_add(4), cpu.r[7]);
    // 8c06b7a2: add #20,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000014);
    // 8c06b7a4: mov.l @(8,r14),r9
    cpu.r[9] = r32(mem, cpu.r[14].wrapping_add(8));
    // 8c06b7a6: mov.l @(12,r14),r7
    cpu.r[7] = r32(mem, cpu.r[14].wrapping_add(12));
    // 8c06b7a8: mov.l r7,@r2
    w32(mem, cpu.r[2], cpu.r[7]);
    // 8c06b7aa: mov.l r9,@(8,r8)
    w32(mem, cpu.r[8].wrapping_add(8), cpu.r[9]);
    // 8c06b7ac: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b7ae: mov.l r7,@(12,r8)
    w32(mem, cpu.r[8].wrapping_add(12), cpu.r[7]);
    // 8c06b7b0: add #36,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000024);
    // 8c06b7b2: mov.l @(4,r15),r7
    cpu.r[7] = r32(mem, cpu.r[15].wrapping_add(4));
    // 8c06b7b4: fmov @r0,fr0
    cpu.fr[0] = rf(mem, cpu.r[0]);
    // 8c06b7b6: fmov @r7,fr7
    cpu.fr[7] = rf(mem, cpu.r[7]);
    // 8c06b7b8: mov.l @r15,r7
    cpu.r[7] = r32(mem, cpu.r[15]);
    // 8c06b7ba: fmul fr0,fr7
    cpu.fr[7] = cpu.fr[7] * cpu.fr[0];
    // 8c06b7bc: fmov @r7,fr6
    cpu.fr[6] = rf(mem, cpu.r[7]);
    // 8c06b7be: mov.l @r2,r7
    cpu.r[7] = r32(mem, cpu.r[2]);
    // 8c06b7c0: fmul fr0,fr6
    cpu.fr[6] = cpu.fr[6] * cpu.fr[0];
    // 8c06b7c2: fmov @r7,fr5
    cpu.fr[5] = rf(mem, cpu.r[7]);
    // 8c06b7c4: fmul fr0,fr5
    cpu.fr[5] = cpu.fr[5] * cpu.fr[0];
    // 8c06b7c6: fmov @r14,fr2
    cpu.fr[2] = rf(mem, cpu.r[14]);
    // 8c06b7c8: mova 0x8c06b8e8,r0
    cpu.r[0] = 0x8c06b8e8;
    // 8c06b7ca: fmov @r0,fr3
    cpu.fr[3] = rf(mem, cpu.r[0]);
    // 8c06b7cc: fmul fr0,fr2
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0];
    // 8c06b7ce: fmov @r6,fr1
    cpu.fr[1] = rf(mem, cpu.r[6]);
    // 8c06b7d0: mov #44,r2
    cpu.r[2] = 0x0000002c;
    // 8c06b7d2: fmov @r5,fr4
    cpu.fr[4] = rf(mem, cpu.r[5]);
    // 8c06b7d4: fmul fr3,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3];
    // 8c06b7d6: fmov @r10,fr0
    cpu.fr[0] = rf(mem, cpu.r[10]);
    // 8c06b7d8: fadd fr1,fr2
    cpu.fr[2] = cpu.fr[2] + cpu.fr[1];
    // 8c06b7da: mov #40,r0
    cpu.r[0] = 0x00000028;
    // 8c06b7dc: fsub fr0,fr2
    cpu.fr[2] = cpu.fr[2] - cpu.fr[0];
    // 8c06b7de: fmov fr2,@(r0,r15)
    wf(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[2]);
    // 8c06b7e0: fmul fr3,fr4
    cpu.fr[4] = cpu.fr[4] * cpu.fr[3];
    // 8c06b7e2: fmov @r11,fr0
    cpu.fr[0] = rf(mem, cpu.r[11]);
    // 8c06b7e4: fadd fr4,fr5
    cpu.fr[5] = cpu.fr[5] + cpu.fr[4];
    // 8c06b7e6: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b7e8: fsub fr0,fr5
    cpu.fr[5] = cpu.fr[5] - cpu.fr[0];
    // 8c06b7ea: fmov fr5,@r2
    wf(mem, cpu.r[2], cpu.fr[5]);
    // 8c06b7ec: mov r15,r8
    cpu.r[8] = cpu.r[15];
    // 8c06b7ee: fmov @r3,fr1
    cpu.fr[1] = rf(mem, cpu.r[3]);
    // 8c06b7f0: mov #48,r2
    cpu.r[2] = 0x00000030;
    // 8c06b7f2: fmov @r12,fr0
    cpu.fr[0] = rf(mem, cpu.r[12]);
    // 8c06b7f4: fmul fr3,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3];
    // 8c06b7f6: fadd fr1,fr6
    cpu.fr[6] = cpu.fr[6] + cpu.fr[1];
    // 8c06b7f8: fsub fr0,fr6
    cpu.fr[6] = cpu.fr[6] - cpu.fr[0];
    // 8c06b7fa: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b7fc: fmov fr6,@r2
    wf(mem, cpu.r[2], cpu.fr[6]);
    // 8c06b7fe: mov r4,r7
    cpu.r[7] = cpu.r[4];
    // 8c06b800: fmov @r1,fr1
    cpu.fr[1] = rf(mem, cpu.r[1]);
    // 8c06b802: mov #52,r2
    cpu.r[2] = 0x00000034;
    // 8c06b804: fmov @r13,fr0
    cpu.fr[0] = rf(mem, cpu.r[13]);
    // 8c06b806: fmul fr3,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3];
    // 8c06b808: fadd fr1,fr7
    cpu.fr[7] = cpu.fr[7] + cpu.fr[1];
    // 8c06b80a: fsub fr0,fr7
    cpu.fr[7] = cpu.fr[7] - cpu.fr[0];
    // 8c06b80c: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b80e: fmov fr7,@r2
    wf(mem, cpu.r[2], cpu.fr[7]);
    // 8c06b810: add #40,r8
    cpu.r[8] = cpu.r[8].wrapping_add(0x00000028);
    // 8c06b812: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b814: mov.l @r8,r9
    cpu.r[9] = r32(mem, cpu.r[8]);
    // 8c06b816: add #24,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000018);
    // 8c06b818: add #80,r7
    cpu.r[7] = cpu.r[7].wrapping_add(0x00000050);
    // 8c06b81a: mov.l r9,@r2
    w32(mem, cpu.r[2], cpu.r[9]);
    // 8c06b81c: mova 0x8c06b8ec,r0
    cpu.r[0] = 0x8c06b8ec;
    // 8c06b81e: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b820: mov.l @(4,r8),r9
    cpu.r[9] = r32(mem, cpu.r[8].wrapping_add(4));
    // 8c06b822: add #28,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x0000001c);
    // 8c06b824: mov.l r9,@r2
    w32(mem, cpu.r[2], cpu.r[9]);
    // 8c06b826: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b828: add #24,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000018);
    // 8c06b82a: mov.l @r2,r9
    cpu.r[9] = r32(mem, cpu.r[2]);
    // 8c06b82c: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b82e: mov.l r9,@r7
    w32(mem, cpu.r[7], cpu.r[9]);
    // 8c06b830: add #28,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x0000001c);
    // 8c06b832: mov.l @r2,r9
    cpu.r[9] = r32(mem, cpu.r[2]);
    // 8c06b834: mov.l r9,@(4,r7)
    w32(mem, cpu.r[7].wrapping_add(4), cpu.r[9]);
    // 8c06b836: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b838: mov.l @(8,r8),r9
    cpu.r[9] = r32(mem, cpu.r[8].wrapping_add(8));
    // 8c06b83a: add #32,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000020);
    // 8c06b83c: mov.l r9,@r2
    w32(mem, cpu.r[2], cpu.r[9]);
    // 8c06b83e: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b840: mov.l @(12,r8),r9
    cpu.r[9] = r32(mem, cpu.r[8].wrapping_add(12));
    // 8c06b842: add #32,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000020);
    // 8c06b844: mov.l @r2,r8
    cpu.r[8] = r32(mem, cpu.r[2]);
    // 8c06b846: mov.l r8,@(8,r7)
    w32(mem, cpu.r[7].wrapping_add(8), cpu.r[8]);
    // 8c06b848: mov r15,r2
    cpu.r[2] = cpu.r[15];
    // 8c06b84a: mov.l r9,@(12,r7)
    w32(mem, cpu.r[7].wrapping_add(12), cpu.r[9]);
    // 8c06b84c: add #36,r2
    cpu.r[2] = cpu.r[2].wrapping_add(0x00000024);
    // 8c06b84e: fmov @r1,fr7
    cpu.fr[7] = rf(mem, cpu.r[1]);
    // 8c06b850: fmov @r0,fr6
    cpu.fr[6] = rf(mem, cpu.r[0]);
    // 8c06b852: fmov @r5,fr2
    cpu.fr[2] = rf(mem, cpu.r[5]);
    // 8c06b854: fmul fr6,fr7
    cpu.fr[7] = cpu.fr[7] * cpu.fr[6];
    // 8c06b856: fmov @r6,fr0
    cpu.fr[0] = rf(mem, cpu.r[6]);
    // 8c06b858: fmul fr6,fr2
    cpu.fr[2] = cpu.fr[2] * cpu.fr[6];
    // 8c06b85a: mov.l @r2,r5
    cpu.r[5] = r32(mem, cpu.r[2]);
    // 8c06b85c: fmul fr6,fr0
    cpu.fr[0] = cpu.fr[0] * cpu.fr[6];
    // 8c06b85e: fmov @r14,fr1
    cpu.fr[1] = rf(mem, cpu.r[14]);
    // 8c06b860: mov #56,r2
    cpu.r[2] = 0x00000038;
    // 8c06b862: mov.l @(4,r15),r1
    cpu.r[1] = r32(mem, cpu.r[15].wrapping_add(4));
    // 8c06b864: fsub fr0,fr1
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
    // 8c06b866: fmov @r10,fr0
    cpu.fr[0] = rf(mem, cpu.r[10]);
    // 8c06b868: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b86a: fmov @r1,fr8
    cpu.fr[8] = rf(mem, cpu.r[1]);
    // 8c06b86c: fadd fr0,fr1
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    // 8c06b86e: fmov @r5,fr3
    cpu.fr[3] = rf(mem, cpu.r[5]);
    // 8c06b870: fmul fr6,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[6];
    // 8c06b872: mov.l @r15,r1
    cpu.r[1] = r32(mem, cpu.r[15]);
    // 8c06b874: fsub fr2,fr3
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    // 8c06b876: fmov @r11,fr0
    cpu.fr[0] = rf(mem, cpu.r[11]);
    // 8c06b878: fsub fr7,fr8
    cpu.fr[8] = cpu.fr[8] - cpu.fr[7];
    // 8c06b87a: fmov @r3,fr4
    cpu.fr[4] = rf(mem, cpu.r[3]);
    // 8c06b87c: fadd fr0,fr3
    cpu.fr[3] = cpu.fr[3] + cpu.fr[0];
    // 8c06b87e: fmov @r13,fr2
    cpu.fr[2] = rf(mem, cpu.r[13]);
    // 8c06b880: fmul fr6,fr4
    cpu.fr[4] = cpu.fr[4] * cpu.fr[6];
    // 8c06b882: fmov @r1,fr5
    cpu.fr[5] = rf(mem, cpu.r[1]);
    // 8c06b884: fmul fr6,fr3
    cpu.fr[3] = cpu.fr[3] * cpu.fr[6];
    // 8c06b886: fmov @r12,fr0
    cpu.fr[0] = rf(mem, cpu.r[12]);
    // 8c06b888: fsub fr4,fr5
    cpu.fr[5] = cpu.fr[5] - cpu.fr[4];
    // 8c06b88a: fadd fr2,fr8
    cpu.fr[8] = cpu.fr[8] + cpu.fr[2];
    // 8c06b88c: fmov fr1,@r2
    wf(mem, cpu.r[2], cpu.fr[1]);
    // 8c06b88e: fadd fr0,fr5
    cpu.fr[5] = cpu.fr[5] + cpu.fr[0];
    // 8c06b890: mov #60,r2
    cpu.r[2] = 0x0000003c;
    // 8c06b892: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b894: fmov fr3,@r2
    wf(mem, cpu.r[2], cpu.fr[3]);
    // 8c06b896: mov #64,r2
    cpu.r[2] = 0x00000040;
    // 8c06b898: fmul fr6,fr5
    cpu.fr[5] = cpu.fr[5] * cpu.fr[6];
    // 8c06b89a: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b89c: fmov fr5,@r2
    wf(mem, cpu.r[2], cpu.fr[5]);
    // 8c06b89e: mov r15,r5
    cpu.r[5] = cpu.r[15];
    // 8c06b8a0: mov #68,r2
    cpu.r[2] = 0x00000044;
    // 8c06b8a2: fmul fr6,fr8
    cpu.fr[8] = cpu.fr[8] * cpu.fr[6];
    // 8c06b8a4: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c06b8a6: fmov fr8,@r2
    wf(mem, cpu.r[2], cpu.fr[8]);
    // 8c06b8a8: add #56,r5
    cpu.r[5] = cpu.r[5].wrapping_add(0x00000038);
    // 8c06b8aa: mov r4,r6
    cpu.r[6] = cpu.r[4];
    // 8c06b8ac: mov.l @r5,r3
    cpu.r[3] = r32(mem, cpu.r[5]);
    // 8c06b8ae: add #64,r6
    cpu.r[6] = cpu.r[6].wrapping_add(0x00000040);
    // 8c06b8b0: mov.l @(4,r5),r1
    cpu.r[1] = r32(mem, cpu.r[5].wrapping_add(4));
    // 8c06b8b2: mov r4,r0
    cpu.r[0] = cpu.r[4];
    // 8c06b8b4: mov.l r3,@r6
    w32(mem, cpu.r[6], cpu.r[3]);
    // 8c06b8b6: mov.l r1,@(4,r6)
    w32(mem, cpu.r[6].wrapping_add(4), cpu.r[1]);
    // 8c06b8b8: mov.l @(8,r5),r3
    cpu.r[3] = r32(mem, cpu.r[5].wrapping_add(8));
    // 8c06b8ba: mov.l @(12,r5),r1
    cpu.r[1] = r32(mem, cpu.r[5].wrapping_add(12));
    // 8c06b8bc: mov.l r3,@(8,r6)
    w32(mem, cpu.r[6].wrapping_add(8), cpu.r[3]);
    // 8c06b8be: mov.l r1,@(12,r6)
    w32(mem, cpu.r[6].wrapping_add(12), cpu.r[1]);
    // 8c06b8c0: add #72,r15
    cpu.r[15] = cpu.r[15].wrapping_add(0x00000048);
    // 8c06b8c2: mov.l @r15+,r8
    { let v = r32(mem, cpu.r[15]); cpu.r[8] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
    // 8c06b8c4: mov.l @r15+,r9
    { let v = r32(mem, cpu.r[15]); cpu.r[9] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
    // 8c06b8c6: mov.l @r15+,r10
    { let v = r32(mem, cpu.r[15]); cpu.r[10] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
    // 8c06b8c8: mov.l @r15+,r11
    { let v = r32(mem, cpu.r[15]); cpu.r[11] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
    // 8c06b8ca: mov.l @r15+,r12
    { let v = r32(mem, cpu.r[15]); cpu.r[12] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
    // 8c06b8cc: mov.l @r15+,r13
    { let v = r32(mem, cpu.r[15]); cpu.r[13] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
    // 8c06b8ce: rts  (leaf: delay slot already emitted)
    // 8c06b8d0: mov.l @r15+,r14
    { let v = r32(mem, cpu.r[15]); cpu.r[14] = v; cpu.r[15] = cpu.r[15].wrapping_add(4); }
}
