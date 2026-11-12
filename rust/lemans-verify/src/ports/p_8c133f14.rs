//! 0x8c133f14 — construct object: stamp vtables, init tail via 0x8c1261d4,
//! copy table words, self-link.
//!
//! ```text
//! sts.l pr,@-r15            ; save pr
//! add #-16,r15              ; frame
//! mov.l r4,@r15             ; frame.obj = param
//! *(obj) = 0x8c1eb6a0       ; pool @0x8c133f70
//! *(obj) = 0x8c1eeec0       ; pool @0x8c133f74
//! jsr 0x8c1261d4            ; pool @0x8c133f78; delay: r4 = obj+28
//! *(obj) = 0x8c1eeea8       ; pool @0x8c133f7c
//! r5 = 0x8c207f08           ; table literal @0x8c133f80
//! frame[1..3] = table[0..2] ; stack snapshot (excluded from diff)
//! *(obj+16) = 0
//! *(u8)(obj+20) = 0
//! *(obj+4)  = table[0]      ; via frame snapshot
//! *(obj+8)  = table[1]
//! *(obj+12) = table[2]
//! *(obj+24) = obj           ; self-link
//! rts                       ; delay: nop; r0 = obj
//! ```
//! Hidden effects: the mid-function `jsr` runs the real callee, which stores
//! obj+28 to the global at 0x8c2ab5e0, writes r5/1/table-sourced words at
//! obj+28/32/36, and leaves r0 = obj+28, r1 = *0x8c206c28 (both clobbered
//! again below). Final state: r0 = obj, r1 = r4 = table[0], r3 = obj+4,
//! r5 = table address, r6 = frame+4, r15/pr restored, sr untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let saved_pr = cpu.pr; // sts.l pr,@-r15 (stack store excluded from diff)
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    let obj = cpu.r[4]; // mov.l r4,@r15 / mov.l @r15,r3
    let _ = mem.write_u32(obj, 0x8C1E_B6A0); // pool @0x8c133f70
    let _ = mem.write_u32(obj, 0x8C1E_EEC0); // pool @0x8c133f74
    // jsr 0x8c1261d4 (pool @0x8c133f78); delay slot runs before the jump.
    cpu.r[4] = obj.wrapping_add(28); // add #28,r4 (delay)
    cpu.pr = 0x8C13_3F2C; // jsr return = post-delay insn
    crate::registry::p_8c1261d4::run(cpu, mem);
    cpu.r[6] = cpu.r[15].wrapping_add(4); // mov r15,r6; add #4,r6
    cpu.r[5] = 0x8C20_7F08; // table literal @0x8c133f80
    cpu.r[3] = 0; // mov #0,r3
    let _ = mem.write_u32(obj, 0x8C1E_EEA8); // pool @0x8c133f7c
    // Frame snapshot of table[0..2] (stack stores, excluded); reloads below
    // read the snapshot, which still equals the table words.
    let t0 = mem.read_u32(cpu.r[5]).unwrap_or(0);
    let t1 = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    let t2 = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.r[4] = t0; // mov.l @r5,r4
    cpu.r[0] = 20; // mov #20,r0
    let _ = mem.write_u32(obj.wrapping_add(16), 0); // mov.l r3,@(16,r1)
    let _ = mem.write_u8(obj.wrapping_add(20), 0); // mov.b r3,@(r0,r1)
    cpu.r[3] = obj.wrapping_add(4); // mov.l @r15,r3; add #4,r3
    cpu.r[1] = t0; // mov.l @r6,r1
    cpu.r[0] = t1; // mov.l @(4,r6),r0
    let _ = mem.write_u32(cpu.r[3], t0); // mov.l r1,@r3  -> obj+4
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), t1); // -> obj+8
    cpu.r[0] = t2; // mov.l @(8,r6),r0
    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), t2); // -> obj+12
    cpu.r[0] = obj; // mov.l @r15,r0
    let _ = mem.write_u32(obj.wrapping_add(24), obj); // self-link
    cpu.r[15] = cpu.r[15].wrapping_add(16); // add #16,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4); // lds.l @r15+,pr
    cpu.pr = saved_pr;
    // rts; delay slot is nop.
}
