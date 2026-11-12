//! 0x8c1365cc — stamp vtable words, call 0x8c1261d4(r4+28), snapshot 12
//! bytes from the 0x8c208078 table into the frame, zero +16/byte +20, copy the
//! frame snapshot to [r4+4..+12], self-link [r4+24]=r4, then a mask tail:
//! r3 = (1 << [[0x8c2ab468]+60]) - 1 stored at [r4+40]. Straight line.
//!
//! ```text
//! sts.l pr,@-r15 ; add #-16,r15 ; mov.l r4,@r15
//! r3=[r15] ; r0=[0x8c136638] (0x8c1eb6a0) ; r1=[0x8c13663c] (0x8c1eef20)
//! [r3]=r0 ; r0=[0x8c136640] (0x8c1261d4) ; [r3]=r1
//! r4=[r15] ; jsr @r0 ; add #28,r4 (delay) -> pr=0x8c1365e4
//! r1=[r15] ; r6=0 ; r0=[0x8c136644] (0x8c1eef08) ; r7=[0x8c136648] (0x8c208078)
//! [r1]=r0 ; r1=r15 ; r3=[r7] ; r1+=4 ; r5=[r7+4] ; r0=20
//! r4=[0x8c13664c] (0x8c2ab468) ; [r1]=r3 ; [r1+4]=r5
//! r3=1 ; r5=[r7+8] ; [r1+8]=r5
//! r5=[r15] ; [r5+16]=r6 ; r5=[r15] ; [r5+r0]=r6 (byte, r0=20)
//! r6=[r15] ; r5=[r1] ; r6+=4 ; r0=[r1+4] ; [r6]=r5 ; [r6+4]=r0
//! r0=[r1+8] ; [r6+8]=r0
//! r0=[r15] ; [r0+24]=r0
//! r0=[r4] ; r1=[r15] ; r0=[r0+60] ; shld r0,r3 ; r3-=1 ; [r1+40]=r3
//! r0=[r15] ; r15+=16 ; lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! Frame traffic (saves at [r15..], snapshot at [r1..]=[r15+4..]) lives in the
//! synthetic stack window, so it is excluded from the write diff — but every
//! reload is transcribed so later values stay exact. The jsr callee is a
//! verified port, so it is invoked directly with pr set to the instruction
//! after the delay slot (0x8c1365e4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1365cc: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1365ce: add #-16,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    // 0x8c1365d0: mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    // 0x8c1365d2: mov.l @r15,r3
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c1365d4: mov.l 0x8c136638,r0
    cpu.r[0] = mem.read_u32(0x8C136638).unwrap_or(0); // 0x8c1eb6a0
    // 0x8c1365d6: mov.l 0x8c13663c,r1
    cpu.r[1] = mem.read_u32(0x8C13663C).unwrap_or(0); // 0x8c1eef20
    // 0x8c1365d8: mov.l r0,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    // 0x8c1365da: mov.l 0x8c136640,r0
    cpu.r[0] = mem.read_u32(0x8C136640).unwrap_or(0); // 0x8c1261d4
    // 0x8c1365dc: mov.l r1,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    // 0x8c1365de: mov.l @r15,r4
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c1365e0: jsr @r0 ; 0x8c1365e2: add #28,r4 (delay)
    cpu.pr = 0x8C1365E4;
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    crate::registry::p_8c1261d4::run(cpu, mem);
    // 0x8c1365e4: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c1365e6: mov #0,r6
    cpu.r[6] = 0;
    // 0x8c1365e8: mov.l 0x8c136644,r0
    cpu.r[0] = mem.read_u32(0x8C136644).unwrap_or(0); // 0x8c1eef08
    // 0x8c1365ea: mov.l 0x8c136648,r7
    cpu.r[7] = mem.read_u32(0x8C136648).unwrap_or(0); // 0x8c208078
    // 0x8c1365ec: mov.l r0,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    // 0x8c1365ee: mov r15,r1
    cpu.r[1] = cpu.r[15];
    // 0x8c1365f0: mov.l @r7,r3
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    // 0x8c1365f2: add #4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    // 0x8c1365f4: mov.l @(4,r7),r5
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
    // 0x8c1365f6: mov #20,r0
    cpu.r[0] = 20;
    // 0x8c1365f8: mov.l 0x8c13664c,r4
    cpu.r[4] = mem.read_u32(0x8C13664C).unwrap_or(0); // 0x8c2ab468
    // 0x8c1365fa: mov.l r3,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[3]);
    // 0x8c1365fc: mov.l r5,@(4,r1)
    let _ = mem.write_u32(cpu.r[1].wrapping_add(4), cpu.r[5]);
    // 0x8c1365fe: mov #1,r3
    cpu.r[3] = 1;
    // 0x8c136600: mov.l @(8,r7),r5
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0);
    // 0x8c136602: mov.l r5,@(8,r1)
    let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[5]);
    // 0x8c136604: mov.l @r15,r5
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c136606: mov.l r6,@(16,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(16), cpu.r[6]);
    // 0x8c136608: mov.l @r15,r5
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c13660a: mov.b r6,@(r0,r5)  (r0=20)
    let _ = mem.write_u8(cpu.r[5].wrapping_add(cpu.r[0]), cpu.r[6] as u8);
    // 0x8c13660c: mov.l @r15,r6
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c13660e: mov.l @r1,r5
    cpu.r[5] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    // 0x8c136610: add #4,r6
    cpu.r[6] = cpu.r[6].wrapping_add(4);
    // 0x8c136612: mov.l @(4,r1),r0
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
    // 0x8c136614: mov.l r5,@r6
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);
    // 0x8c136616: mov.l r0,@(4,r6)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[0]);
    // 0x8c136618: mov.l @(8,r1),r0
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
    // 0x8c13661a: mov.l r0,@(8,r6)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(8), cpu.r[0]);
    // 0x8c13661c: mov.l @r15,r0
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c13661e: mov.l r0,@(24,r0)
    let base = cpu.r[0];
    let _ = mem.write_u32(base.wrapping_add(24), base);
    // 0x8c136620: mov.l @r4,r0
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    // 0x8c136622: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c136624: mov.l @(60,r0),r0
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(60)).unwrap_or(0);
    // 0x8c136626: shld r0,r3
    let amount = cpu.r[0] as i32;
    if amount >= 0 {
        cpu.r[3] <<= (amount as u32) & 31;
    } else {
        cpu.r[3] >>= ((-amount) as u32) & 31;
    }
    // 0x8c136628: add #-1,r3
    cpu.r[3] = cpu.r[3].wrapping_sub(1);
    // 0x8c13662a: mov.l r3,@(40,r1)
    let _ = mem.write_u32(cpu.r[1].wrapping_add(40), cpu.r[3]);
    // 0x8c13662c: mov.l @r15,r0
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c13662e: add #16,r15
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    // 0x8c136630: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c136632: rts ; 0x8c136634: nop (delay slot)
}
