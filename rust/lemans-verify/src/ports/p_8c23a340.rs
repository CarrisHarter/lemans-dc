//! 0x8c23a340 — store fr4 into an object reached through a registry hit.
//!
//! The routine pushes `pr`, spills `r4` and `fr4`, then calls
//! `FUN_8c237780` (0x8c237780), which searches the address table rooted at
//! `[0x8c2377c4]` for the object address in `r4`.
//!
//! * miss (`r0 == 0`): return the literal at 0x8c23a388 (0xeeee); nothing is
//!   written.
//! * hit: reload `r4`, take the pointer at `[r4+0x88]`, and store the `fr4`
//!   bit pattern at `[pointer+12]`. Return 0.
//!
//! ```text
//! sts.l pr,@-r15 ; add #-8,r15
//! r3=r15+4 ; fmov fr4,@r3
//! r3=[0x8c23a384] (0x8c237780) ; jsr @r3 ; mov.l r4,@r15 (delay)
//! tst r0,r0 ; bf hit
//!   r0=[0x8c23a388] (0xeeee)
//!   bra/fall epilogue
//! hit:
//!   r3=r15 ; r0=(i16)[0x8c23a380] (0x88) ; r4=[r15]
//!   r3+=4 ; r2=[r3] (fr4 bits) ; r4=[r4+0x88]
//!   r0=0 ; [r4+12]=r2
//! epilogue: add #8,r15 ; lds.l @r15+,pr ; rts
//! ```
//!
//! `tst r0,r0` is the last flag-setting instruction on both arms: T is 1 on
//! the miss path and 0 on the hit path at the `rts`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c23a340: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c23a342: add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // 0x8c23a344: mov r15,r3 ; 0x8c23a346: add #4,r3
    cpu.r[3] = cpu.r[15].wrapping_add(4);
    // 0x8c23a348: fmov fr4,@r3
    let _ = mem.write_f32(cpu.r[3], cpu.fr[4]);
    // 0x8c23a34a: mov.l 0x8c23a384,r3
    cpu.r[3] = mem.read_u32(0x8c23a384).unwrap_or(0x8c237780);
    // 0x8c23a34c: jsr @r3 ; 0x8c23a34e: mov.l r4,@r15 (delay slot)
    cpu.pr = 0x8c23a350;
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    crate::registry::p_8c237780::run(cpu, mem);
    // 0x8c23a350: tst r0,r0 ; 0x8c23a352: bf 0x8c23a35e
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        // 0x8c23a354: mov.l 0x8c23a388,r0
        cpu.r[0] = mem.read_u32(0x8c23a388).unwrap_or(0x0000_eeee);
    } else {
        // 0x8c23a35e: mov r15,r3
        cpu.r[3] = cpu.r[15];
        // 0x8c23a360: mov.w 0x8c23a380,r0 (sign-extended)
        cpu.r[0] = mem.read_u16(0x8c23a380).unwrap_or(0x0088) as i16 as i32 as u32;
        // 0x8c23a362: mov.l @r15,r4
        cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        // 0x8c23a364: add #4,r3
        cpu.r[3] = cpu.r[3].wrapping_add(4);
        // 0x8c23a366: mov.l @r3,r2
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c23a368: mov.l @(r0,r4),r4
        cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
        // 0x8c23a36a: mov #0,r0
        cpu.r[0] = 0;
        // 0x8c23a36c: mov.l r2,@(12,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[2]);
    }
    // 0x8c23a36e: add #8,r15 ; 0x8c23a370: lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23a372: rts ; 0x8c23a374: nop (delay slot)
}
