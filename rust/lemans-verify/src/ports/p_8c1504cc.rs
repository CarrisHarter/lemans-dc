//! 0x8c1504cc — build a 68-byte transform struct at r4 from two 6-float
//! vectors at r5/r6, using the float literal pool at 0x8c150698 (2.0),
//! 0x8c15069c (-3.0) and 0x8c1506a0 (3.0).
//!
//! Straight-line code (no branches): saves fr12-fr15/r8-r14, reserves 40
//! bytes of scratch on the stack, computes cross-product-style terms with
//! fmul/fadd/fsub (all single precision), stores results to r4+4..r4+64,
//! restores the saved registers and returns. This port is a mechanical,
//! instruction-by-instruction translation of the disassembly (each comment
//! carries the original address); the r15-relative scratch slots are real
//! memory ops exactly as the original does.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: fmov fr15..fr12,@-r15 ; mov.l r14..r8,@-r15 ; add #-40,r15
    for f in [15usize, 14, 13, 12] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_f32(cpu.r[15], cpu.fr[f]);
    }
    for n in [14usize, 13, 12, 11, 10, 9, 8] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let v = cpu.r[n];
        let _ = mem.write_u32(cpu.r[15], v);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(40);

    cpu.r[0] = mem.read_u32(0x8c150690).unwrap_or(0); // 8c1504e4: mov.l 0x8c150690,r0
    cpu.r[3] = cpu.r[5]; // 8c1504e6
    cpu.r[8] = cpu.r[6]; // 8c1504e8
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // 8c1504ea: mov.l r0,@r4
    cpu.r[1] = cpu.r[6]; // 8c1504ec
    cpu.r[0] = mem.read_u32(0x8c150694).unwrap_or(0); // 8c1504ee
    cpu.r[9] = cpu.r[6]; // 8c1504f0
    cpu.r[10] = cpu.r[5]; // 8c1504f2
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // 8c1504f4
    cpu.r[3] = cpu.r[3].wrapping_add(12); // 8c1504f6
    cpu.r[0] = 0x8c150698; // 8c1504f8: mova
    cpu.fr[5] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 8c1504fa: fmov @r0,fr5
    cpu.r[8] = cpu.r[8].wrapping_add(12); // 8c1504fc
    cpu.fr[14] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // 8c1504fe
    cpu.r[0] = cpu.r[6]; // 8c150500
    cpu.fr[13] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // 8c150502
    cpu.r[7] = cpu.r[5]; // 8c150504
    cpu.fr[15] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // 8c150506
    cpu.r[3] = cpu.r[5]; // 8c150508
    cpu.r[9] = cpu.r[9].wrapping_add(4); // 8c15050a
    cpu.r[10] = cpu.r[10].wrapping_add(16); // 8c15050c
    cpu.r[7] = cpu.r[7].wrapping_add(4); // 8c15050e
    cpu.fr[9] = mem.read_f32(cpu.r[7]).unwrap_or(0.0); // 8c150510
    cpu.r[6] = cpu.r[6].wrapping_add(8); // 8c150512
    cpu.r[3] = cpu.r[3].wrapping_add(20); // 8c150514
    cpu.r[5] = cpu.r[5].wrapping_add(8); // 8c150516
    cpu.fr[6] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // 8c150518
    cpu.r[2] = cpu.r[15]; // 8c15051a
    cpu.fr[11] = mem.read_f32(cpu.r[9]).unwrap_or(0.0); // 8c15051c
    cpu.r[1] = cpu.r[1].wrapping_add(16); // 8c15051e
    cpu.fr[10] = mem.read_f32(cpu.r[10]).unwrap_or(0.0); // 8c150520
    cpu.fr[0] = cpu.fr[15]; // 8c150522
    cpu.fr[8] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // 8c150524
    cpu.fr[4] = cpu.fr[13]; // 8c150526
    cpu.fr[7] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // 8c150528
    cpu.fr[0] = cpu.fr[0] * cpu.fr[5]; // 8c15052a: fmul fr5,fr0
    cpu.fr[2] = mem.read_f32(cpu.r[8]).unwrap_or(0.0); // 8c15052c
    cpu.r[9] = cpu.r[4]; // 8c15052e
    cpu.r[2] = cpu.r[2].wrapping_add(4); // 8c150530
    let _ = mem.write_f32(cpu.r[15], cpu.fr[2]); // 8c150532: fmov fr2,@r15
    cpu.r[0] = cpu.r[0].wrapping_add(20); // 8c150534
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // 8c150536
    cpu.fr[0] = cpu.fr[0] + cpu.fr[14]; // 8c150538: fadd fr14,fr0
    cpu.fr[4] = cpu.fr[4] * cpu.fr[5]; // 8c15053a: fmul fr5,fr4
    let _ = mem.write_f32(cpu.r[2], cpu.fr[2]); // 8c15053c: fmov fr2,@r2
    cpu.fr[0] = cpu.fr[0] - cpu.fr[4]; // 8c15053e: fsub fr4,fr0
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 8c150540
    cpu.r[2] = cpu.r[15]; // 8c150542
    cpu.r[2] = cpu.r[2].wrapping_add(8); // 8c150544
    let _ = mem.write_f32(cpu.r[2], cpu.fr[2]); // 8c150546
    cpu.r[9] = cpu.r[9].wrapping_add(4); // 8c150548
    cpu.fr[2] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // 8c15054a: fmov @r15,fr2
    cpu.r[2] = cpu.r[15]; // 8c15054c
    cpu.r[0] = 0x8c15069c; // 8c15054e: mova
    cpu.fr[0] = cpu.fr[0] + cpu.fr[2]; // 8c150550: fadd fr2,fr0
    let _ = mem.write_f32(cpu.r[9], cpu.fr[0]); // 8c150552: fmov fr0,@r9  -> [r4+4]
    cpu.r[2] = cpu.r[2].wrapping_add(36); // 8c150554
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 8c150556
    cpu.fr[4] = cpu.fr[15]; // 8c150558
    cpu.r[10] = cpu.r[4]; // 8c15055a
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // 8c15055c
    cpu.r[0] = 0x8c1506a0; // 8c15055e: mova
    cpu.r[2] = cpu.r[15]; // 8c150560
    cpu.fr[12] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 8c150562
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 8c150564
    cpu.r[11] = cpu.r[4]; // 8c150566
    let _ = mem.write_f32(cpu.r[2], cpu.fr[12]); // 8c150568
    cpu.r[10] = cpu.r[10].wrapping_add(20); // 8c15056a
    cpu.r[2] = cpu.r[15]; // 8c15056c
    cpu.r[2] = cpu.r[2].wrapping_add(36); // 8c15056e
    cpu.fr[12] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c150570
    cpu.fr[3] = cpu.fr[9]; // 8c150572
    cpu.r[12] = cpu.r[4]; // 8c150574
    cpu.fr[4] = cpu.fr[4] * cpu.fr[12]; // 8c150576: fmul fr12,fr4
    cpu.fr[12] = cpu.fr[14]; // 8c150578
    cpu.r[2] = cpu.r[15]; // 8c15057a
    cpu.fr[12] = cpu.fr[12] * cpu.fr[5]; // 8c15057c: fmul fr5,fr12
    cpu.r[2] = cpu.r[2].wrapping_add(12); // 8c15057e
    let _ = mem.write_f32(cpu.r[2], cpu.fr[12]); // 8c150580
    cpu.fr[3] = cpu.fr[3] * cpu.fr[5]; // 8c150582: fmul fr5,fr3
    cpu.r[2] = cpu.r[15]; // 8c150584
    cpu.fr[4] = cpu.fr[4] - cpu.fr[12]; // 8c150586: fsub fr12,fr4
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 8c150588
    cpu.fr[12] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c15058a
    cpu.r[11] = cpu.r[11].wrapping_add(36); // 8c15058c
    cpu.r[12] = cpu.r[12].wrapping_add(52); // 8c15058e
    cpu.fr[13] = cpu.fr[13] * cpu.fr[12]; // 8c150590: fmul fr12,fr13
    cpu.r[2] = cpu.r[15]; // 8c150592
    cpu.fr[12] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // 8c150594
    cpu.fr[4] = cpu.fr[4] + cpu.fr[13]; // 8c150596: fadd fr13,fr4
    cpu.r[2] = cpu.r[2].wrapping_add(20); // 8c150598
    cpu.fr[4] = cpu.fr[4] - cpu.fr[12]; // 8c15059a: fsub fr12,fr4
    let _ = mem.write_f32(cpu.r[10], cpu.fr[4]); // 8c15059c: fmov fr4,@r10 -> [r4+20]
    cpu.fr[3] = cpu.fr[3] + cpu.fr[10]; // 8c15059e: fadd fr10,fr3
    cpu.fr[4] = cpu.fr[11]; // 8c1505a0
    cpu.fr[4] = cpu.fr[4] * cpu.fr[5]; // 8c1505a2: fmul fr5,fr4
    let _ = mem.write_f32(cpu.r[11], cpu.fr[14]); // 8c1505a4: fmov fr14,@r11 -> [r4+36]
    cpu.r[13] = cpu.r[4]; // 8c1505a6
    let _ = mem.write_f32(cpu.r[2], cpu.fr[4]); // 8c1505a8
    cpu.fr[3] = cpu.fr[3] - cpu.fr[4]; // 8c1505aa: fsub fr4,fr3
    cpu.r[2] = cpu.r[15]; // 8c1505ac
    let _ = mem.write_f32(cpu.r[12], cpu.fr[15]); // 8c1505ae: fmov fr15,@r12 -> [r4+52]
    cpu.r[2] = cpu.r[2].wrapping_add(36); // 8c1505b0
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c1505b2
    cpu.fr[2] = cpu.fr[9]; // 8c1505b4
    cpu.r[14] = cpu.r[4]; // 8c1505b6
    cpu.fr[2] = cpu.fr[2] * cpu.fr[4]; // 8c1505b8: fmul fr4,fr2
    cpu.fr[4] = cpu.fr[10]; // 8c1505ba
    cpu.r[2] = cpu.r[15]; // 8c1505bc
    cpu.r[7] = cpu.r[4]; // 8c1505be
    cpu.fr[1] = cpu.fr[6]; // 8c1505c0
    cpu.r[6] = cpu.r[4]; // 8c1505c2
    cpu.fr[1] = cpu.fr[1] * cpu.fr[5]; // 8c1505c4: fmul fr5,fr1
    cpu.fr[4] = cpu.fr[4] * cpu.fr[5]; // 8c1505c6: fmul fr5,fr4
    cpu.r[2] = cpu.r[2].wrapping_add(24); // 8c1505c8
    let _ = mem.write_f32(cpu.r[2], cpu.fr[4]); // 8c1505ca
    cpu.r[13] = cpu.r[13].wrapping_add(8); // 8c1505cc
    cpu.r[2] = cpu.r[15]; // 8c1505ce
    cpu.r[2] = cpu.r[2].wrapping_add(4); // 8c1505d0
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c1505d2
    cpu.r[14] = cpu.r[14].wrapping_add(24); // 8c1505d4
    cpu.r[7] = cpu.r[7].wrapping_add(40); // 8c1505d6
    cpu.r[2] = cpu.r[15]; // 8c1505d8
    cpu.fr[3] = cpu.fr[3] + cpu.fr[4]; // 8c1505da: fadd fr4,fr3
    cpu.r[2] = cpu.r[2].wrapping_add(24); // 8c1505dc
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c1505de
    cpu.r[6] = cpu.r[6].wrapping_add(56); // 8c1505e0
    cpu.fr[1] = cpu.fr[1] + cpu.fr[7]; // 8c1505e2: fadd fr7,fr1
    cpu.r[2] = cpu.r[15]; // 8c1505e4
    cpu.r[5] = cpu.r[4]; // 8c1505e6
    cpu.fr[2] = cpu.fr[2] - cpu.fr[4]; // 8c1505e8: fsub fr4,fr2
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 8c1505ea
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c1505ec
    cpu.fr[0] = cpu.fr[6]; // 8c1505ee
    cpu.r[3] = cpu.r[4]; // 8c1505f0
    cpu.r[2] = cpu.r[15]; // 8c1505f2
    cpu.fr[11] = cpu.fr[11] * cpu.fr[4]; // 8c1505f4: fmul fr4,fr11
    let _ = mem.write_f32(cpu.r[13], cpu.fr[3]); // 8c1505f6: fmov fr3,@r13 -> [r4+8]
    cpu.r[2] = cpu.r[2].wrapping_add(4); // 8c1505f8
    cpu.fr[3] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c1505fa
    cpu.fr[2] = cpu.fr[2] + cpu.fr[11]; // 8c1505fc: fadd fr11,fr2
    cpu.r[5] = cpu.r[5].wrapping_add(12); // 8c1505fe
    cpu.fr[2] = cpu.fr[2] - cpu.fr[3]; // 8c150600: fsub fr3,fr2
    let _ = mem.write_f32(cpu.r[14], cpu.fr[2]); // 8c150602: fmov fr2,@r14 -> [r4+24]
    cpu.r[2] = cpu.r[15]; // 8c150604
    cpu.fr[2] = cpu.fr[8]; // 8c150606
    cpu.fr[2] = cpu.fr[2] * cpu.fr[5]; // 8c150608: fmul fr5,fr2
    let _ = mem.write_f32(cpu.r[7], cpu.fr[10]); // 8c15060a: fmov fr10,@r7 -> [r4+40]
    cpu.r[2] = cpu.r[2].wrapping_add(28); // 8c15060c
    let _ = mem.write_f32(cpu.r[2], cpu.fr[2]); // 8c15060e
    cpu.fr[1] = cpu.fr[1] - cpu.fr[2]; // 8c150610: fsub fr2,fr1
    cpu.r[2] = cpu.r[15]; // 8c150612
    let _ = mem.write_f32(cpu.r[6], cpu.fr[9]); // 8c150614: fmov fr9,@r6 -> [r4+56]
    cpu.r[2] = cpu.r[2].wrapping_add(36); // 8c150616
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c150618
    cpu.r[3] = cpu.r[3].wrapping_add(28); // 8c15061a
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // 8c15061c: fmul fr2,fr0
    cpu.fr[2] = cpu.fr[7]; // 8c15061e
    cpu.r[2] = cpu.r[15]; // 8c150620
    cpu.fr[2] = cpu.fr[2] * cpu.fr[5]; // 8c150622: fmul fr5,fr2
    cpu.r[2] = cpu.r[2].wrapping_add(32); // 8c150624
    let _ = mem.write_f32(cpu.r[2], cpu.fr[2]); // 8c150626
    cpu.r[2] = cpu.r[15]; // 8c150628
    cpu.r[2] = cpu.r[2].wrapping_add(8); // 8c15062a
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c15062c
    cpu.r[2] = cpu.r[15]; // 8c15062e
    cpu.fr[1] = cpu.fr[1] + cpu.fr[2]; // 8c150630: fadd fr2,fr1
    cpu.r[2] = cpu.r[2].wrapping_add(32); // 8c150632
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c150634
    cpu.r[2] = cpu.r[15]; // 8c150636
    cpu.fr[0] = cpu.fr[0] - cpu.fr[2]; // 8c150638: fsub fr2,fr0
    cpu.fr[2] = cpu.fr[4]; // 8c15063a
    cpu.fr[8] = cpu.fr[8] * cpu.fr[2]; // 8c15063c: fmul fr2,fr8
    let _ = mem.write_f32(cpu.r[5], cpu.fr[1]); // 8c15063e: fmov fr1,@r5 -> [r4+12]
    cpu.r[2] = cpu.r[2].wrapping_add(8); // 8c150640
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 8c150642
    cpu.fr[0] = cpu.fr[0] + cpu.fr[8]; // 8c150644: fadd fr8,fr0
    cpu.fr[0] = cpu.fr[0] - cpu.fr[1]; // 8c150646: fsub fr1,fr0
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // 8c150648: fmov fr0,@r3 -> [r4+28]
    cpu.r[1] = cpu.r[4]; // 8c15064a
    cpu.r[1] = cpu.r[1].wrapping_add(44); // 8c15064c
    cpu.r[0] = cpu.r[4]; // 8c15064e
    let _ = mem.write_f32(cpu.r[1], cpu.fr[7]); // 8c150650: fmov fr7,@r1 -> [r4+44]
    cpu.r[0] = cpu.r[0].wrapping_add(60); // 8c150652
    cpu.r[5] = cpu.r[4]; // 8c150654
    let _ = mem.write_f32(cpu.r[0], cpu.fr[6]); // 8c150656: fmov fr6,@r0 -> [r4+60]
    cpu.r[3] = cpu.r[4]; // 8c150658
    cpu.r[1] = cpu.r[4]; // 8c15065a
    cpu.r[5] = cpu.r[5].wrapping_add(16); // 8c15065c
    cpu.fr[0] = 0.0; // 8c15065e: fldi0 fr0
    cpu.r[0] = cpu.r[4]; // 8c150660
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]); // 8c150662: fmov fr0,@r5 -> [r4+16] = 0.0
    cpu.r[3] = cpu.r[3].wrapping_add(32); // 8c150664
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // 8c150666: fmov fr0,@r3 -> [r4+32] = 0.0
    cpu.r[1] = cpu.r[1].wrapping_add(48); // 8c150668
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // 8c15066a: fmov fr0,@r1 -> [r4+48] = 0.0
    cpu.r[0] = cpu.r[0].wrapping_add(64); // 8c15066c
    cpu.fr[0] = 1.0; // 8c15066e: fldi1 fr0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // 8c150670: fmov fr0,@r0 -> [r4+64] = 1.0
    cpu.r[0] = cpu.r[4]; // 8c150672

    // epilogue: add #40,r15 ; pops ; rts (delay slot pops fr15)
    cpu.r[15] = cpu.r[15].wrapping_add(40); // 8c150674
    for n in [8usize, 9, 10, 11, 12, 13, 14] {
        cpu.r[n] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,rN
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    for f in [12usize, 13, 14, 15] {
        cpu.fr[f] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15+,frN
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
