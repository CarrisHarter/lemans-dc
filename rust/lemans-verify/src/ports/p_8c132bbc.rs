//! 0x8c132bbc — stamp vtable words, call 0x8c1261d4(r4+28), snapshot 12
//! bytes from the 0x8c207df0 table into the frame, zero +16/byte +20, copy the
//! frame snapshot to [r4+4..+12], self-link [r4+24]=r4. Straight line.
//!
//! ```text
//! sts.l pr,@-r15 ; add #-16,r15 ; mov.l r4,@r15
//! r3=[r15] ; r0=[0x8c132c18] (0x8c1eb6a0) ; r1=[0x8c132c1c] (0x8c1eee74)
//! [r3]=r0 ; r0=[0x8c132c20] (0x8c1261d4) ; [r3]=r1
//! r4=[r15] ; jsr @r0 ; add #28,r4 (delay) -> pr=0x8c132bd4
//! r1=[r15] ; r6=r15 ; r0=[0x8c132c24] (0x8c1eee5c) ; r6+=4
//! r5=[0x8c132c28] (0x8c207df0) ; r3=0 ; [r1]=r0
//! r4=[r5] ; r0=20 ; r1=[r5+4] ; [r6]=r4 ; [r6+4]=r1
//! r1=[r5+8] ; [r6+8]=r1
//! r1=[r15] ; [r1+16]=r3 ; r1=[r15] ; [r1+r0]=r3 (byte, r0=20)
//! r3=[r15] ; r1=[r6] ; r3+=4 ; r0=[r6+4] ; [r3]=r1 ; [r3+4]=r0
//! r0=[r6+8] ; [r3+8]=r0
//! r0=[r15] ; [r0+24]=r0
//! r0=[r15] ; r15+=16 ; lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! Frame traffic (saves at [r15..], snapshot at [r6..]=[r15+4..]) lives in the
//! synthetic stack window, so it is excluded from the write diff — but every
//! reload is transcribed so later values stay exact. The jsr callee is a
//! verified port, so it is invoked directly with pr set to the instruction
//! after the delay slot (0x8c132bd4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c132bbc: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c132bbe: add #-16,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    // 0x8c132bc0: mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    // 0x8c132bc2: mov.l @r15,r3
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132bc4: mov.l 0x8c132c18,r0
    cpu.r[0] = mem.read_u32(0x8C132C18).unwrap_or(0); // 0x8c1eb6a0
    // 0x8c132bc6: mov.l 0x8c132c1c,r1
    cpu.r[1] = mem.read_u32(0x8C132C1C).unwrap_or(0); // 0x8c1eee74
    // 0x8c132bc8: mov.l r0,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    // 0x8c132bca: mov.l 0x8c132c20,r0
    cpu.r[0] = mem.read_u32(0x8C132C20).unwrap_or(0); // 0x8c1261d4
    // 0x8c132bcc: mov.l r1,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    // 0x8c132bce: mov.l @r15,r4
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132bd0: jsr @r0 ; 0x8c132bd2: add #28,r4 (delay)
    cpu.pr = 0x8C132BD4;
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    crate::registry::p_8c1261d4::run(cpu, mem);
    // 0x8c132bd4: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132bd6: mov r15,r6
    cpu.r[6] = cpu.r[15];
    // 0x8c132bd8: mov.l 0x8c132c24,r0
    cpu.r[0] = mem.read_u32(0x8C132C24).unwrap_or(0); // 0x8c1eee5c
    // 0x8c132bda: add #4,r6
    cpu.r[6] = cpu.r[6].wrapping_add(4);
    // 0x8c132bdc: mov.l 0x8c132c28,r5
    cpu.r[5] = mem.read_u32(0x8C132C28).unwrap_or(0); // 0x8c207df0
    // 0x8c132bde: mov #0,r3
    cpu.r[3] = 0;
    // 0x8c132be0: mov.l r0,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    // 0x8c132be2: mov.l @r5,r4
    cpu.r[4] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    // 0x8c132be4: mov #20,r0
    cpu.r[0] = 20;
    // 0x8c132be6: mov.l @(4,r5),r1
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    // 0x8c132be8: mov.l r4,@r6
    let _ = mem.write_u32(cpu.r[6], cpu.r[4]);
    // 0x8c132bea: mov.l r1,@(4,r6)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[1]);
    // 0x8c132bec: mov.l @(8,r5),r1
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    // 0x8c132bee: mov.l r1,@(8,r6)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(8), cpu.r[1]);
    // 0x8c132bf0: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132bf2: mov.l r3,@(16,r1)
    let _ = mem.write_u32(cpu.r[1].wrapping_add(16), cpu.r[3]);
    // 0x8c132bf4: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132bf6: mov.b r3,@(r0,r1)  (r0=20)
    let _ = mem.write_u8(cpu.r[1].wrapping_add(cpu.r[0]), cpu.r[3] as u8);
    // 0x8c132bf8: mov.l @r15,r3
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132bfa: mov.l @r6,r1
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    // 0x8c132bfc: add #4,r3
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    // 0x8c132bfe: mov.l @(4,r6),r0
    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
    // 0x8c132c00: mov.l r1,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    // 0x8c132c02: mov.l r0,@(4,r3)
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
    // 0x8c132c04: mov.l @(8,r6),r0
    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
    // 0x8c132c06: mov.l r0,@(8,r3)
    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[0]);
    // 0x8c132c08: mov.l @r15,r0
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132c0a: mov.l r0,@(24,r0)
    let base = cpu.r[0];
    let _ = mem.write_u32(base.wrapping_add(24), base);
    // 0x8c132c0c: mov.l @r15,r0
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c132c0e: add #16,r15
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    // 0x8c132c10: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c132c12: rts ; 0x8c132c14: nop (delay slot)
}
